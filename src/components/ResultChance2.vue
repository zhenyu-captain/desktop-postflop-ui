<template>
  <div class="flex w-full h-full">
    <!-- 卡牌选择区 -->
    <div
      class="flex flex-col h-full items-center pt-[0.75%] gap-[0.75%] bg-white rounded-lg shadow-sm border border-gray-200 m-1"
      style="flex: 4"
    >
      <!-- 标题和玩家信息 -->
      <div class="w-full px-4 py-1.5 border-b border-gray-100 flex items-center justify-between">
        <div class="flex items-center">
          <div class="h-3.5 w-3.5 rounded-full bg-red-500 mr-2"></div>
          <h3 class="font-medium text-gray-900">
            {{ selectedChance.player === 'turn' ? 'Turn Card Selection' : 'River Card Selection' }}
          </h3>
        </div>
        <div class="text-sm text-gray-500">
          Previous: <span class="text-indigo-600 font-medium">{{ selectedChance.prevPlayer.toUpperCase() }}</span>
        </div>
      </div>
      
      <!-- 卡牌网格 -->
      <div class="w-full px-3 py-1.5">
        <div
          v-for="suit in 4"
          :key="suit"
          class="flex shrink-0 w-full justify-center gap-[1%] mb-1.5 transition-all duration-300 hover:translate-y-0.5"
        >
          <!-- 花色标识 -->
          <div class="w-[3.25rem] flex items-center justify-end pr-2">
            <span class="text-xl" :class="getSuitClass(suit-1)">{{ getSuitSymbol(suit-1) }}</span>
          </div>
          
          <!-- 卡牌行 -->
          <BoardSelectorCard
            v-for="rank in 13"
            :key="rank"
            class="transition-all duration-200"
            font-size="max(1.2vw, 13px)"
            width="5.5%"
            :card-id="56 - 4 * rank - suit"
            :disabled="selectedChance.cards[56 - 4 * rank - suit].isDead"
            :is-selected="selectedChance.selectedIndex === 56 - 4 * rank - suit"
            @click="deal(56 - 4 * rank - suit)"
          />
          <div></div>
        </div>
      </div>

      <!-- 图表区域 -->
      <div
        ref="chartParentDiv"
        class="relative flex-grow max-h-[50%] my-1.5"
        style="width: calc(84.5% + 3.25rem)"
      >
        <div
          v-if="chartData"
          class="absolute left-0 top-0 w-full h-full"
        >
          <Bar :data="chartData" :options="chartOptions" />
        </div>
        <div v-else class="w-full h-full flex flex-col items-center justify-center">
          <div class="text-gray-400 text-center">
            <p>No data available</p>
            <p class="text-xs mt-1">{{ debugInfo }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- 结果表格 -->
    <ResultTable
      style="flex: 3"
      table-mode="chance"
      :chance-type="selectedChance.player"
      :selected-spot="selectedSpot"
      :chance-reports="chanceReports"
      :display-player="displayPlayer"
      class="mx-1 my-1"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { ranks, suits, cardId, toFixed1, toFixedAdaptive } from "../utils";
import {
  ChanceReports,
  Spot,
  SpotChance,
  SpotPlayer,
  DisplayOptions,
} from "../result-types";

import {
  Chart,
  ChartData,
  ChartOptions,
  BarController,
  BarElement,
  CategoryScale,
  LinearScale,
  Title,
  Tooltip,
} from "chart.js";

import BoardSelectorCard from "./BoardSelectorCard.vue";
import ResultTable from "./ResultTable.vue";
import { Bar } from "vue-chartjs";

Chart.register(
  BarController,
  BarElement,
  CategoryScale,
  LinearScale,
  Title,
  Tooltip
);

Chart.defaults.font.size = 14;
Chart.defaults.font.family =
  "system-ui, 'Noto Sans', 'Open Sans', Roboto, sans-serif";

const labels = [...ranks].reverse();

const green600 = "#16a34a";
const blue600 = "#2563eb";
const pink600 = "#db2777";
const black = "#000000";
const suitColor = [green600, blue600, pink600, black];

const props = defineProps<{
  selectedSpot: Spot;
  selectedChance: SpotChance;
  chanceReports: ChanceReports | null;
  displayOptions: DisplayOptions;
  displayPlayer: "oop" | "ip";
}>();

const emit = defineEmits<{
  (event: "deal-card", card: number): void;
}>();

const chartParentDiv = ref<HTMLDivElement | null>(null);
const chartParentDivHeight = ref(0);

const assignChartParentDivHeight = () => {
  if (chartParentDiv.value) {
    chartParentDivHeight.value = chartParentDiv.value.clientHeight;
  }
};

watch(chartParentDiv, assignChartParentDivHeight);
window.addEventListener("resize", assignChartParentDivHeight);

// 获取花色颜色
const getSuitClass = (suitIndex: number) => {
  if (suitIndex === 1 || suitIndex === 2) {
    return 'text-pink-600';
  }
  return 'text-gray-900';
};

// 获取花色符号
const getSuitSymbol = (suitIndex: number) => {
  const symbols = ['♠', '♥', '♦', '♣'];
  return symbols[suitIndex] || '';
};

// 处理卡牌选择
const deal = (card: number) => {
  emit("deal-card", card);
};

const chartData = computed((): ChartData<"bar", number[]> | null => {
  const reports = props.chanceReports;
  if (!reports) return null;

  const options = props.displayOptions;
  const playerIndex = props.displayPlayer === "oop" ? 0 : 1;

  let datasets: ChartData<"bar", number[]>["datasets"];
  const stacks = ["clubs", "diamonds", "hearts", "spades"];
  const defaultData = { barPercentage: 0.85, categoryPercentage: 0.75 };

  if (
    options.chartChance === "strategy-combos" ||
    options.chartChance === "strategy"
  ) {
    const isCombos = options.chartChance === "strategy-combos";
    if (reports.currentPlayer === props.displayPlayer) {
      const spot = props.selectedSpot as SpotPlayer;
      datasets = Array.from({ length: reports.numActions * 4 }, (_, i) => {
        const actionIndex = i >> 2;
        const suit = i & 3;
        const action = spot.actions[actionIndex];
        let label = action.name;
        if (action.amount !== "0") label += ` ${action.amount}`;
        return {
          data: Array.from({ length: 13 }, (_, rank) => {
            const card = cardId(rank, suit);
            if (reports.status[card] === 0) return 0;
            const coef = isCombos ? reports.combos[playerIndex][card] : 1;
            return coef * reports.strategy[actionIndex * 52 + card];
          }).reverse(),
          label,
          backgroundColor: action.color,
          stack: stacks[suit],
          ...defaultData,
        };
      }).reverse();
    } else {
      datasets = Array.from({ length: 4 }, (_, suit) => ({
        data: Array.from({ length: 13 }, (_, rank) => {
          const card = cardId(rank, suit);
          if (reports.status[card] === 0) return 0;
          return isCombos ? reports.combos[playerIndex][card] : 1;
        }).reverse(),
        backgroundColor: suitColor[suit],
        stack: stacks[suit],
        ...defaultData,
      })).reverse();
    }
  } else {
    const data =
      options.chartChance === "eq"
        ? reports.equity[playerIndex]
        : options.chartChance === "ev"
        ? reports.ev[playerIndex]
        : reports.eqr[playerIndex];
    datasets = Array.from({ length: 4 }, (_, suit) => ({
      data: Array.from(
        { length: 13 },
        (_, rank) => data[4 * rank + suit]
      ).reverse(),
      backgroundColor: suitColor[suit],
      stack: stacks[suit],
      ...defaultData,
    })).reverse();
  }

  return { labels, datasets };
});

const chartOptions = computed((): ChartOptions<"bar"> => {
  const option = props.displayOptions.chartChance;
  const style = ["strategy", "eq", "eqr"].includes(option)
    ? "percent"
    : "decimal";
  const format = { style, useGrouping: false, minimumFractionDigits: 0 };

  const titleText =
    props.displayPlayer.toUpperCase() +
    " " +
    {
      "strategy-combos": "Strategy (Combos)",
      strategy: "Strategy",
      eq: "Equity",
      ev: "EV",
      eqr: "EQR",
    }[option];

  return {
    responsive: true,
    maintainAspectRatio: false,
    animation: {
      duration: 300
    },
    normalized: true,
    scales: {
      y: {
        stacked: true,
        min: ["ev", "eqr"].includes(option) ? undefined : 0,
        max: option === "strategy" ? 1 : undefined,
        ticks: { format },
        afterFit(axis) {
          axis.width = 52;
        },
        grid: {
          color: 'rgba(0, 0, 0, 0.05)'
        }
      },
      x: {
        grid: {
          color: 'rgba(0, 0, 0, 0.05)'
        }
      }
    },
    plugins: {
      title: {
        display: true,
        text: titleText,
        font: { size: 16, weight: "normal" },
        color: "rgba(0, 0, 0, 0.9)",
        padding: {
          top: 10,
          bottom: 16
        }
      },
      legend: {
        display: false,
      },
      tooltip: {
        titleMarginBottom: 4,
        backgroundColor: 'rgba(255, 255, 255, 0.9)',
        titleColor: 'rgba(0, 0, 0, 0.8)',
        bodyColor: 'rgba(0, 0, 0, 0.7)',
        borderColor: 'rgba(0, 0, 0, 0.1)',
        borderWidth: 1,
        padding: 10,
        cornerRadius: 6,
        boxShadow: '0 2px 5px rgba(0, 0, 0, 0.15)',
        callbacks: {
          title(context) {
            const rank = 12 - context[0].dataIndex;
            const suit = 3 - (context[0].datasetIndex & 3);
            return ranks[rank] + suits[suit];
          },
          label(context) {
            const value = context.parsed.y;
            let label = context.dataset.label ?? "";
            if (label) label += ": ";
            if (["strategy-combos", "ev"].includes(option)) {
              label += toFixed1(value);
            } else if (["strategy", "eq", "eqr"].includes(option)) {
              label += Math.round(value * 100) + "%";
            } else {
              label += toFixedAdaptive(value);
            }
            return label;
          },
        },
      },
    },
  };
});

// 添加调试信息
const debugInfo = computed(() => {
  return `Reports: ${props.chanceReports ? 'Yes' : 'No'}, Height: ${chartParentDivHeight.value}, Player: ${props.displayPlayer}, Chart Type: ${props.displayOptions.chartChance}`;
});
</script>
