<template>
  <div class="flex flex-col w-full border-l border-gray-200 overflow-x-auto">
    <!-- 只保留标题，移除控件区域 -->
    <div class="flex shrink-0 h-12 border-b border-gray-200 bg-gray-50">
      <div class="flex h-full px-4 items-center">
        <span class="relative pb-0.5 text-lg font-semibold text-gray-800">
          Summary
          <span class="absolute bottom-0 left-0 w-full h-0.5 bg-blue-500 rounded-full"></span>
        </span>
      </div>
      
      <div class="flex h-full ml-auto pr-4 items-center">
        <!-- 只保留导出按钮 -->
        <div class="flex items-center px-3">
          <Tippy content="Export summary to CSV file">
            <button
              :class="
                'flex w-8 h-8 items-center justify-center border border-gray-300 bg-white ' +
                'rounded-lg shadow-sm transition-all duration-200 hover:bg-gray-100 active:bg-gray-200'
              "
              @click="exportSummary"
            >
              <ArrowTopRightOnSquareIcon class="w-5 h-5 text-gray-600" />
            </button>
          </Tippy>
        </div>
      </div>
    </div>

    <div
      ref="tableDiv"
      class="flex-grow overflow-y-scroll will-change-scroll"
      @scroll.passive="onTableScroll"
    >
      <table class="w-full h-full text-sm text-center align-middle">
        <thead class="sticky top-0 z-30 bg-blue-100 shadow-md">
          <tr style="height: calc(1.9rem + 1px)" class="bg-blue-500 text-white">
            <th
              v-for="column in columns"
              :key="column.label"
              scope="col"
              :class="
                'whitespace-nowrap select-none ' +
                (column.type === 'card'
                  ? 'sticky left-0 z-40 bg-blue-500 '
                  : '') +
                (column.type !== 'bar' ? 'cursor-pointer hover:bg-blue-600 ' : '') +
                (tableMode === 'chance' ? 'header-divider' : '')
              "
              :style="{
                'min-width':
                  (column.type === 'card'
                    ? '4'
                    : column.type === 'bar'
                    ? '6'
                    : '3.5') + 'rem',
              }"
              @click="column.type !== 'bar' && sortBy(columnIndex(column))"
            >
              <span
                v-if="sortKey.key === columnIndex(column)"
                class="inline-block text-xs pr-1 text-white"
              >
                {{ sortKey.order === "asc" ? "▲" : "▼" }}
              </span>
              <span class="font-medium">{{ column.label }}</span>
            </th>
          </tr>

          <tr v-if="tableMode !== 'chance'" style="height: calc(1.9rem + 1px)" class="bg-blue-300">
            <th
              v-for="column in columns"
              :key="column.label"
              scope="col"
              :class="
                'header-divider ' +
                (column.type === 'card'
                  ? 'sticky left-0 z-40 underline bg-blue-300 text-gray-800 '
                  : 'relative ') +
                (column.type === 'bar'
                  ? 'pt-[0.3125rem] pb-1 px-1'
                  : 'cursor-pointer hover:bg-blue-400')
              "
              :style="{
                height: column.type === 'bar' ? 'calc(1.9rem + 1px)' : 'auto',
              }"
              @click="column.type !== 'bar' && sortBy(columnIndex(column))"
            >
              <template v-if="column.type === 'card'">
                <span class="font-medium">{{ hoverContent?.name ?? "Total" }}</span>
              </template>

              <template v-else-if="column.type === 'bar'">
                <div
                  v-if="summary && column.label === 'Strategy'"
                  class="w-full h-full bg-gray-200 bg-left bg-no-repeat rounded-md"
                  :style="{
                    'background-image': strategyBarBgImage(summary),
                  }"
                ></div>
              </template>

              <template
                v-else-if="
                  summary == null || isNaN(summary[columnIndex(column)])
                "
              >
                <span>-</span>
              </template>

              <template v-else>
                <div class="inline-block w-12 text-right">
                  <span
                    v-if="column.type === 'weight'"
                    :data-set="
                      (strTmp = toFixedAdaptive(summary[columnIndex(column)]))
                    "
                    class="text-gray-800"
                  >
                    <span>{{ strTmp.slice(0, strTmp.indexOf(".") + 1) }}</span>
                    <span class="text-xs">{{
                      strTmp.slice(strTmp.indexOf(".") + 1)
                    }}</span>
                  </span>
                  <span
                    v-else-if="
                      column.type === 'percentage' || column.type === 'action'
                    "
                    :data-set="
                      (strTmp = toFixed1(summary[columnIndex(column)] * 100))
                    "
                    class="text-gray-800"
                  >
                    <span>{{ strTmp.slice(0, -1) }}</span>
                    <span class="text-xs">{{ strTmp.slice(-1) }}%</span>
                  </span>
                  <span
                    v-else-if="column.type === 'action-ev'"
                    :data-set="
                      (strTmp = toFixed1(
                        summary[columnIndex(column) - 1] * 100
                      ))
                    "
                    class="text-gray-800"
                  >
                    <span>{{ strTmp.slice(0, -1) }}</span>
                    <span class="text-xs">{{ strTmp.slice(-1) }}%</span>
                  </span>
                  <span
                    v-else-if="column.type === 'ev'"
                    :data-set="
                      (strTmp = toFixed[evDigits - 1](
                        summary[columnIndex(column)]
                      ))
                    "
                    :class="summary[columnIndex(column)] >= 0 ? 'text-emerald-600' : 'text-rose-600'"
                  >
                    <span>{{ strTmp.slice(0, -evDigits) }}</span>
                    <span class="text-xs">{{ strTmp.slice(-evDigits) }}</span>
                  </span>
                </div>
              </template>

              <div
                v-if="
                  summary &&
                  (column.type === 'action' || column.type === 'action-ev')
                "
                class="absolute w-12 h-1 bottom-0 inset-x-0 mx-auto rounded-sm"
                :style="{
                  background: actionBarBg(column.index, summary),
                }"
              ></div>
            </th>
          </tr>
        </thead>

        <tbody>
          <!-- Top empty row -->
          <tr
            v-if="emptyBufferTop > 0"
            :style="{
              '--num-rows': emptyBufferTop,
              height: 'calc(var(--num-rows) * (1.9rem + 1px))',
            }"
          >
            <td :colspan="columns.length"></td>
          </tr>

          <!-- Body -->
          <tr
            v-for="(item, index) in resultsRendered"
            :key="item[0]"
            :class="
              'relative ' +
              (item[0] === scrollTarget ? 'bg-yellow-100' : (index % 2 === 0 ? 'bg-white' : 'bg-gray-50')) +
              ' hover:bg-blue-50 transform transition-all duration-200 hover:scale-[1.01]'
            "
            style="height: calc(1.9rem + 1px)"
          >
            <td
              v-for="column in columns"
              :key="column.label"
              :class="
                'row-divider ' +
                (column.type === 'card'
                  ? 'sticky left-0 z-10 ' +
                    (item[0] === scrollTarget
                      ? 'bg-yellow-100 '
                      : (index % 2 === 0 ? 'bg-white ' : 'bg-gray-50 ')) +
                    'font-medium'
                  : 'relative ') +
                (column.type === 'bar' ? 'pt-[0.3125rem] pb-1 px-1' : 'pt-0.5')
              "
              :style="{
                height: column.type === 'bar' ? 'calc(1.9rem + 1px)' : 'auto',
              }"
            >
              <template v-if="column.type === 'card'">
                <span
                  v-for="card in pairText(item[columnIndex(column)])"
                  :key="card.rank + card.suit"
                  :class="card.colorClass"
                >
                  {{ card.rank + card.suit }}
                </span>
              </template>

              <template v-else-if="column.type === 'bar'">
                <div
                  class="w-full h-full bg-gray-200 bg-left bg-no-repeat rounded-md"
                  :style="{
                    'background-image': strategyBarBgImage(item),
                    'background-size': strategyBarBgSize(item),
                  }"
                ></div>
              </template>

              <template v-else-if="isNaN(item[columnIndex(column)])">
                <span>-</span>
              </template>

              <template v-else>
                <div class="inline-block w-12 text-right">
                  <span
                    v-if="tableMode === 'chance' && column.type === 'weight'"
                    :data-set="
                      (strTmp = toFixedAdaptive(item[columnIndex(column)]))
                    "
                    class="text-gray-800"
                  >
                    <span>{{ strTmp.slice(0, strTmp.indexOf(".") + 1) }}</span>
                    <span class="text-xs">{{
                      strTmp.slice(strTmp.indexOf(".") + 1)
                    }}</span>
                  </span>
                  <span
                    v-else-if="
                      column.type === 'weight' ||
                      column.type === 'percentage' ||
                      column.type === 'action'
                    "
                    :data-set="
                      (strTmp = toFixed1(item[columnIndex(column)] * 100))
                    "
                    class="text-gray-800"
                  >
                    <span>{{ strTmp.slice(0, -1) }}</span>
                    <span class="text-xs">{{ strTmp.slice(-1) }}%</span>
                  </span>
                  <span
                    v-else-if="
                      column.type === 'ev' || column.type === 'action-ev'
                    "
                    :data-set="
                      (strTmp = toFixed[evDigits - 1](
                        item[columnIndex(column)]
                      ))
                    "
                    :class="item[columnIndex(column)] >= 0 ? 'text-emerald-600' : 'text-rose-600'"
                  >
                    <span>{{ strTmp.slice(0, -evDigits) }}</span>
                    <span class="text-xs">{{ strTmp.slice(-evDigits) }}</span>
                  </span>
                </div>
              </template>

              <div
                v-if="column.type === 'action' || column.type === 'action-ev'"
                class="absolute w-12 h-1 bottom-0 inset-x-0 mx-auto rounded-sm"
                :style="{
                  background: actionBarBg(column.index, item),
                }"
              ></div>
            </td>
          </tr>

          <!-- No results -->
          <tr v-if="resultsRendered.length === 0">
            <td
              class="relative bg-white row-divider"
              style="height: calc(1.9rem + 1px)"
              :colspan="columns.length"
            >
              <span class="text-gray-500">
                {{
                  tableMode === "chance"
                    ? `${capitalize(chanceType)} reports not available`
                    : "No results"
                }}
              </span>
            </td>
          </tr>

          <!-- Bottom empty row -->
          <tr
            v-if="emptyBufferBottom > 0"
            :style="{
              '--num-rows': emptyBufferBottom,
              height: 'calc(var(--num-rows) * (1.9rem + 1px))',
            }"
          >
            <td :colspan="columns.length" class="relative row-divider"></td>
          </tr>

          <!-- Spacer -->
          <tr>
            <td :colspan="columns.length" class="relative row-divider"></td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, reactive, ref, toRefs, watch } from "vue";

import {
  ranks,
  suitLetters,
  cardText,
  cardPairOrder,
  toFixed1,
  toFixed,
  toFixedAdaptive,
  capitalize,
} from "../utils";

import {
  Results,
  ChanceReports,
  Spot,
  SpotPlayer,
  contentGraphsList,
  HoverContent,
  TableMode,
} from "../result-types";

import { save } from "@tauri-apps/api/dialog";
import { writeTextFile } from "@tauri-apps/api/fs";

import { Tippy } from "vue-tippy";
import { ArrowTopRightOnSquareIcon } from "@heroicons/vue/24/solid";

const barWidthListBasics = ["normalized", "absolute", "full"] as const;
const contentListBasics = ["percentage", "ev"] as const;
type DisplayOptionsBasics = {
  barWidth: (typeof barWidthListBasics)[number];
  content: (typeof contentListBasics)[number];
};

const barWidthListChance = ["normalized", "full"] as const;
type DisplayOptionsChance = {
  barWidth: (typeof barWidthListChance)[number];
};

type ColumnCard = {
  label: string;
  type: "card";
};

type ColumnBar = {
  label: string;
  type: "bar";
};

type ColumnWeight = {
  label: string;
  type: "weight";
};

type ColumnPercentage = {
  label: string;
  type: "percentage";
  index: number;
};

type ColumnEv = {
  label: string;
  type: "ev";
};

type ColumnAction = {
  label: string;
  type: "action";
  index: number;
};

type ColumnActionEV = {
  label: string;
  type: "action-ev";
  index: number;
};

type Column =
  | ColumnCard
  | ColumnBar
  | ColumnWeight
  | ColumnPercentage
  | ColumnEv
  | ColumnAction
  | ColumnActionEV;

const INDEX_CARD_PAIR = 0;
const INDEX_WEIGHT = 1;
const INDEX_NORMALIZER = 2;
const INDEX_EQUITY = 3;
const INDEX_EV = 4;
const INDEX_EQR = 5;
const INDEX_STRATEGY_BASE = 6;

const columnIndex = (column: Column) => {
  switch (column.type) {
    case "card":
      return INDEX_CARD_PAIR;
    case "bar":
      return -1;
    case "weight":
      return INDEX_WEIGHT;
    case "percentage":
      return column.index;
    case "ev":
      return INDEX_EV;
    case "action":
      return INDEX_STRATEGY_BASE + column.index * 2;
    case "action-ev":
      return INDEX_STRATEGY_BASE + column.index * 2 + 1;
  }
};

const yellow500 = "#eab308";
const blue500 = "#3b82f6";  // 蓝色 (blue-500)
const green500 = "#10b981"; // 绿色 (emerald-500)
const neutral200 = "#e5e7eb"; // 浅灰色背景
const neutral800 = "#1f2937"; // 深灰色前景

const cardStr = (card: number) => {
  const rank = ranks[card >>> 2];
  const suit = suitLetters[card & 3];
  return rank + suit;
};

interface Props {
  tableMode: TableMode;
  graphType?: (typeof contentGraphsList)[number];
  chanceType?: "turn" | "river";
  cards?: number[][] | null;
  selectedSpot: Spot;
  results?: Results | null;
  chanceReports?: ChanceReports | null;
  displayPlayer: "oop" | "ip";
  hoverContent?: HoverContent | null;
  scrollTarget?: number | null;
  tableBarWidth?: "normalized" | "absolute" | "full";
  tableDisplayContent?: "percentage" | "ev";
}

const props = withDefaults(defineProps<Props>(), {
  graphType: "eq",
  chanceType: "turn",
  cards: null,
  results: null,
  chanceReports: null,
  hoverContent: null,
  scrollTarget: null,
  tableBarWidth: "normalized",
  tableDisplayContent: "percentage"
});

const displayOptions =
  props.tableMode !== "chance"
    ? reactive<DisplayOptionsBasics>({
        barWidth: props.tableBarWidth as (typeof barWidthListBasics)[number],
        content: props.tableDisplayContent as (typeof contentListBasics)[number],
      })
    : reactive<DisplayOptionsChance>({
        barWidth: props.tableBarWidth as (typeof barWidthListChance)[number],
      });

const optionsStorageKey = `display-options-table-${props.tableMode}`;
const savedDisplayOptions = localStorage.getItem(optionsStorageKey);

if (savedDisplayOptions) {
  if (props.tableMode !== "chance") {
    const saved = JSON.parse(savedDisplayOptions) as DisplayOptionsBasics;
    const options = displayOptions as DisplayOptionsBasics;
    if (barWidthListBasics.includes(saved.barWidth)) {
      options.barWidth = saved.barWidth;
    }
    if (contentListBasics.includes(saved.content)) {
      options.content = saved.content;
    }
  } else {
    const saved = JSON.parse(savedDisplayOptions) as DisplayOptionsChance;
    const options = displayOptions as DisplayOptionsChance;
    if (barWidthListChance.includes(saved.barWidth)) {
      options.barWidth = saved.barWidth;
    }
  }
}

const updateDisplayOptions = () => {
  localStorage.setItem(optionsStorageKey, JSON.stringify(displayOptions));
};

const tableDiv = ref<HTMLDivElement | null>(null);
const tableHeight = ref(0);

const assignTableHeight = () => {
  if (tableDiv.value) {
    tableHeight.value = tableDiv.value.clientHeight;
  }
};

watch(tableDiv, assignTableHeight);
window.addEventListener("resize", assignTableHeight);

const pairText = (pair: number) => {
  const card1 = pair & 0xff;
  const card2 = pair >>> 8;
  if (card2 !== 0xff) {
    return [cardText(card2), cardText(card1)];
  } else {
    return [cardText(card1)];
  }
};

const defaultSortKey = computed(() => {
  if (props.tableMode !== "graphs") return INDEX_CARD_PAIR;
  if (props.graphType === "eq") return INDEX_EQUITY;
  if (props.graphType === "ev") return INDEX_EV;
  return INDEX_EQR;
});

type Key = { key: number; order: "asc" | "desc" };
const sortKey = ref<Key>({ key: defaultSortKey.value, order: "desc" });

const sortStorageKey = `sort-key-table-${props.tableMode}`;
const savedSortKey = sessionStorage.getItem(sortStorageKey);

if (savedSortKey) {
  const saved = JSON.parse(savedSortKey) as Key;
  if (saved.key < INDEX_STRATEGY_BASE) {
    sortKey.value = saved;
  }
}

const sortBy = (key: number) => {
  const order =
    key === sortKey.value.key && sortKey.value.order === "desc"
      ? "asc"
      : "desc";
  sortKey.value = { key, order };
  sessionStorage.setItem(sortStorageKey, JSON.stringify(sortKey.value));
};

const resetSortKey = () => {
  if (sortKey.value.key >= INDEX_STRATEGY_BASE) {
    sortBy(defaultSortKey.value);
  }
};

const { selectedSpot, displayPlayer } = toRefs(props);
watch([selectedSpot, displayPlayer], resetSortKey);

if (props.tableMode !== "chance") {
  watch(() => (displayOptions as DisplayOptionsBasics).content, resetSortKey);
}

if (props.tableMode === "graphs") {
  watch(defaultSortKey, (newValue, oldValue) => {
    if (sortKey.value.key === oldValue) {
      const order = sortKey.value.order;
      sortKey.value = { key: newValue, order };
      sessionStorage.setItem(sortStorageKey, JSON.stringify(sortKey.value));
    }
  });
}

const maxEv = computed(() => {
  const playerIndex = props.displayPlayer === "oop" ? 0 : 1;
  if (props.tableMode !== "chance") {
    const results = props.results;
    if (!results || results.isEmpty) return 0;
    return Math.max(...results.ev[playerIndex].map((v) => Math.abs(v)));
  } else {
    const reports = props.chanceReports;
    if (!reports || reports.status.every((s) => s <= 1)) return 0;
    return Math.max(...reports.ev[playerIndex].map((v) => Math.abs(v)));
  }
});

const evDigits = computed(() => {
  return maxEv.value < 9.9995 ? 3 : maxEv.value < 99.995 ? 2 : 1;
});

const numActions = computed(() => {
  let data: Results | ChanceReports | null;
  if (props.tableMode !== "chance") {
    data = props.results;
  } else {
    data = props.chanceReports;
  }

  const spot = props.selectedSpot;
  return (
    (spot.type === "player" &&
      spot.player === props.displayPlayer &&
      data &&
      data.numActions) ||
    0
  );
});

const columns = computed(() => {
  const ret: Column[] = [];

  if (props.tableMode !== "chance") {
    ret.push({ label: "Hand", type: "card" });
    ret.push({
      label: numActions.value > 0 ? "Strategy" : "Weight (Bar)",
      type: "bar",
    });

    ret.push({ label: "Weight", type: "weight" });
    ret.push({ label: "EQ", type: "percentage", index: INDEX_EQUITY });
    ret.push({ label: "EV", type: "ev" });
    ret.push({ label: "EQR", type: "percentage", index: INDEX_EQR });

    const options = displayOptions as DisplayOptionsBasics;

    if (numActions.value > 0) {
      const spot = props.selectedSpot as SpotPlayer;
      for (let i = 0; i < numActions.value; ++i) {
        const j = numActions.value - i - 1; // reverse order
        const action = spot.actions[j];
        const label =
          action.amount === "0"
            ? action.name
            : `${action.name[0]} ${action.amount}`;
        if (options.content === "percentage") {
          ret.push({ label, type: "action", index: i });
        } else {
          ret.push({ label, type: "action-ev", index: i });
        }
      }
    }
  } else {
    ret.push({ label: capitalize(props.chanceType), type: "card" });
    ret.push({
      label: numActions.value > 0 ? "Strategy" : "Combos (Bar)",
      type: "bar",
    });

    ret.push({ label: "Combos", type: "weight" });
    ret.push({ label: "EQ", type: "percentage", index: INDEX_EQUITY });
    ret.push({ label: "EV", type: "ev" });
    ret.push({ label: "EQR", type: "percentage", index: INDEX_EQR });

    if (numActions.value > 0) {
      const spot = props.selectedSpot as SpotPlayer;
      for (let i = 0; i < numActions.value; ++i) {
        const j = numActions.value - i - 1;
        const action = spot.actions[j];
        const label =
          action.amount === "0"
            ? action.name
            : `${action.name[0]} ${action.amount}`;
        ret.push({ label, type: "action", index: i });
      }
    }
  }

  return ret;
});

const resultsFiltered = computed(() => {
  const ret: number[][] = [];

  const playerIndex = props.displayPlayer === "oop" ? 0 : 1;

  let cards: number[];
  if (props.tableMode !== "chance" && props.cards) {
    cards = props.cards[playerIndex];
  } else {
    cards = Array.from({ length: 52 }, (_, i) => 0xff00 + i);
  }

  let dataRow: (index: number) => number[] | null;

  if (props.tableMode !== "chance") {
    dataRow = (index: number) => {
      const results = props.results;
      if (!results) return null;

      const weight = results.weights[playerIndex][index];
      const normalizer = results.normalizer[playerIndex][index];
      if (weight === 0 || normalizer === 0) return null;

      const ret: number[] = [];

      ret.push(cards[index]);
      ret.push(results.weights[playerIndex][index]);
      ret.push(results.normalizer[playerIndex][index]);
      ret.push(results.equity[playerIndex][index] ?? Number.NaN);
      ret.push(results.ev[playerIndex][index] ?? Number.NaN);

      const eqr = results.eqr[playerIndex][index] ?? Number.NaN;
      ret.push(isFinite(eqr) ? eqr : Number.NaN);

      for (let i = numActions.value - 1; i >= 0; --i) {
        const j = i * cards.length + index;
        ret.push(results.strategy[j]);
        ret.push(results.actionEv[j] ?? 0);
      }

      return ret;
    };
  } else {
    dataRow = (index: number) => {
      const reports = props.chanceReports;
      if (!reports) return null;

      const status = reports.status[index];
      if (status === 0) return null;

      const ret: number[] = [];

      ret.push(cards[index]);
      ret.push(reports.combos[playerIndex][index]);
      ret.push(Number.NaN);

      if (status === 1) {
        ret.push(Number.NaN, Number.NaN, Number.NaN);
      } else {
        ret.push(reports.equity[playerIndex][index]);
        ret.push(reports.ev[playerIndex][index]);
        ret.push(reports.eqr[playerIndex][index]);
      }

      for (let i = numActions.value - 1; i >= 0; --i) {
        const j = i * cards.length + index;
        ret.push(reports.strategy[j]);
        ret.push(Number.NaN);
      }

      return ret;
    };
  }

  if (props.hoverContent) {
    for (const index of props.hoverContent.indices) {
      const data = dataRow(index);
      if (data) ret.push(data);
    }
  } else {
    for (let i = 0; i < cards.length; ++i) {
      const data = dataRow(i);
      if (data) ret.push(data);
    }
  }

  return ret;
});

const resultsSorted = computed(() => {
  const { key, order } = sortKey.value;
  const ret = [...resultsFiltered.value];

  if (key === 0) {
    ret.sort((a, b) => {
      return cardPairOrder(a[0]) - cardPairOrder(b[0]);
    });
  } else {
    ret.sort((a, b) => {
      if (!isNaN(a[key]) && !isNaN(b[key])) {
        return a[key] - b[key] || cardPairOrder(a[0]) - cardPairOrder(b[0]);
      } else if (!isNaN(a[key])) {
        return -1;
      } else if (!isNaN(b[key])) {
        return 1;
      } else {
        return cardPairOrder(a[0]) - cardPairOrder(b[0]);
      }
    });
  }

  if (order === "desc") ret.reverse();
  return ret;
});

const rem = Number(
  getComputedStyle(document.documentElement).fontSize.slice(0, -2)
);
const rowHeight = 1.9 * rem + 1;

const bufferUnit = 10;
const numDisplayedRows = computed(() =>
  Math.ceil(tableHeight.value / rowHeight)
);

const emptyBufferTop = ref(-2 * bufferUnit);
const emptyBufferBottom = computed(
  () =>
    resultsFiltered.value.length -
    (emptyBufferTop.value + numDisplayedRows.value + 4 * bufferUnit)
);

watch(resultsSorted, () => {
  if (!tableDiv.value) return;
  tableDiv.value.scrollTop = 0;
  emptyBufferTop.value = -2 * bufferUnit;
});

let ticking = false;

const onTableScroll = async () => {
  if (ticking) return;
  ticking = true;

  requestAnimationFrame(() => {
    ticking = false;
    if (!tableDiv.value) return;

    const { scrollTop } = tableDiv.value;
    const topIndex = Math.max(scrollTop / rowHeight, 0);
    if (topIndex < emptyBufferTop.value + bufferUnit) {
      emptyBufferTop.value =
        (Math.floor(topIndex / bufferUnit) - 1) * bufferUnit;
    } else if (topIndex > emptyBufferTop.value + 3 * bufferUnit) {
      emptyBufferTop.value =
        (Math.floor(topIndex / bufferUnit) - 2) * bufferUnit;
    }
  });
};

const resultsRendered = computed(() => {
  return resultsSorted.value.slice(
    Math.max(emptyBufferTop.value, 0),
    emptyBufferTop.value + numDisplayedRows.value + 4 * bufferUnit
  );
});

const { scrollTarget } = toRefs(props);
watch(scrollTarget, () => {
  if (!tableDiv.value || !scrollTarget.value) return;
  const scrollIndex = resultsSorted.value.findIndex(
    (row) => row[0] === scrollTarget.value
  );
  if (scrollIndex === -1) return;
  const scrollTop =
    scrollIndex * rowHeight - (tableHeight.value - 3 * rowHeight) / 2;
  tableDiv.value.scrollTop = Math.max(scrollTop, 0);
});

const summary = computed(() => {
  const results = resultsFiltered.value;
  if (!props.results || results.length === 0) return null;

  let normalizer = 0;
  const ret = results[0].map(() => 0);

  for (const row of results) {
    const n = row[INDEX_NORMALIZER];
    normalizer += n;
    ret[INDEX_WEIGHT] += row[INDEX_WEIGHT];
    for (let i = INDEX_EQUITY; i < row.length; ++i) {
      ret[i] += row[i] * n;
    }
  }

  for (let i = INDEX_EQUITY; i < ret.length; ++i) {
    ret[i] /= normalizer;
  }

  // EQR is not the average
  const playerIndex = props.displayPlayer === "oop" ? 0 : 1;
  const eqrBase = props.results.eqrBase[playerIndex];
  ret[INDEX_EQR] = ret[INDEX_EV] / (eqrBase * ret[INDEX_EQUITY]);
  if (!isFinite(ret[INDEX_EQR])) ret[INDEX_EQR] = Number.NaN;

  return ret;
});

const actionColors = computed(() => {
  if (numActions.value === 0) return [];
  const spot = props.selectedSpot as SpotPlayer;
  return spot.actions.map((a) => a.color).reverse();
});

const maxWeight = computed(() => {
  return Math.max(...resultsFiltered.value.map((r) => r[INDEX_WEIGHT]));
});

const strategyBarBgImage = (row: number[]) => {
  if (!row || row.length === INDEX_STRATEGY_BASE) {
    return `linear-gradient(to right, ${blue500} 0% 50%, ${green500} 50% 100%)`;
  }

  let pos = 0;
  let ret = "linear-gradient(to right";

  for (let i = 0; i < numActions.value; ++i) {
    const width = row[INDEX_STRATEGY_BASE + i * 2] * 100;
    const color = actionColors.value[i];
    ret += `, ${color} ${pos}% ${pos + width}%`;
    pos += width;
  }

  ret += ")";
  return ret;
};

const strategyBarBgSize = (row: number[]) => {
  const weight = row[INDEX_WEIGHT];
  if (weight === 0) {
    return "0% 100%";
  } else if (displayOptions.barWidth === "normalized") {
    return `${(weight / maxWeight.value) * 100}% 100%`;
  } else if (displayOptions.barWidth === "absolute") {
    return `${weight * 100}% 100%`;
  } else {
    return "100% 100%";
  }
};

const actionBarBg = (index: number, row: number[]) => {
  const color = actionColors.value[index];
  const value = row[INDEX_STRATEGY_BASE + index * 2];
  return `linear-gradient(to right, ${color} 0% ${
    value * 100
  }%, ${neutral200} ${value * 100}% 100%)`;
};

const exportSummary = async () => {
  const data: string[] = [];

  if (props.tableMode !== "chance") {
    if (!props.results) return;

    const ary = ["Hand", "Weight", "Combos"];

    if (!props.results.isEmpty) {
      ary.push("Equity", "EV", "EQR");
    }

    if (numActions.value > 0) {
      const actions = (props.selectedSpot as SpotPlayer).actions;
      for (let i = actions.length - 1; i >= 0; --i) {
        const action = actions[i];
        const amount = action.amount === "0" ? "" : ` ${action.amount}`;
        ary.push(`${action.name}${amount} %`);
        ary.push(`${action.name}${amount} EV`);
      }
    }

    data.push(ary.join(","));

    const startIndex = props.results.isEmpty
      ? INDEX_STRATEGY_BASE
      : INDEX_EQUITY;

    for (const row of resultsSorted.value) {
      const card1 = row[0] & 0xff;
      const card2 = row[0] >>> 8;
      const pairStr = cardStr(card2) + cardStr(card1);
      const ary = [
        pairStr,
        row[INDEX_WEIGHT],
        row[INDEX_NORMALIZER],
        ...row.slice(startIndex),
      ];
      data.push(ary.join(","));
    }
  } else {
    if (!props.chanceReports) return;

    const ary = [columns.value[0].label, "Combos", "Equity", "EV", "EQR"];

    if (numActions.value > 0) {
      const actions = (props.selectedSpot as SpotPlayer).actions;
      for (let i = actions.length - 1; i >= 0; --i) {
        const action = actions[i];
        const amount = action.amount === "0" ? "" : ` ${action.amount}`;
        ary.push(`${action.name}${amount} %`);
      }
    }

    data.push(ary.join(","));

    for (const row of resultsSorted.value) {
      const ary = [
        cardStr(row[0] & 0xff),
        row[INDEX_WEIGHT],
        isNaN(row[INDEX_EQUITY]) ? "-" : row[INDEX_EQUITY],
        isNaN(row[INDEX_EV]) ? "-" : row[INDEX_EV],
        isNaN(row[INDEX_EQR]) ? "-" : row[INDEX_EQR],
        ...row.slice(INDEX_STRATEGY_BASE).filter((_, i) => i % 2 === 0),
      ];
      data.push(ary.join(","));
    }
  }

  const filePath = await save({
    defaultPath: "summary.csv",
    filters: [{ name: "CSV Files", extensions: ["csv"] }],
  });

  if (filePath) {
    await writeTextFile(filePath, data.join("\n"));
  }
};

/* eslint-disable prefer-const */
let strTmp = "";

watch(() => props.tableBarWidth, (newValue) => {
  if (newValue) {
    displayOptions.barWidth = newValue as any;
    updateDisplayOptions();
  }
});

if (props.tableMode !== "chance") {
  watch(() => props.tableDisplayContent, (newValue) => {
    if (newValue && displayOptions) {
      (displayOptions as DisplayOptionsBasics).content = newValue;
      updateDisplayOptions();
    }
  });
}
</script>

<style scoped>
.header-divider::before {
  content: "";
  @apply absolute left-0 -bottom-px w-full border-b border-gray-200;
}

.row-divider::before {
  content: "";
  @apply absolute left-0 top-0 w-full border-t border-gray-200;
}

/* 设置表格行过渡效果 */
tr {
  transition: background-color 0.2s ease, transform 0.2s ease;
}

/* 添加表头动效 */
thead th {
  transition: background-color 0.2s ease;
  position: relative;
  overflow: hidden;
}

thead th::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  width: 0;
  height: 2px;
  background-color: white;
  transition: width 0.2s ease;
}

thead th:hover::after {
  width: 100%;
}

/* 为卡片添加投影效果 */
td span[class*="text-"] {
  transition: transform 0.15s ease, text-shadow 0.15s ease;
}

td:hover span[class*="text-"] {
  text-shadow: 0 0 1px rgba(0,0,0,0.2);
  transform: translateY(-1px);
}

/* 增强EV值视觉效果 */
span.text-emerald-600, span.text-rose-600 {
  font-weight: 500;
  transition: all 0.2s ease;
}

span.text-emerald-600:hover, span.text-rose-600:hover {
  transform: scale(1.05);
}

/* 表格整体阴影 */
table {
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  border-radius: 0.5rem;
  overflow: hidden;
}
</style>