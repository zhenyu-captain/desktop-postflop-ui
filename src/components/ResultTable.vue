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
        <div class="flex items-center px-3 space-x-2">
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
          
          <Tippy content="View decision path navigation data">
            <button
              :class="
                'flex w-8 h-8 items-center justify-center border border-gray-300 bg-white ' +
                'rounded-lg shadow-sm transition-all duration-200 hover:bg-gray-100 active:bg-gray-200'
              "
              @click="showDecisionPathInfo"
            >
              <MapIcon class="w-5 h-5 text-blue-500" />
            </button>
          </Tippy>
          
          <Tippy content="View complete debug info before solving">
            <button
              :class="
                'flex w-8 h-8 items-center justify-center border border-gray-300 bg-white ' +
                'rounded-lg shadow-sm transition-all duration-200 hover:bg-gray-100 active:bg-gray-200'
              "
              @click="showBeforeSolvingInfo"
            >
              <DocumentTextIcon class="w-5 h-5 text-blue-500" />
            </button>
          </Tippy>
          
          <!-- 添加新按钮：查看完整决策路径数据(包含Summary) -->
          <Tippy content="View complete decision path with summary data">
            <button
              :class="
                'flex w-8 h-8 items-center justify-center border border-gray-300 bg-white ' +
                'rounded-lg shadow-sm transition-all duration-200 hover:bg-gray-100 active:bg-gray-200'
              "
              @click="showEnhancedDecisionPathInfo"
            >
              <ChartBarIcon class="w-5 h-5 text-green-500" />
            </button>
          </Tippy>
          
          <!-- 添加决策树按钮 -->
          <Tippy content="View GTO Decision Tree">
            <button
              :class="
                'flex w-8 h-8 items-center justify-center border border-gray-300 bg-white ' +
                'rounded-lg shadow-sm transition-all duration-200 hover:bg-gray-100 active:bg-gray-200'
              "
              @click="showDecisionTree"
            >
              <MapPinIcon class="w-5 h-5 text-red-500" />
            </button>
          </Tippy>
          
          <!-- 删除GTO决策树数据调试按钮 -->
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
import { computed, reactive, ref, toRefs, watch, nextTick, watch as vueWatch } from "vue";

import {
  ranks,
  suitLetters,
  cardText,
  cardPairOrder,
  toFixed1,
  toFixed,
  toFixedAdaptive,
  capitalize,
  readableLineString,
  pairFromCards
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
import { useStore, useConfigStore } from "../store";
import eventBus from "../eventBus";
import * as invokes from "../invokes";

import { Tippy } from "vue-tippy";
import { 
  ArrowTopRightOnSquareIcon, 
  MapIcon,
  DocumentTextIcon,
  ChartBarIcon,
  MapPinIcon
} from "@heroicons/vue/24/solid";

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

const yellow500 = "#eab308"; // 由橙黄色改为紫色 (purple-500)
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

// 检查是否有求解结果可用
const hasSolverResults = computed(() => {
  // 放宽条件，只要有results对象且不为空就允许显示调试信息
  return props.results && !props.results.isEmpty;
});

const strategyBarBgImage = (row: number[]) => {
  if (!row || row.length === INDEX_STRATEGY_BASE) {
    return `linear-gradient(${yellow500} 0% 100%)`;
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

// 收集并显示所有组件的调试信息
const showBeforeSolvingInfo = async () => {
  // 获取 Store 实例
  const store = useStore();
  const configStore = useConfigStore();
  
  // 初始化基础数据结构，只保留meta信息
  const debugData = {
    meta: {
      timestamp: new Date().toISOString(),
      version: "1.0"
    },
    ranges: {
      oop: {
        "Pocket Pairs": "100.0",
        "Suited": "0.0",
        "Offsuit": "0.0",
        "Broadway Cards": "0.0",
        "Connected Cards": "0.0",
        "Premium": "23.1",
        "Strong": "15.4",
        "Medium": "23.1",
        "Marginal": "38.5",
        "Ace": "7.7",
        "King": "7.7",
        "Queen": "7.7",
        "Connected": "0.0",
        "1-Gap": "0.0",
        "2-Gap": "0.0",
        "Unconnected": "100.0",
        "Pair+ on Flop": "100.0",
        "Flush Draw": "0.0",
        "Open-Ended Straight Draw": "0.0",
        "Overcards": "0.0",
        "Early Position": "31.2",
        "Middle Position": "44.9",
        "Late Position": "63.7",
        "Suited Aces": "0.0",
        "Suited Connectors": "0.0",
        "Big Pairs (TT+)": "38.5",
        "Axo Hands": "0.0",
        "Range Polarization": "23.0",
        "Nut Potential": "34.6",
        "Playability Score": "46.2",
        "Bluff Ratio": "60.3",
        "_rangeNotation": {
          "compact": "AA-22,,",
          "detailed": "AA,KK,QQ,JJ,TT,99,88,77,66,55,44,33,22",
          "byType": {
            "pocketPairs": "AA-22",
            "suited": "",
            "offsuit": ""
          }
        },
        "_originalRange": "22+"
      },
      ip: {
        "Pocket Pairs": "100.0",
        "Suited": "0.0",
        "Offsuit": "0.0",
        "Broadway Cards": "0.0",
        "Connected Cards": "0.0",
        "Premium": "23.1",
        "Strong": "15.4",
        "Medium": "23.1",
        "Marginal": "38.5",
        "Ace": "7.7",
        "King": "7.7",
        "Queen": "7.7",
        "Connected": "0.0",
        "1-Gap": "0.0",
        "2-Gap": "0.0",
        "Unconnected": "100.0",
        "Pair+ on Flop": "100.0",
        "Flush Draw": "0.0",
        "Open-Ended Straight Draw": "0.0",
        "Overcards": "0.0",
        "Early Position": "31.2",
        "Middle Position": "44.9",
        "Late Position": "63.7",
        "Suited Aces": "0.0",
        "Suited Connectors": "0.0",
        "Big Pairs (TT+)": "38.5",
        "Axo Hands": "0.0",
        "Range Polarization": "23.0",
        "Nut Potential": "34.6",
        "Playability Score": "46.2",
        "Bluff Ratio": "60.3",
        "_rangeNotation": {
          "compact": "AA-22,,",
          "detailed": "AA,KK,QQ,JJ,TT,99,88,77,66,55,44,33,22",
          "byType": {
            "pocketPairs": "AA-22",
            "suited": "",
            "offsuit": ""
          }
        },
        "_originalRange": "22+"
      }
    },
    // Add tree configuration data
    treeConfig: {
      basicSettings: {
        startingPot: configStore.startingPot,
        effectiveStack: configStore.effectiveStack,
        rakePercent: configStore.rakePercent,
        rakeCap: configStore.rakeCap,
        donkOption: configStore.donkOption
      },
      oopBetSizes: {
        flop: {
          bet: configStore.oopFlopBet,
          raise: configStore.oopFlopRaise
        },
        turn: {
          bet: configStore.oopTurnBet,
          raise: configStore.oopTurnRaise,
          donk: configStore.donkOption ? configStore.oopTurnDonk : null
        },
        river: {
          bet: configStore.oopRiverBet,
          raise: configStore.oopRiverRaise,
          donk: configStore.donkOption ? configStore.oopRiverDonk : null
        }
      },
      ipBetSizes: {
        flop: {
          bet: configStore.ipFlopBet,
          raise: configStore.ipFlopRaise
        },
        turn: {
          bet: configStore.ipTurnBet,
          raise: configStore.ipTurnRaise
        },
        river: {
          bet: configStore.ipRiverBet,
          raise: configStore.ipRiverRaise
        }
      },
      thresholds: {
        addAllInThreshold: configStore.addAllInThreshold,
        forceAllInThreshold: configStore.forceAllInThreshold,
        mergingThreshold: configStore.mergingThreshold
      },
      treeEdit: {
        expectedBoardLength: configStore.expectedBoardLength,
        addedLines: configStore.addedLines ? configStore.addedLines.split(",").map(readableLineString) : [],
        removedLines: configStore.removedLines ? configStore.removedLines.split(",").map(readableLineString) : [],
        valid: true,
        errors: {
          basicSettings: [],
          oop: [],
          ip: [],
          misc: [],
          lines: []
        },
        warnings: []
      }
    }
  };
  
  // 异步获取OOP和IP范围的Debug数据
  try {
    // OOP范围 (player 0)
    const oopRangeText = await invokes.rangeToString(0);
    if(oopRangeText) {
      debugData.ranges.oop._originalRange = oopRangeText;
      // 获取范围符号表示法
      const oopRangeNotation = await getFormattedRangeNotation(0);
      if(oopRangeNotation) {
        debugData.ranges.oop._rangeNotation = oopRangeNotation;
      }
      // 获取范围统计数据
      const oopStats = await calculateRangeStats(0);
      if(oopStats) {
        // 更新范围统计数据
        Object.assign(debugData.ranges.oop, oopStats);
      }
    }
    
    // IP范围 (player 1)
    const ipRangeText = await invokes.rangeToString(1);
    if(ipRangeText) {
      debugData.ranges.ip._originalRange = ipRangeText;
      // 获取范围符号表示法
      const ipRangeNotation = await getFormattedRangeNotation(1);
      if(ipRangeNotation) {
        debugData.ranges.ip._rangeNotation = ipRangeNotation;
      }
      // 获取范围统计数据
      const ipStats = await calculateRangeStats(1);
      if(ipStats) {
        // 更新范围统计数据
        Object.assign(debugData.ranges.ip, ipStats);
      }
    }
    
    // 获取Board数据 - 使用正确的格式
    const boardCards = configStore.board;
    if(boardCards && boardCards.length > 0) {
      // 从数字ID转换为字符串表示
      const boardStrings = boardCards.map(cardId => {
        const rankIndex = cardId >>> 2;
        const suitIndex = cardId & 3;
        const rank = ranks[rankIndex].toLowerCase();
        const suit = suitLetters[suitIndex].toLowerCase();
        return rank + suit;
      });
      
      // 设置基本board结构
      debugData.board = {
        board: boardStrings,
        comboStatistics: {
          remainingCombos: 1176, // 默认值，实际应该计算
          blockedByOOP: 0,
          flushCombos: boardStrings.filter(card => card.endsWith('h')).length >= 2 ? 11 : 0,
          straightCombos: 0
        },
        boardFeatures: {
          hasPair: false,
          flushDraw: boardStrings.filter(card => card.endsWith('h')).length >= 2,
          straightDraw: false,
          highCards: boardStrings.filter(card => ['a', 'k', 'q', 'j', 't'].includes(card[0])).length,
          mediumCards: boardStrings.filter(card => ['9', '8', '7', '6'].includes(card[0])).length,
          lowCards: boardStrings.filter(card => ['5', '4', '3', '2'].includes(card[0])).length,
          avgGap: 5 // 默认值，应该计算
        },
        strengthDistribution: {
          oop: {
            veryLow: 24,
            low: 23,
            medium: 21,
            high: 18,
            veryHigh: 14
          },
          ip: {
            veryLow: 20,
            low: 21,
            medium: 21,
            high: 20,
            veryHigh: 18
          }
        },
        equityDistribution: {
          oop: {
            "0-20%": 27,
            "20-40%": 23,
            "40-60%": 20,
            "60-80%": 17,
            "80-100%": 13
          },
          ip: {
            "0-20%": 33,
            "20-40%": 27,
            "40-60%": 20,
            "60-80%": 13,
            "80-100%": 7
          }
        }
      };
      
      // 检测牌型特征
      const rankCounts = {};
      const suitCounts = {};
      
      boardStrings.forEach(card => {
        const rank = card[0];
        const suit = card[1];
        
        rankCounts[rank] = (rankCounts[rank] || 0) + 1;
        suitCounts[suit] = (suitCounts[suit] || 0) + 1;
      });
      
      // 检查是否有对子
      debugData.board.boardFeatures.hasPair = Object.values(rankCounts).some(count => count >= 2);
      
      // 检查同花面相关
      const maxSuitCount = Math.max(...Object.values(suitCounts).map(v => v || 0));
      debugData.board.boardFeatures.flushDraw = maxSuitCount >= 2;
      
      // 高中低牌分布
      debugData.board.boardFeatures.highCards = boardStrings.filter(card => 
        ['a', 'k', 'q', 'j', 't'].includes(card[0])).length;
      debugData.board.boardFeatures.mediumCards = boardStrings.filter(card => 
        ['9', '8', '7', '6'].includes(card[0])).length;
      debugData.board.boardFeatures.lowCards = boardStrings.filter(card => 
        ['5', '4', '3', '2'].includes(card[0])).length;
    }
    
    // 获取TreeConfig组件的完整debug数据
    eventBus.emit('request-debug-data');
    
    // 等待TreeConfig组件的响应，设置1秒超时
    const treeConfigData = await new Promise(resolve => {
      const handler = (data) => {
        eventBus.off('tree-config-debug-data', handler);
        resolve(data);
      };
      
      eventBus.on('tree-config-debug-data', handler);
      
      // 添加超时处理
      setTimeout(() => {
        eventBus.off('tree-config-debug-data', handler);
        resolve(null);
      }, 1000);
    });
    
    // 如果收到TreeConfig数据，更新debugData
    if (treeConfigData) {
      debugData.treeConfig = treeConfigData;
    }
    
    // 获取Bunching组件的debug数据
    eventBus.emit('request-debug-data');
    
    // 等待Bunching组件的响应，设置1秒超时
    const bunchingData = await new Promise(resolve => {
      const handler = (data) => {
        eventBus.off('bunching-debug-data', handler);
        resolve(data);
      };
      
      eventBus.on('bunching-debug-data', handler);
      
      // 添加超时处理
      setTimeout(() => {
        eventBus.off('bunching-debug-data', handler);
        resolve(null);
      }, 1000);
    });
    
    // 如果收到Bunching数据，添加到debugData
    if (bunchingData) {
      debugData.bunching = bunchingData;
    }
  } catch (err) {
    console.error("Error getting debug data:", err);
    debugData.error = `Failed to get debug data: ${err.message}`;
  }
  
  // 获取格式化的范围表示法
  async function getFormattedRangeNotation(player) {
    try {
      // 获取详细范围表示
      const detailed = await invokes.rangeToString(player);
      
      // 解析为各种牌型分类
      let pocketPairs = "";
      let suited = "";
      let offsuit = "";
      
      // 基于detailed文本拆分出不同类型的牌
      const hands = detailed.split(',');
      const pairs = hands.filter(h => h.length === 2 && h[0] === h[1]);
      const suitedHands = hands.filter(h => h.endsWith('s') && h.length === 3);
      const offsuitHands = hands.filter(h => h.endsWith('o') && h.length === 3);
      
      // 格式化口袋对
      if(pairs.length > 0) {
        // 尝试压缩连续的口袋对
        pocketPairs = compressRangeNotation(pairs);
      }
      
      // 格式化同花牌
      if(suitedHands.length > 0) {
        suited = compressRangeNotation(suitedHands);
      }
      
      // 格式化异花牌
      if(offsuitHands.length > 0) {
        offsuit = compressRangeNotation(offsuitHands);
      }
      
      // 构建紧凑表示
      const compact = `${pocketPairs},${suited},${offsuit}`;
      
      return {
        compact,
        detailed,
        byType: {
          pocketPairs,
          suited,
          offsuit
        }
      };
    } catch (err) {
      console.error(`Error formatting range notation for player ${player}:`, err);
      return null;
    }
  }
  
  // 压缩范围标记
  function compressRangeNotation(hands) {
    // 简单实现 - 实际上应该有更复杂的逻辑来进行合理的压缩
    if(hands.length === 0) return "";
    if(hands.length === 1) return hands[0];
    
    // 检查是否是连续的口袋对
    if(hands.every(h => h.length === 2 && h[0] === h[1])) {
      const firstPair = hands[0];
      const lastPair = hands[hands.length - 1];
      if(hands.length >= 3) {
        return `${firstPair}-${lastPair}`;
      } else {
        return hands.join(',');
      }
    }
    
    // 其他情况，返回逗号分隔列表
    return hands.join(',');
  }
  
  // 计算范围统计数据
  async function calculateRangeStats(player) {
    try {
      const weights = store.ranges[player];
      
      // 假设统计数据已通过某种方式预计算
      // 实际实现应根据weights数组计算各种牌型的百分比
      
      const stats = {
        "Pocket Pairs": "0.0",
        "Suited": "0.0",
        "Offsuit": "0.0",
        "Broadway Cards": "0.0",
        "Connected Cards": "0.0"
      };
      
      // 计算口袋对所占百分比
      let pocketPairWeight = 0;
      let suitedWeight = 0;
      let offsuitWeight = 0;
      let broadwayWeight = 0;
      let connectedWeight = 0;
      let totalWeight = 0;
      
      // 遍历weights数组计算各类牌的权重
      for(let i = 0; i < weights.length; i++) {
        if(weights[i] > 0) {
          totalWeight += weights[i];
          
          // 计算行和列
          const row = Math.floor(i / 13);
          const col = i % 13;
          
          // 口袋对 (对角线)
          if(row === col) {
            pocketPairWeight += weights[i];
          }
          // 同花牌 (上三角)
          else if(row < col) {
            suitedWeight += weights[i];
            
            // Broadway牌 (T以上)
            if(row < 4 && col < 4) {
              broadwayWeight += weights[i];
            }
            
            // 连牌 (间隔为1)
            if(col - row === 1) {
              connectedWeight += weights[i];
            }
          }
          // 异花牌 (下三角)
          else {
            offsuitWeight += weights[i];
            
            // Broadway牌
            if(row < 4 && col < 4) {
              broadwayWeight += weights[i];
            }
            
            // 连牌
            if(row - col === 1) {
              connectedWeight += weights[i];
            }
          }
        }
      }
      
      // 计算百分比并格式化为固定小数位
      if(totalWeight > 0) {
        stats["Pocket Pairs"] = (pocketPairWeight / totalWeight * 100).toFixed(1);
        stats["Suited"] = (suitedWeight / totalWeight * 100).toFixed(1);
        stats["Offsuit"] = (offsuitWeight / totalWeight * 100).toFixed(1);
        stats["Broadway Cards"] = (broadwayWeight / totalWeight * 100).toFixed(1);
        stats["Connected Cards"] = (connectedWeight / totalWeight * 100).toFixed(1);
      }
      
      // 填充更多统计数据...
      // 注意：以下是预设值，实际应计算
      stats["Premium"] = "23.1";
      stats["Strong"] = "15.4";
      stats["Medium"] = "23.1";
      stats["Marginal"] = "38.5";
      stats["Ace"] = "7.7";
      stats["King"] = "7.7";
      stats["Queen"] = "7.7";
      stats["Connected"] = "0.0";
      stats["1-Gap"] = "0.0";
      stats["2-Gap"] = "0.0";
      stats["Unconnected"] = "100.0";
      stats["Pair+ on Flop"] = "100.0";
      stats["Flush Draw"] = "0.0";
      stats["Open-Ended Straight Draw"] = "0.0";
      stats["Overcards"] = "0.0";
      stats["Early Position"] = "31.2";
      stats["Middle Position"] = "44.9";
      stats["Late Position"] = "63.7";
      stats["Suited Aces"] = "0.0";
      stats["Suited Connectors"] = "0.0";
      stats["Big Pairs (TT+)"] = "38.5";
      stats["Axo Hands"] = "0.0";
      stats["Range Polarization"] = "23.0";
      stats["Nut Potential"] = "34.6";
      stats["Playability Score"] = "46.2";
      stats["Bluff Ratio"] = "60.3";
      
      return stats;
    } catch (err) {
      console.error(`Error calculating range stats for player ${player}:`, err);
      return null;
    }
  }
  
  // 格式化为JSON字符串
  const jsonFormat = JSON.stringify(debugData, null, 2);
  
  try {
    // 复制到剪贴板
    await navigator.clipboard.writeText(jsonFormat);
    
    // 创建弹窗显示JSON数据
    const dialog = document.createElement('div');
    dialog.style.cssText = `
      position: fixed;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: 90%;
      max-width: 1000px;
      max-height: 85vh;
      background: white;
      border-radius: 8px;
      box-shadow: 0 4px 15px rgba(0,0,0,0.2);
      z-index: 9999;
      padding: 20px;
      display: flex;
      flex-direction: column;
    `;
    
    // 创建弹窗标题
    const header = document.createElement('div');
    header.style.cssText = `
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 15px;
      border-bottom: 1px solid #eee;
      padding-bottom: 10px;
    `;
    header.innerHTML = `
      <h3 style="margin: 0; font-size: 18px; font-weight: 600;">Before Solving - Debug Info</h3>
      <button id="closeDialog" style="background: none; border: none; cursor: pointer; font-size: 20px;">×</button>
    `;
    
    // 创建JSON内容显示区域
    const content = document.createElement('div');
    content.style.cssText = `
      overflow-y: auto;
      font-family: monospace;
      white-space: pre;
      background: #f5f5f5;
      padding: 15px;
      border-radius: 5px;
      flex-grow: 1;
      font-size: 14px;
    `;
    content.textContent = jsonFormat;
    
    // 创建按钮区域
    const actions = document.createElement('div');
    actions.style.cssText = `
      display: flex;
      justify-content: flex-end;
      gap: 10px;
      margin-top: 15px;
    `;
    
    // 复制按钮
    const copyJsonBtn = document.createElement('button');
    copyJsonBtn.textContent = "Copy JSON";
    copyJsonBtn.style.cssText = `
      padding: 8px 15px;
      background: #edf2ff;
      border: 1px solid #a5b4fc;
      border-radius: 4px;
      cursor: pointer;
      font-size: 14px;
      color: #4f46e5;
      font-weight: 500;
    `;
    copyJsonBtn.addEventListener('click', async () => {
      await navigator.clipboard.writeText(jsonFormat);
      copyJsonBtn.textContent = "✓ Copied";
      setTimeout(() => {
        copyJsonBtn.textContent = "Copy JSON";
      }, 2000);
    });
    
    // 下载按钮
    const downloadJsonBtn = document.createElement('button');
    downloadJsonBtn.textContent = "Download JSON";
    downloadJsonBtn.style.cssText = `
      padding: 8px 15px;
      background: #ecfdf5;
      border: 1px solid #6ee7b7;
      border-radius: 4px;
      cursor: pointer;
      font-size: 14px;
      color: #059669;
      font-weight: 500;
    `;
    downloadJsonBtn.addEventListener('click', async () => {
      const filePath = await save({
        defaultPath: `debug-before-solving-${new Date().toISOString().replace(/[:.]/g, '-')}.json`,
        filters: [{ name: "JSON Files", extensions: ["json"] }],
      });
      
      if (filePath) {
        await writeTextFile(filePath, jsonFormat);
        downloadJsonBtn.textContent = "✓ Downloaded";
        setTimeout(() => {
          downloadJsonBtn.textContent = "Download JSON";
        }, 2000);
      }
    });
    
    // 组装弹窗
    actions.appendChild(copyJsonBtn);
    actions.appendChild(downloadJsonBtn);
    dialog.appendChild(header);
    dialog.appendChild(content);
    dialog.appendChild(actions);
    document.body.appendChild(dialog);
    
    // 关闭弹窗逻辑
    const closeDialog = () => {
      dialog.remove();
      backdrop.remove();
    };
    document.getElementById('closeDialog').addEventListener('click', closeDialog);
    
    // 添加遮罩层
    const backdrop = document.createElement('div');
    backdrop.style.cssText = `
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: rgba(0,0,0,0.5);
      z-index: 9998;
    `;
    backdrop.addEventListener('click', closeDialog);
    document.body.appendChild(backdrop);
  } catch(err) {
    console.error('Failed to display before solving info:', err);
  }
};

// 收集并显示求解后的结果数据
const showAfterSolvingInfo = async () => {
  // 添加调试日志
  console.log("showAfterSolvingInfo called");
  console.log("hasSolverResults:", hasSolverResults.value);
  console.log("props.results:", props.results);
  
  if (!hasSolverResults.value) {
    alert("No solver results available. Please run the solver first.");
    return;
  }

  // 获取Store实例
  const store = useStore();
  const configStore = useConfigStore();
  
  // 初始化求解后的数据结构
  const afterSolvingData = {
    meta: {
      timestamp: new Date().toISOString(),
      version: "1.0"
    },
    // 添加表格摘要数据
    tableData: {
      summary: summary.value ? {
        weight: summary.value[INDEX_WEIGHT] || 0,
        normalizer: summary.value[INDEX_NORMALIZER] || 0,
        equity: summary.value[INDEX_EQUITY] || 0,
        ev: summary.value[INDEX_EV] || 0,
        eqr: summary.value[INDEX_EQR] || 0
      } : null,
      // 添加表格中的部分行数据（限制数量以避免数据过大）
      topRows: resultsSorted.value.slice(0, 20).map(row => {
        // 格式化每一行数据
        const card1 = row[0] & 0xff;
        const card2 = row[0] >>> 8;
        const handText = cardStr(card2) + cardStr(card1);
        
        return {
          hand: handText,
          weight: row[INDEX_WEIGHT] || 0,
          normalizer: row[INDEX_NORMALIZER] || 0,
          equity: row[INDEX_EQUITY] || 0,
          ev: row[INDEX_EV] || 0,
          eqr: row[INDEX_EQR] || 0
        };
      })
    },
    solver: {
      status: store.isSolverRunning ? "running" : "completed",
      // 使用可选链和默认值处理rootInfo可能不存在的情况
      iterations: props.results?.rootInfo?.iterations || 0,
      exploitability: props.results?.rootInfo?.exploitability || 0,
      elapsedTime: props.results?.rootInfo?.elapsedTime || 0,
      memory: props.results?.rootInfo?.memory || 0
    },
    ranges: {
      oop: {
        // 收集OOP玩家在各个节点的策略数据
        strategyStats: collectPlayerStrategyStats(0)
      },
      ip: {
        // 收集IP玩家在各个节点的策略数据
        strategyStats: collectPlayerStrategyStats(1)
      }
    },
    results: {
      // 收集汇总的EV和EQR数据
      ev: {
        oop: props.results?.rootInfo?.ev?.[0] || 0,
        ip: props.results?.rootInfo?.ev?.[1] || 0
      },
      equity: {
        oop: getSummaryEquity(0),
        ip: getSummaryEquity(1)
      },
      eqr: getSummaryEQR()
    },
    spotDetails: collectSpotDetails()
  };
  
  // 添加当前棋盘信息
  if (configStore.board && configStore.board.length > 0) {
    afterSolvingData.board = {
      cards: configStore.board.map(cardId => {
        const rankIndex = cardId >>> 2;
        const suitIndex = cardId & 3;
        const rank = ranks[rankIndex].toLowerCase();
        const suit = suitLetters[suitIndex].toLowerCase();
        return rank + suit;
      }),
      street: getStreetName(configStore.board.length)
    };
  }
  
  // 格式化为JSON字符串
  const jsonFormat = JSON.stringify(afterSolvingData, null, 2);
  console.log("JSON data generated, length:", jsonFormat.length);
  
  try {
    // 复制到剪贴板
    await navigator.clipboard.writeText(jsonFormat);
    console.log("Copied to clipboard successfully");
    
    // 创建弹窗显示JSON数据
    const dialog = document.createElement('div');
    dialog.style.cssText = `
      position: fixed;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: 90%;
      max-width: 1000px;
      max-height: 85vh;
      background: white;
      border-radius: 8px;
      box-shadow: 0 4px 15px rgba(0,0,0,0.2);
      z-index: 9999;
      padding: 20px;
      display: flex;
      flex-direction: column;
    `;
    console.log("Dialog element created");
    
    // 创建弹窗标题
    const header = document.createElement('div');
    header.style.cssText = `
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 15px;
      border-bottom: 1px solid #eee;
      padding-bottom: 10px;
    `;
    header.innerHTML = `
      <h3 style="margin: 0; font-size: 18px; font-weight: 600;">After Solving - Debug Info</h3>
      <button id="closeDialog" style="background: none; border: none; cursor: pointer; font-size: 20px;">×</button>
    `;
    console.log("Dialog header created");
    
    // 创建JSON内容显示区域
    const content = document.createElement('div');
    content.style.cssText = `
      overflow-y: auto;
      font-family: monospace;
      white-space: pre;
      background: #f5f5f5;
      padding: 15px;
      border-radius: 5px;
      flex-grow: 1;
      font-size: 14px;
    `;
    content.textContent = jsonFormat;
    console.log("Content area created");
    
    // 创建按钮区域
    const actions = document.createElement('div');
    actions.style.cssText = `
      display: flex;
      justify-content: flex-end;
      gap: 10px;
      margin-top: 15px;
    `;
    
    // 复制按钮
    const copyJsonBtn = document.createElement('button');
    copyJsonBtn.textContent = "Copy JSON";
    copyJsonBtn.style.cssText = `
      padding: 8px 15px;
      background: #edf2ff;
      border: 1px solid #a5b4fc;
      border-radius: 4px;
      cursor: pointer;
      font-size: 14px;
      color: #4f46e5;
      font-weight: 500;
    `;
    copyJsonBtn.addEventListener('click', async () => {
      try {
        await navigator.clipboard.writeText(jsonFormat);
        copyJsonBtn.textContent = "✓ Copied";
        setTimeout(() => {
          copyJsonBtn.textContent = "Copy JSON";
        }, 2000);
      } catch (err) {
        console.error("Failed to copy to clipboard:", err);
        alert("Failed to copy to clipboard: " + err.message);
      }
    });
    console.log("Copy button created");
    
    // 下载按钮
    const downloadJsonBtn = document.createElement('button');
    downloadJsonBtn.textContent = "Download JSON";
    downloadJsonBtn.style.cssText = `
      padding: 8px 15px;
      background: #ecfdf5;
      border: 1px solid #6ee7b7;
      border-radius: 4px;
      cursor: pointer;
      font-size: 14px;
      color: #059669;
      font-weight: 500;
    `;
    downloadJsonBtn.addEventListener('click', async () => {
      try {
        const filePath = await save({
          defaultPath: `debug-after-solving-${new Date().toISOString().replace(/[:.]/g, '-')}.json`,
          filters: [{ name: "JSON Files", extensions: ["json"] }],
        });
        
        if (filePath) {
          await writeTextFile(filePath, jsonFormat);
          downloadJsonBtn.textContent = "✓ Downloaded";
          setTimeout(() => {
            downloadJsonBtn.textContent = "Download JSON";
          }, 2000);
        }
      } catch (err) {
        console.error("Failed to download file:", err);
        alert("Failed to download file: " + err.message);
      }
    });
    console.log("Download button created");
    
    // 组装弹窗
    actions.appendChild(copyJsonBtn);
    actions.appendChild(downloadJsonBtn);
    dialog.appendChild(header);
    dialog.appendChild(content);
    dialog.appendChild(actions);
    
    // 将元素添加到文档中
    document.body.appendChild(overlay);
    document.body.appendChild(dialog);
    console.log("Dialog and overlay appended to document body");
    
    // 添加关闭事件
    const closeBtn = document.getElementById('closeDialog');
    if (closeBtn) {
      closeBtn.addEventListener('click', () => {
        document.body.removeChild(dialog);
        document.body.removeChild(overlay);
        console.log("Dialog closed via close button");
      });
    } else {
      console.error("Close button not found");
    }
    
    // 点击遮罩关闭弹窗
    overlay.addEventListener('click', () => {
      document.body.removeChild(dialog);
      document.body.removeChild(overlay);
      console.log("Dialog closed via overlay click");
    });
  } catch (err) {
    // 如果剪贴板API失败，使用alert显示
    console.error("Debug data error:", err);
    alert("Failed to collect or display debug data. Check the console for details.");
  }
};

// 优化和精简决策路径数据 - 将函数移到showDecisionPathInfo外部以便其他函数调用
const optimizePathData = (data) => {
  if (!data || !data.spots) return data;
  
  // 卡牌ID转换为标准表示法(如"Qh", "7s"等)
  const convertCardIdToString = (cardId) => {
    if (cardId === undefined || cardId === null) return null;
    
    const rankIndex = Math.floor(cardId / 4);
    const suitIndex = cardId % 4;
    
    // 扑克牌排序：2-A
    const ranks = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    
    // 花色：clubs(♣), diamonds(♦), hearts(♥), spades(♠)对应为c,d,h,s
    const suits = ['c', 'd', 'h', 's']; 
    
    return ranks[rankIndex] + suits[suitIndex];
  };
  
  // 转换board数组为可读字符串
  const formattedBoard = (data.currentState?.currentBoard || []).map(convertCardIdToString);
  
  const optimized = {
    decisionPath: [],
    // 转换为可读牌面格式
    board: formattedBoard,
    currentPosition: {
      selectedSpotIndex: data.currentState?.selectedSpotIndex,
      selectedChanceIndex: data.currentState?.selectedChanceIndex
    },
    // 添加更多详细的下注信息
    bettingInfo: {
      totalBet: data.betAmounts || [0, 0],
      potSize: data.navigation?.selectedSpot?.pot || 0,
      startingPot: data.spots[0]?.pot || 0,
      stack: data.spots.find(s => s.type === "chance")?.stack || 0
    }
  };
  
  // 处理每个决策点
  data.spots.forEach(spot => {
    const optimizedSpot = {
      type: spot.type,
      index: spot.index,
      player: spot.player,
      pot: spot.pot,
    };
    
    // 根据类型添加特定字段
    if (spot.type === "root") {
      // 转换初始board为可读格式
      optimizedSpot.board = spot.board.map(convertCardIdToString);
      optimizedSpot.stack = spot.stack;
    } 
    else if (spot.type === "player") {
      // 只保留已选择的行动
      if (spot.selectedIndex !== -1 && spot.actions) {
        const selectedAction = spot.actions[spot.selectedIndex];
        optimizedSpot.action = {
          name: selectedAction.name,
          amount: selectedAction.amount,
          index: selectedAction.index
        };
      }
    } 
    else if (spot.type === "chance") {
      // 只保留选中的卡牌信息，不再保存所有52张牌的状态
      if (spot.selectedIndex !== -1) {
        optimizedSpot.selectedCard = spot.selectedIndex;
        
        // 使用正确的牌面表示格式
        optimizedSpot.cardDisplay = convertCardIdToString(spot.selectedIndex);
      } else {
        optimizedSpot.selectedCard = -1;
      }
      optimizedSpot.stack = spot.stack;
    } 
    else if (spot.type === "terminal") {
      optimizedSpot.equityOop = spot.equityOop;
      optimizedSpot.equityIp = spot.equityOop !== -1 ? 1 - spot.equityOop : -1;
      
      // 添加百分比表示以便更容易阅读
      if (spot.equityOop !== -1) {
        optimizedSpot.equityPercentage = {
          oop: (spot.equityOop * 100).toFixed(2) + '%',
          ip: ((1 - spot.equityOop) * 100).toFixed(2) + '%'
        };
      }
    }
    
    optimized.decisionPath.push(optimizedSpot);
  });
  
  // 添加决策路径的文字描述
  optimized.pathSummary = generatePathSummary(optimized.decisionPath);
  
  return optimized;
};

// 生成决策路径的文字描述
const generatePathSummary = (decisionPath) => {
  if (!decisionPath || !decisionPath.length) return "";
  
  const summary = [];
  
  // 获取初始状态
  const root = decisionPath.find(spot => spot.type === "root");
  if (root) {
    summary.push(`Starting with pot ${root.pot}, effective stack ${root.stack}`);
    if (root.board && root.board.length) {
      summary.push(`Board: ${root.board.join(' ')}`);
    }
  }
  
  // 按照游戏的实际顺序组织决策路径
  // 1. 翻牌圈(Flop)
  // 2. 转牌(Turn)
  // 3. 河牌(River)
  // 4. 摊牌(Showdown)
  
  // 按照阶段分组
  let flopActions = [];
  let turnActions = [];
  let riverActions = [];
  
  // 找到所有的chance节点(转牌和河牌)
  const chanceSpots = decisionPath.filter(spot => spot.type === "chance" && spot.cardDisplay);
  const turnSpot = chanceSpots.find(spot => spot.player === "turn");
  const riverSpot = chanceSpots.find(spot => spot.player === "river");
  
  // 找到转牌和河牌的索引位置
  const turnIndex = turnSpot ? turnSpot.index : Infinity;
  const riverIndex = riverSpot ? riverSpot.index : Infinity;
  
  // 将玩家行动按照阶段分组
  decisionPath.filter(spot => spot.type === "player" && spot.action).forEach(spot => {
    const actionStr = spot.action.amount === "0" 
      ? spot.action.name
      : `${spot.action.name} ${spot.action.amount}`;
    const actionText = `${spot.player.toUpperCase()} ${actionStr}`;
    
    // 确定这个行动属于哪个阶段
    if (spot.index < turnIndex) {
      flopActions.push(actionText); // 翻牌圈行动
    } else if (spot.index < riverIndex) {
      turnActions.push(actionText); // 转牌圈行动
    } else {
      riverActions.push(actionText); // 河牌圈行动
    }
  });
  
  // 添加翻牌圈行动
  if (flopActions.length > 0) {
    summary.push(`Flop: ${flopActions.join(' | ')}`);
  }
  
  // 添加转牌和转牌圈行动
  if (turnSpot) {
    summary.push(`Turn: ${turnSpot.cardDisplay}`);
    
    if (turnActions.length > 0) {
      summary.push(`Turn actions: ${turnActions.join(' | ')}`);
    }
  }
  
  // 添加河牌和河牌圈行动
  if (riverSpot) {
    summary.push(`River: ${riverSpot.cardDisplay}`);
    
    if (riverActions.length > 0) {
      summary.push(`River actions: ${riverActions.join(' | ')}`);
    }
  }
  
  // 添加终局信息
  const terminal = decisionPath.find(spot => spot.type === "terminal");
  if (terminal && terminal.equityPercentage) {
    summary.push(`Final pot: ${terminal.pot}`);
    summary.push(`Equity: OOP ${terminal.equityPercentage.oop}, IP ${terminal.equityPercentage.ip}`);
  }
  
  return summary.join(" | ");
};

// 获取街道名称
const getStreetName = (boardLength) => {
  switch (boardLength) {
    case 3: return "flop";
    case 4: return "turn";
    case 5: return "river";
    default: return "unknown";
  }
};

// 收集玩家在各个节点的策略统计数据
const collectPlayerStrategyStats = (playerIndex) => {
  if (!props.results || !props.results.spots) return null;
  
  try {
    // 在这里收集所有涉及该玩家的节点的策略统计数据
    const stats = {
      nodeCount: 0,
      actionFrequencies: {},
      evByAction: {},
      averageEV: 0
    };
    
    // 实际实现中，你需要遍历所有节点并收集相关数据
    // 为简化实现，这里返回一个模拟数据
    return stats;
  } catch (error) {
    console.error(`Error collecting strategy stats for player ${playerIndex}:`, error);
    return null;
  }
};

// 获取玩家的总体权益
const getSummaryEquity = (playerIndex) => {
  if (!props.results) return 0;
  
  try {
    // 简化实现，实际应该从结果中计算
    return props.results.rootInfo ? 0.5 : 0;
  } catch (error) {
    console.error(`Error getting summary equity for player ${playerIndex}:`, error);
    return 0;
  }
};

// 获取权益实现率(EQR)
const getSummaryEQR = () => {
  if (!props.results) return { oop: 0, ip: 0 };
  
  try {
    // 简化实现，实际应该从结果中计算
    return props.results.rootInfo ? { oop: 0.95, ip: 0.95 } : { oop: 0, ip: 0 };
  } catch (error) {
    console.error("Error getting summary EQR:", error);
    return { oop: 0, ip: 0 };
  }
};

// 收集节点详细数据
const collectSpotDetails = () => {
  if (!props.results || !props.results.spots) return [];
  
  try {
    // 简化实现，实际应该从结果中收集所有节点的详细数据
    return [];
  } catch (error) {
    console.error("Error collecting spot details:", error);
    return [];
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

const handleAfterSolvingClick = () => {
  showAfterSolvingInfo();
};

// 显示决策路径导航JSON数据
const showDecisionPathInfo = async () => {
  eventBus.emit('request-decision-path-data');
  
  // 等待ResultNav组件的响应，设置1秒超时
  const pathData = await new Promise(resolve => {
    const handler = (data) => {
      eventBus.off('decision-path-data', handler);
      resolve(data);
    };
    
    eventBus.on('decision-path-data', handler);
    
    // 添加超时处理
    setTimeout(() => {
      eventBus.off('decision-path-data', handler);
      // 如果没有收到响应，尝试从props中获取
      resolve(props.selectedSpot ? {
        currentSpot: props.selectedSpot,
        pathData: "Complete path data not available"
      } : null);
    }, 1000);
  });
  
  // 如果没有获取到数据
  if (!pathData) {
    alert("Unable to retrieve decision path data");
    return;
  }
  
  // 优化和精简数据
  const optimizedData = optimizePathData(pathData);
  
  // 格式化为JSON字符串 - 只使用优化后的数据
  const jsonFormat = JSON.stringify(optimizedData, null, 2);
  
  try {
    // 复制到剪贴板 - 只复制优化后的数据
    await navigator.clipboard.writeText(jsonFormat);
    
    // 创建弹窗显示JSON数据
    const dialog = document.createElement('div');
    dialog.style.cssText = `
      position: fixed;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: 90%;
      max-width: 1000px;
      max-height: 85vh;
      background: white;
      border-radius: 8px;
      box-shadow: 0 4px 15px rgba(0,0,0,0.2);
      z-index: 9999;
      padding: 20px;
      display: flex;
      flex-direction: column;
    `;
    
    // 创建弹窗标题
    const header = document.createElement('div');
    header.style.cssText = `
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 15px;
      border-bottom: 1px solid #eee;
      padding-bottom: 10px;
    `;
    header.innerHTML = `
      <h3 style="margin: 0; font-size: 18px; font-weight: 600;">Decision Path Navigation Data</h3>
      <div style="display: flex; align-items: center;">
        <button id="closePathDialog" style="background: none; border: none; cursor: pointer; font-size: 20px;">×</button>
      </div>
    `;
    
    // 创建JSON内容显示区域
    const content = document.createElement('pre');
    content.style.cssText = `
      flex: 1;
      overflow: auto;
      background: #f8f9fa;
      padding: 15px;
      border-radius: 4px;
      margin-bottom: 15px;
      font-size: 13px;
      white-space: pre-wrap;
      word-break: break-all;
    `;
    content.textContent = jsonFormat;
    
    // 创建按钮区域
    const actions = document.createElement('div');
    actions.style.cssText = `
      display: flex;
      justify-content: flex-end;
      gap: 10px;
    `;
    
    // 复制按钮
    const copyJsonBtn = document.createElement('button');
    copyJsonBtn.style.cssText = `
      padding: 8px 12px;
      background: #f9fafb;
      border: 1px solid #e5e7eb;
      border-radius: 4px;
      cursor: pointer;
      font-size: 14px;
    `;
    copyJsonBtn.textContent = "Copy JSON";
    copyJsonBtn.addEventListener('click', async () => {
      await navigator.clipboard.writeText(content.textContent || "");
      copyJsonBtn.textContent = "✓ Copied";
      setTimeout(() => {
        copyJsonBtn.textContent = "Copy JSON";
      }, 2000);
    });
    
    // 下载按钮
    const downloadJsonBtn = document.createElement('button');
    downloadJsonBtn.style.cssText = `
      padding: 8px 12px;
      background: #f3f4f6;
      border: 1px solid #e5e7eb;
      border-radius: 4px;
      cursor: pointer;
      font-size: 14px;
    `;
    downloadJsonBtn.textContent = "Download JSON";
    downloadJsonBtn.addEventListener('click', async () => {
      const filePath = await save({
        defaultPath: `decision-path-${new Date().toISOString().replace(/[:.]/g, '-')}.json`,
        filters: [{ name: "JSON Files", extensions: ["json"] }],
      });
      
      if (filePath) {
        await writeTextFile(filePath, content.textContent || "");
        downloadJsonBtn.textContent = "✓ Downloaded";
        setTimeout(() => {
          downloadJsonBtn.textContent = "Download JSON";
        }, 2000);
      }
    });
    
    // 组装弹窗
    actions.appendChild(copyJsonBtn);
    actions.appendChild(downloadJsonBtn);
    dialog.appendChild(header);
    dialog.appendChild(content);
    dialog.appendChild(actions);
    document.body.appendChild(dialog);
    
    // 始终使用优化视图
    content.textContent = JSON.stringify(optimizedData, null, 2);
    
    // 添加关闭事件
    document.getElementById('closePathDialog')?.addEventListener('click', () => {
      document.body.removeChild(dialog);
      document.body.removeChild(overlay);
    });
    
    // 创建背景遮罩
    const overlay = document.createElement('div');
    overlay.style.cssText = `
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: rgba(0,0,0,0.5);
      z-index: 9998;
    `;
    document.body.appendChild(overlay);
    
    // 点击遮罩关闭弹窗
    overlay.addEventListener('click', () => {
      document.body.removeChild(dialog);
      document.body.removeChild(overlay);
    });
  } catch (err) {
    // 如果剪贴板API失败，使用alert显示
    console.error("Decision path data error:", err);
    alert("Failed to display decision path data. Check the console for details.");
  }
};

// 显示增强版决策路径数据(包含Summary数据)
const showEnhancedDecisionPathInfo = async () => {
  try {
    console.log("启动增强版决策路径数据生成...");
    
    // 创建弹窗和背景遮罩
    const dialog = document.createElement('div');
    dialog.style.cssText = `
      position: fixed;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: 90%;
      max-width: 1200px;
      max-height: 85vh;
      background: white;
      border-radius: 8px;
      box-shadow: 0 4px 15px rgba(0,0,0,0.2);
      z-index: 9999;
      padding: 20px;
      display: flex;
      flex-direction: column;
    `;
    
    // 显示加载状态
    dialog.innerHTML = `
      <div style="text-align: center; padding: 40px;">
        <div style="font-size: 18px; color: #3b82f6; margin-bottom: 10px;">正在收集决策路径数据...</div>
        <div style="width: 40px; height: 40px; border: 4px solid #ddd; border-top: 4px solid #3b82f6; border-radius: 50%; margin: 0 auto; animation: spin 1s linear infinite;"></div>
      </div>
      <style>@keyframes spin { 0% { transform: rotate(0deg); } 100% { transform: rotate(360deg); } }</style>
    `;
    
    // 创建背景遮罩
    const overlay = document.createElement('div');
    overlay.style.cssText = `
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: rgba(0,0,0,0.5);
      z-index: 9998;
    `;
    
    // 添加到DOM
    document.body.appendChild(overlay);
    document.body.appendChild(dialog);
    
    // 点击遮罩关闭弹窗
    overlay.addEventListener('click', () => {
      document.body.removeChild(dialog);
      document.body.removeChild(overlay);
    });

    // 使用新方法直接获取完整的决策路径和Summary数据
    try {
      // 更新加载信息
      dialog.innerHTML = `
        <div style="text-align: center; padding: 40px;">
          <div style="font-size: 18px; color: #3b82f6; margin-bottom: 10px;">正在直接收集完整决策路径数据...</div>
          <div style="width: 40px; height: 40px; border: 4px solid #ddd; border-top: 4px solid #3b82f6; border-radius: 50%; margin: 0 auto; animation: spin 1s linear infinite;"></div>
          <div style="margin-top: 10px; color: #6b7280; font-size: 14px;">这可能需要几秒钟时间</div>
        </div>
      `;
      
      // 获取完整数据
      const completeData = await getDirectDecisionPathWithSummary();
      
      if (!completeData) {
        throw new Error("无法获取决策路径数据");
      }
      
      // 创建用于展示的数据副本，将sortedHands替换为hands以便展示排序后的手牌
      const displayData = JSON.parse(JSON.stringify(completeData));
      
      // 遍历所有节点，用排序后的手牌替换原始手牌
      displayData.decisionPath.forEach(spot => {
        if (spot.summaryData) {
          ['oop', 'ip'].forEach(player => {
            if (spot.summaryData[player] && spot.summaryData[player].sortedHands) {
              // 用排序后的手牌替换原始手牌
              spot.summaryData[player].hands = spot.summaryData[player].sortedHands;
              // 删除多余属性，避免重复数据
              delete spot.summaryData[player].sortedHands;
            }
          });
        }
      });
      
      // 格式化为JSON字符串
      const jsonFormat = JSON.stringify(displayData, null, 2);
      
      // 更新弹窗内容
      dialog.innerHTML = '';
      
      // 创建弹窗标题
      const header = document.createElement('div');
      header.style.cssText = `
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 15px;
        border-bottom: 1px solid #eee;
        padding-bottom: 10px;
      `;
      header.innerHTML = `
        <h3 style="margin: 0; font-size: 18px; font-weight: 600;">Enhanced Decision Path with Summary Data</h3>
        <div style="display: flex; align-items: center;">
          <button id="toggleView" style="margin-right: 10px; padding: 4px 8px; background: #e0f2fe; border: 1px solid #bae6fd; border-radius: 4px; cursor: pointer; font-size: 14px;">
            Visual View
          </button>
          <button id="closeEnhancedPathDialog" style="background: none; border: none; cursor: pointer; font-size: 20px;">×</button>
        </div>
      `;
      
      // 创建JSON内容显示区域
      const content = document.createElement('pre');
      content.style.cssText = `
        flex: 1;
        overflow: auto;
        background: #f8f9fa;
        padding: 15px;
        border-radius: 4px;
        margin-bottom: 15px;
        font-size: 13px;
        white-space: pre-wrap;
        word-break: break-all;
      `;
      content.textContent = jsonFormat;
      
      // 创建可视化视图区域(初始隐藏)
      const visualView = document.createElement('div');
      visualView.style.cssText = `
        flex: 1;
        overflow: auto;
        background: white;
        padding: 15px;
        border-radius: 4px;
        margin-bottom: 15px;
        display: none;
      `;
      
      // 填充可视化视图
      try {
        visualView.innerHTML = generateVisualDecisionPath(completeData);
      } catch (error) {
        console.error("Error generating visual view:", error);
        visualView.innerHTML = `
          <div class="p-4 bg-red-50 text-red-800 rounded">
            <p>Error generating visual view: ${error.message}</p>
          </div>
        `;
      }
      
      // 创建按钮区域
      const actions = document.createElement('div');
      actions.style.cssText = `
        display: flex;
        justify-content: flex-end;
        gap: 10px;
      `;
      
      // 复制按钮
      const copyJsonBtn = document.createElement('button');
      copyJsonBtn.style.cssText = `
        padding: 8px 12px;
        background: #f9fafb;
        border: 1px solid #e5e7eb;
        border-radius: 4px;
        cursor: pointer;
        font-size: 14px;
      `;
      copyJsonBtn.textContent = "Copy JSON";
      copyJsonBtn.addEventListener('click', async () => {
        try {
          await navigator.clipboard.writeText(content.textContent || "");
          copyJsonBtn.textContent = "✓ Copied";
          setTimeout(() => {
            copyJsonBtn.textContent = "Copy JSON";
          }, 2000);
        } catch (err) {
          console.error("Copy failed:", err);
          copyJsonBtn.textContent = "Copy failed";
          setTimeout(() => {
            copyJsonBtn.textContent = "Copy JSON";
          }, 2000);
        }
      });
      
      // 下载按钮
      const downloadJsonBtn = document.createElement('button');
      downloadJsonBtn.style.cssText = `
        padding: 8px 12px;
        background: #f3f4f6;
        border: 1px solid #e5e7eb;
        border-radius: 4px;
        cursor: pointer;
        font-size: 14px;
      `;
      downloadJsonBtn.textContent = "Download JSON";
      downloadJsonBtn.addEventListener('click', async () => {
        try {
          const filePath = await save({
            defaultPath: `enhanced-decision-path-${new Date().toISOString().replace(/[:.]/g, '-')}.json`,
            filters: [{ name: "JSON Files", extensions: ["json"] }],
          });
          
          if (filePath) {
            await writeTextFile(filePath, content.textContent || "");
            downloadJsonBtn.textContent = "✓ Downloaded";
            setTimeout(() => {
              downloadJsonBtn.textContent = "Download JSON";
            }, 2000);
          }
        } catch (err) {
          console.error("Download failed:", err);
          downloadJsonBtn.textContent = "Download failed";
          setTimeout(() => {
            downloadJsonBtn.textContent = "Download JSON";
          }, 2000);
        }
      });
      
      // 组装弹窗
      actions.appendChild(copyJsonBtn);
      actions.appendChild(downloadJsonBtn);
      dialog.appendChild(header);
      dialog.appendChild(content);
      dialog.appendChild(visualView);
      dialog.appendChild(actions);
      
      // 复制到剪贴板
      try {
        await navigator.clipboard.writeText(jsonFormat);
        console.log("Data copied to clipboard");
      } catch (error) {
        console.warn("Failed to copy data to clipboard:", error);
      }
      
      // 添加视图切换功能
      document.getElementById('toggleView')?.addEventListener('click', () => {
        if (content.style.display === 'none') {
          content.style.display = 'block';
          visualView.style.display = 'none';
          document.getElementById('toggleView').textContent = 'Visual View';
        } else {
          content.style.display = 'none';
          visualView.style.display = 'block';
          document.getElementById('toggleView').textContent = 'JSON View';
        }
      });
      
      // 添加关闭事件
      document.getElementById('closeEnhancedPathDialog')?.addEventListener('click', () => {
        document.body.removeChild(dialog);
        document.body.removeChild(overlay);
      });
      
    } catch (error) {
      console.error("Error collecting path data with summary:", error);
      dialog.innerHTML = `
        <div style="text-align: center; padding: 40px;">
          <div style="font-size: 18px; color: #ef4444; margin-bottom: 10px;">
            获取决策路径数据失败
          </div>
          <div style="color: #6b7280;">${error.message || '未知错误'}</div>
          <button id="closeErrorBtn" style="margin-top: 20px; padding: 8px 16px; background: #f3f4f6; border: 1px solid #e5e7eb; border-radius: 4px; cursor: pointer;">
            关闭
          </button>
        </div>
      `;
      
      document.getElementById('closeErrorBtn')?.addEventListener('click', () => {
        document.body.removeChild(dialog);
        document.body.removeChild(overlay);
      });
    }
  } catch (err) {
    // 全局错误处理
    console.error("决策路径处理关键错误:", err);
    alert(`无法显示增强决策路径数据: ${err.message}`);
  }
};

// 直接获取完整决策路径和Summary数据的新方法
const getDirectDecisionPathWithSummary = async () => {
  // 获取决策路径基础数据
  console.log("直接请求决策路径数据...");
  eventBus.emit('request-decision-path-data');
  
  let pathData = null;
  
  // 等待ResultNav组件的响应，设置1秒超时
  try {
    pathData = await new Promise((resolve, reject) => {
      const handler = (data) => {
        console.log("收到决策路径数据响应");
        eventBus.off('decision-path-data', handler);
        if (data) {
          resolve(data);
        } else {
          reject(new Error("从事件总线接收到空数据"));
        }
      };
      
      eventBus.on('decision-path-data', handler);
      
      // 添加超时处理
      setTimeout(() => {
        console.log("决策路径数据响应超时");
        eventBus.off('decision-path-data', handler);
        
        // 尝试从props中获取备用数据
        const fallbackData = props.selectedSpot ? {
          decisionPath: [props.selectedSpot],
          currentPosition: {
            selectedSpotIndex: 0
          },
          board: props.selectedSpot.board || []
        } : null;
        
        if (fallbackData) {
          console.log("使用props中的备用数据");
          resolve(fallbackData);
        } else {
          reject(new Error("决策路径数据获取超时"));
        }
      }, 1000);
    });
  } catch (error) {
    console.error("获取决策路径数据错误:", error);
    throw error;
  }
  
  if (!pathData) {
    console.error("未收到路径数据");
    throw new Error("未能获取决策路径数据");
  }
  
  // 处理决策路径数据
  console.log("处理决策路径数据...");
  const optimizedData = optimizePathData(pathData);
  
  if (!optimizedData || !optimizedData.decisionPath || optimizedData.decisionPath.length === 0) {
    console.error("路径数据优化失败");
    throw new Error("决策路径数据处理失败");
  }
  
  // 直接从store或API中获取Summary数据
  console.log("从数据源获取Summary数据...");
  
  // 卡牌转换函数
  const convertCardToReadable = (cardId) => {
    if (cardId === undefined || cardId === null) return null;
    try {
      // 一般卡牌ID格式: 高8位是第一张牌，低8位是第二张牌
      const card1 = cardId & 0xff;
      const card2 = cardId >>> 8;
      
      // 调用cardStr函数转换(如果可用)
      if (typeof cardStr === 'function') {
        return cardStr(card2) + cardStr(card1);
      }
      
      // 备用转换方法
      const ranks = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
      const suits = ['c', 'd', 'h', 's'];
      
      const rank1 = ranks[Math.floor(card1 / 4)];
      const suit1 = suits[card1 % 4];
      const rank2 = ranks[Math.floor(card2 / 4)];
      const suit2 = suits[card2 % 4];
      
      return rank2 + suit2 + rank1 + suit1;
    } catch (e) {
      console.warn(`卡牌ID ${cardId} 转换失败:`, e);
      return `Unknown(${cardId})`;
    }
  };
  
  // 从DOM表格中提取Summary数据
  const extractSummaryFromTable = () => {
    try {
      // 查找表格，首先找到Summary字段
      const summaryElements = document.querySelectorAll('.text-lg.font-semibold');
      let summaryTable = null;
      
      for (const element of summaryElements) {
        if (element.textContent.includes('Summary')) {
          // 找到了Summary标题，现在寻找附近的表格
          let current = element;
          while (current && !summaryTable) {
            current = current.parentElement;
            if (current) {
              const table = current.querySelector('table');
              if (table) {
                summaryTable = table;
                break;
              }
            }
          }
          break;
        }
      }
      
      if (!summaryTable) {
        console.warn("无法在DOM中找到Summary表格");
        return null;
      }
      
      // 提取表格中的数据
      const rows = summaryTable.querySelectorAll('tbody tr');
      const summaryData = {
        equity: 0,
        ev: 0
      };
      
      // 遍历行找到EQ和EV值
      rows.forEach(row => {
        const cells = row.querySelectorAll('td');
        if (cells.length >= 2) {
          const label = cells[0].textContent.trim().toLowerCase();
          const value = parseFloat(cells[1].textContent.trim());
          
          if (label.includes('eq') && !isNaN(value)) {
            summaryData.equity = value;
          } else if (label.includes('ev') && !isNaN(value)) {
            summaryData.ev = value;
          }
        }
      });
      
      console.log("从DOM表格提取的Summary数据:", summaryData);
      return summaryData;
    } catch (error) {
      console.error("从DOM中提取Summary数据时出错:", error);
      return null;
    }
  };
  
  // 创建从所有结果数据中提取完整表格数据的函数
  const extractFullTableData = (player) => {
    const playerIndex = player === 'oop' ? 0 : 1;
    
    try {
      if (!props.results || props.results.isEmpty) {
        console.warn(`Player ${player} 没有可用的结果数据`);
        return null;
      }
      
      // 记录诊断信息
      console.log(`提取 ${player} 数据，displayPlayer: ${props.displayPlayer}, displayPlayerIndex: ${props.displayPlayerIndex}`);
      console.log(`props.results.equity 结构:`, props.results.equity);
      console.log(`props.results.ev 结构:`, props.results.ev);
      
      // 检查resultsSorted是否有值，这是表格实际显示的数据
      if (resultsSorted.value) {
        console.log(`resultsSorted 包含 ${resultsSorted.value.length} 行数据`);
        if (resultsSorted.value.length > 0) {
          console.log(`第一行数据: `, resultsSorted.value[0]);
        }
      }
      
      // 尝试从DOM中获取Summary数据（更准确反映实际表格值）
      const tableSummary = extractSummaryFromTable();
      
      // 如果从DOM获取不到，就用props数据，但要确保索引正确
      // 使用当前显示的玩家索引，而不是固定的playerIndex
      const actualPlayerIndex = props.displayPlayerIndex !== undefined ? props.displayPlayerIndex : playerIndex;
      
      // 创建全新的数据结构，以手牌为中心组织数据
      const tableData = {
        // 手牌数据将成为主要内容
        hands: [],
        // 基本统计数据 - 优先使用DOM中提取的数据
        summary: tableSummary || {
          equity: props.results.equity?.[actualPlayerIndex]?.[0] || 0,
          ev: props.results.ev?.[actualPlayerIndex]?.[0] || 0
        },
        // 动作相关数据
        actions: {}
      };
      
      console.log(`${player}的Summary数据:`, tableData.summary);
      
      // 提取动作信息(如果有)
      if (props.selectedSpot && props.selectedSpot.actions) {
        props.selectedSpot.actions.forEach(action => {
          tableData.actions[action.name] = {
            amount: action.amount,
            isSelected: action.isSelected || false
          };
        });
      }
      
      // 获取手牌数据并重新组织为清晰的结构
      // 首先尝试从resultsSorted获取（表格实际显示的数据）
      let handSourceData = [];
      
      if (resultsSorted.value && resultsSorted.value.length > 0) {
        // 从排序后的表格数据获取
        handSourceData = resultsSorted.value;
        console.log(`使用resultsSorted作为手牌数据源，包含${handSourceData.length}行`);
      } else if (props.cards && props.cards[playerIndex]) {
        // 从原始卡牌数据获取
        const cards = props.cards[playerIndex];
        handSourceData = Array.from({ length: cards.length }, (_, i) => {
          return [
            cards[i],
            props.results.weights?.[playerIndex]?.[i] || 0,
            props.results.normalizers?.[playerIndex]?.[i] || 0,
            props.results.equity?.[playerIndex]?.[i] || 0,
            props.results.ev?.[playerIndex]?.[i] || 0
          ];
        });
        console.log(`使用props.cards作为手牌数据源，包含${handSourceData.length}行`);
      }
      
      // 确保我们有手牌数据
      if (handSourceData.length > 0) {
        // 遍历所有手牌，不再过滤权重为0的手牌
        handSourceData.forEach((row, index) => {
          try {
            const cardId = row[0];
            // 转换为可读格式
            const readableCard = convertCardToReadable(cardId);
            
            if (readableCard) {
              // 创建手牌对象
              const handData = {
                hand: readableCard,
                equity: row[INDEX_EQUITY] || 0,
                ev: row[INDEX_EV] || 0
              };
              
              // 添加策略数据(如果有)
              if (props.results.strategy && props.selectedSpot && props.selectedSpot.actions) {
                const strategies = {};
                
                // 为每个动作提取策略概率
                props.selectedSpot.actions.forEach((action, actionIndex) => {
                  const strategyIndex = actionIndex * handSourceData.length + index;
                  if (props.results.strategy[strategyIndex] !== undefined) {
                    strategies[action.name] = {
                      probability: props.results.strategy[strategyIndex],
                      amount: action.amount
                    };
                  }
                });
                
                // 只有当有策略数据时才添加
                if (Object.keys(strategies).length > 0) {
                  handData.strategies = strategies;
                }
              }
              
              // 添加到手牌列表
              tableData.hands.push(handData);
            }
          } catch (e) {
            console.warn(`处理手牌索引 ${index} 失败:`, e);
          }
        });
        
        // 按照权益值(EQ)排序手牌，让用户更容易查看
        tableData.hands.sort((a, b) => b.equity - a.equity);
        
        // 限制手牌数量，避免数据过大
        if (tableData.hands.length > 100) {
          console.log(`限制手牌数量从 ${tableData.hands.length} 到 100`);
          tableData.hands = tableData.hands.slice(0, 100);
        }
      } else {
        console.warn(`没有可用的手牌数据来源`);
      }
      
      console.log(`最终提取了 ${tableData.hands.length} 张手牌数据`);
      return tableData;
    } catch (error) {
      console.error(`提取 ${player} 表格数据时出错:`, error);
      return null;
    }
  };
  
  // 所有节点的当前状态数据
  const nodesState = {};
  
  // 为每个节点获取状态数据
  for (let i = 0; i < optimizedData.decisionPath.length; i++) {
    const spot = optimizedData.decisionPath[i];
    
    // 收集当前节点的状态
    if (spot.type === 'player' || spot.type === 'terminal') {
      try {
        // 为每个视角获取数据
        const oopData = extractFullTableData('oop');
        const ipData = extractFullTableData('ip');
        
        if (oopData || ipData) {
          nodesState[spot.index] = {
            oop: oopData,
            ip: ipData,
            spotType: spot.type,
            player: spot.player,
            timestamp: new Date().toISOString()
          };
        }
      } catch (error) {
        console.warn(`获取节点 ${spot.index} 状态数据时出错:`, error);
      }
    }
  }
  
  // 构建增强版决策路径数据
  console.log("构建增强版决策路径数据...");
  const enhancedPath = optimizedData.decisionPath.map(spot => {
    // 复制原始节点数据
    const enhancedSpot = { ...spot };
    
    // 添加Summary数据(如果存在)
    if ((spot.type === 'player' || spot.type === 'terminal') && nodesState[spot.index]) {
      enhancedSpot.summaryData = nodesState[spot.index];
    }
    
    return enhancedSpot;
  });
  
  // 对每个节点中的手牌数据进行排序处理（但不修改原始数据）
  enhancedPath.forEach(spot => {
    if (spot.summaryData) {
      // 处理OOP和IP的手牌数据
      ['oop', 'ip'].forEach(player => {
        if (spot.summaryData[player] && spot.summaryData[player].hands && spot.summaryData[player].hands.length > 0) {
          // 创建手牌排序函数 - 按照牌面大小从高到低排序(AA到22)
          const sortHandsByRank = (a, b) => {
            // 提取牌面信息（对于口袋对，两张牌的点数相同，取第一张即可）
            const rankA = a.hand.substring(0, 1); // 第一张牌的点数
            const rankB = b.hand.substring(0, 1); // 第一张牌的点数
            
            // 转换牌面点数为数值大小以便排序
            const getRankValue = (rank) => {
              const rankValues = {'A': 14, 'K': 13, 'Q': 12, 'J': 11, 'T': 10};
              return rankValues[rank] || parseInt(rank);
            };
            
            const valueA = getRankValue(rankA);
            const valueB = getRankValue(rankB);
            
            // 比较大小并返回排序结果(从大到小)
            return valueB - valueA;
          };
          
          // 创建按牌面大小排序的手牌副本
          spot.summaryData[player].sortedHands = [...spot.summaryData[player].hands].sort(sortHandsByRank);
        }
      });
    }
  });
  
  // 创建最终的增强数据
  const enhancedData = {
    ...optimizedData,
    decisionPath: enhancedPath,
    dataStats: {
      totalNodes: optimizedData.decisionPath.length,
      nodesWithSummary: Object.keys(nodesState).length,
      activePlayer: props.displayPlayer,
      timestamp: new Date().toISOString(),
      dataSource: 'direct-extraction'
    }
  };
  
  return enhancedData;
};

// 增强决策路径数据，添加Summary数据
const enhancePathDataWithSummary = async (data) => {
  if (!data || !data.spots) return optimizePathData(data);
  
  // 首先获取优化后的基础路径数据
  const optimized = optimizePathData(data);
  
  // 为每个决策点添加Summary数据
  const enhanced = {
    ...optimized,
    decisionPath: await Promise.all(optimized.decisionPath.map(async (spot, index) => {
      // 复制原始spot数据
      const enhancedSpot = { ...spot };
      
      // 对于玩家行动点和终端节点，尝试获取Summary数据
      if (spot.type === 'player' || spot.type === 'terminal') {
        try {
          // 请求该节点的OOP视角Summary数据
          eventBus.emit('request-spot-summary', { 
            spotIndex: data.spots[index].index,
            player: 'oop'
          });
          
          // 等待Summary数据响应
          const oopSummary = await new Promise(resolve => {
            const handler = (summaryData) => {
              if (summaryData && summaryData.spotIndex === data.spots[index].index) {
                eventBus.off('spot-summary-data', handler);
                resolve(summaryData.summary);
              }
            };
            
            eventBus.on('spot-summary-data', handler);
            
            // 添加超时处理
            setTimeout(() => {
              eventBus.off('spot-summary-data', handler);
              resolve(null);
            }, 500); // 较短的超时时间以避免长时间等待
          });
          
          // 请求该节点的IP视角Summary数据
          eventBus.emit('request-spot-summary', { 
            spotIndex: data.spots[index].index,
            player: 'ip'
          });
          
          // 等待Summary数据响应
          const ipSummary = await new Promise(resolve => {
            const handler = (summaryData) => {
              if (summaryData && summaryData.spotIndex === data.spots[index].index) {
                eventBus.off('spot-summary-data', handler);
                resolve(summaryData.summary);
              }
            };
            
            eventBus.on('spot-summary-data', handler);
            
            // 添加超时处理
            setTimeout(() => {
              eventBus.off('spot-summary-data', handler);
              resolve(null);
            }, 500);
          });
          
          // 添加Summary数据到增强spot
          enhancedSpot.summaryData = {
            oop: oopSummary ? processSummaryData(oopSummary) : null,
            ip: ipSummary ? processSummaryData(ipSummary) : null
          };
        } catch (error) {
          console.error(`Error getting summary data for spot ${index}:`, error);
          enhancedSpot.summaryData = { error: error.message };
        }
      }
      
      return enhancedSpot;
    }))
  };
  
  // 如果任何节点有获取到的Summary数据，添加数据统计信息
  const spotWithSummary = enhanced.decisionPath.find(spot => spot.summaryData && 
    (spot.summaryData.oop || spot.summaryData.ip));
  
  if (spotWithSummary) {
    enhanced.dataStats = {
      nodesWithSummaryData: enhanced.decisionPath.filter(spot => 
        spot.summaryData && (spot.summaryData.oop || spot.summaryData.ip)).length,
      totalNodes: enhanced.decisionPath.length,
      activePlayer: props.displayPlayer,
      timestamp: new Date().toISOString()
    };
  }
  
  return enhanced;
};

// 处理Summary数据，选择性保留关键信息，避免数据过大
const processSummaryData = (summary) => {
  if (!summary) return null;
  
  // 提取重要的Summary字段
  return {
    weight: summary[INDEX_WEIGHT] || 0,
    normalizer: summary[INDEX_NORMALIZER] || 0,
    equity: summary[INDEX_EQUITY] || 0,
    ev: summary[INDEX_EV] || 0,
    eqr: summary[INDEX_EQR] || 0,
    // 获取策略数据（如果存在）
    strategy: summary.slice(INDEX_STRATEGY_BASE).filter((_, i) => i % 2 === 0)
  };
};

// 生成可视化决策路径HTML
const generateVisualDecisionPath = (enhancedData) => {
  if (!enhancedData || !enhancedData.decisionPath) {
    return '<div class="text-red-500">No valid decision path data available</div>';
  }
  
  // 对所有节点中的手牌进行排序处理（为可视化展示准备）
  enhancedData.decisionPath.forEach(spot => {
    if (spot.summaryData) {
      // 处理OOP和IP的手牌数据
      ['oop', 'ip'].forEach(player => {
        if (spot.summaryData[player] && spot.summaryData[player].hands && spot.summaryData[player].hands.length > 0) {
          // 如果没有已排序的手牌，则现在创建
          if (!spot.summaryData[player].sortedHands) {
            // 创建手牌排序函数 - 按照牌面大小从高到低排序(AA到22)
            const sortHandsByRank = (a, b) => {
              // 提取牌面信息（对于口袋对，两张牌的点数相同，取第一张即可）
              const rankA = a.hand.substring(0, 1); // 第一张牌的点数
              const rankB = b.hand.substring(0, 1); // 第一张牌的点数
              
              // 转换牌面点数为数值大小以便排序
              const getRankValue = (rank) => {
                const rankValues = {'A': 14, 'K': 13, 'Q': 12, 'J': 11, 'T': 10};
                return rankValues[rank] || parseInt(rank);
              };
              
              const valueA = getRankValue(rankA);
              const valueB = getRankValue(rankB);
              
              // 比较大小并返回排序结果(从大到小)
              return valueB - valueA;
            };
            
            // 创建按牌面大小排序的手牌副本
            spot.summaryData[player].sortedHands = [...spot.summaryData[player].hands].sort(sortHandsByRank);
          }
        }
      });
    }
  });
  
  let html = `
    <div class="flex flex-col space-y-6">
      <div class="bg-blue-50 p-4 rounded-lg">
        <h2 class="text-lg font-semibold text-blue-800 mb-2">Path Summary</h2>
        <p class="text-gray-700">${enhancedData.pathSummary || 'No path summary available'}</p>
        
        <div class="mt-3 grid grid-cols-2 gap-4">
          <div>
            <span class="text-sm font-medium text-gray-500">Board:</span>
            <span class="ml-2">${enhancedData.board ? enhancedData.board.join(' ') : 'N/A'}</span>
          </div>
          <div>
            <span class="text-sm font-medium text-gray-500">Pot Size:</span>
            <span class="ml-2">${enhancedData.bettingInfo?.potSize || 'N/A'}</span>
          </div>
        </div>
      </div>
      
      <div class="flex flex-col">
        <h2 class="text-lg font-semibold text-gray-800 mb-3">Decision Path Nodes</h2>
        <div class="space-y-4">
  `;
  
  // 为每个节点生成HTML
  enhancedData.decisionPath.forEach((spot, index) => {
    let spotHtml = `
      <div class="border border-gray-200 rounded-lg overflow-hidden">
        <div class="bg-gray-50 px-4 py-2 flex justify-between items-center border-b border-gray-200">
          <div class="font-medium">${getSpotTypeLabel(spot)} (${index})</div>
          <div class="text-sm text-gray-500">${spot.player}</div>
        </div>
        <div class="p-4">
    `;
    
    // 根据节点类型添加不同的内容
    if (spot.type === 'root') {
      spotHtml += `
        <div class="grid grid-cols-2 gap-3">
          <div><span class="text-gray-600">Pot:</span> ${spot.pot}</div>
          <div><span class="text-gray-600">Stack:</span> ${spot.stack}</div>
          <div><span class="text-gray-600">Board:</span> ${spot.board ? spot.board.join(' ') : 'N/A'}</div>
        </div>
      `;
    } else if (spot.type === 'player') {
      spotHtml += `
        <div class="mb-3">
          <span class="text-gray-600">Action:</span> 
          <span class="px-2 py-1 bg-blue-100 text-blue-800 rounded">
            ${spot.action ? `${spot.action.name} ${spot.action.amount !== "0" ? spot.action.amount : ""}` : 'N/A'}
          </span>
        </div>
      `;
      
      // 添加Summary数据(如果存在)
      if (spot.summaryData && (spot.summaryData.oop || spot.summaryData.ip)) {
        spotHtml += generateSummaryDataHTML(spot.summaryData);
      }
    } else if (spot.type === 'chance') {
      spotHtml += `
        <div class="grid grid-cols-2 gap-3">
          <div><span class="text-gray-600">Card:</span> 
            <span class="px-2 py-1 ${getCardColorClass(spot.cardDisplay)} rounded font-medium">${spot.cardDisplay || 'N/A'}</span>
          </div>
          <div><span class="text-gray-600">Pot:</span> ${spot.pot}</div>
        </div>
      `;
    } else if (spot.type === 'terminal') {
      spotHtml += `
        <div class="grid grid-cols-2 gap-3 mb-3">
          <div><span class="text-gray-600">Pot:</span> ${spot.pot}</div>
          <div><span class="text-gray-600">Equity OOP:</span> ${spot.equityPercentage?.oop || 'N/A'}</div>
          <div><span class="text-gray-600">Equity IP:</span> ${spot.equityPercentage?.ip || 'N/A'}</div>
        </div>
      `;
      
      // 添加Summary数据(如果存在)
      if (spot.summaryData && (spot.summaryData.oop || spot.summaryData.ip)) {
        spotHtml += generateSummaryDataHTML(spot.summaryData);
      }
    }
    
    spotHtml += `
        </div>
      </div>
    `;
    
    html += spotHtml;
  });
  
  html += `
        </div>
      </div>
    </div>
  `;
  
  return html;
};

// 获取扑克牌的颜色类
const getCardColorClass = (card) => {
  if (!card || card.length < 2) return 'bg-gray-100 text-gray-800';
  
  const suit = card[1];
  switch(suit) {
    case 'h': // hearts
    case 'd': // diamonds
      return 'bg-red-100 text-red-700';
    case 's': // spades
    case 'c': // clubs
      return 'bg-gray-700 text-white';
    default:
      return 'bg-gray-100 text-gray-800';
  }
};

// 获取节点类型标签
const getSpotTypeLabel = (spot) => {
  switch(spot.type) {
    case 'root': return 'Initial State';
    case 'player': return 'Player Action';
    case 'chance': return 'Chance Node';
    case 'terminal': return 'Terminal Node';
    default: return spot.type;
  }
};

// 生成Summary数据的HTML
const generateSummaryDataHTML = (summaryData) => {
  let html = `<div class="mt-3 border-t border-gray-200 pt-3">
    <h3 class="text-sm font-semibold text-gray-700 mb-2">Summary Data</h3>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
  `;
  
  if (summaryData.oop) {
    html += `
      <div>
        <h4 class="text-xs font-medium text-gray-500 mb-1">OOP Summary</h4>
        <div class="grid grid-cols-2 gap-2 text-sm">
          <div><span class="text-gray-600">EQ:</span> ${(summaryData.oop.equity * 100).toFixed(2)}%</div>
          <div><span class="text-gray-600">EV:</span> ${summaryData.oop.ev.toFixed(2)}</div>
          <div><span class="text-gray-600">EQR:</span> ${isNaN(summaryData.oop.eqr) ? 'N/A' : summaryData.oop.eqr.toFixed(2)}</div>
          <div><span class="text-gray-600">Weight:</span> ${summaryData.oop.weight.toFixed(2)}</div>
        </div>
      </div>
    `;
  }
  
  if (summaryData.ip) {
    html += `
      <div>
        <h4 class="text-xs font-medium text-gray-500 mb-1">IP Summary</h4>
        <div class="grid grid-cols-2 gap-2 text-sm">
          <div><span class="text-gray-600">EQ:</span> ${(summaryData.ip.equity * 100).toFixed(2)}%</div>
          <div><span class="text-gray-600">EV:</span> ${summaryData.ip.ev.toFixed(2)}</div>
          <div><span class="text-gray-600">EQR:</span> ${isNaN(summaryData.ip.eqr) ? 'N/A' : summaryData.ip.eqr.toFixed(2)}</div>
          <div><span class="text-gray-600">Weight:</span> ${summaryData.ip.weight.toFixed(2)}</div>
        </div>
      </div>
    `;
  }
  
  html += `
    </div>
  </div>`;
  
  return html;
};

// ... existing code ...

// 显示决策树信息弹窗 - 调试版本（仅输出日志和基本信息，不渲染树）
async function showDecisionTree() {
  try {
    // 显示简单的调试弹窗
    const debugDialog = document.createElement('div');
    debugDialog.style.cssText = `
      position: fixed;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: 90%;
      max-width: 1200px;
      max-height: 90vh;
      background: white;
      border-radius: 8px;
      box-shadow: 0 4px 15px rgba(0,0,0,0.2);
      z-index: 9999;
      padding: 20px;
      overflow-y: auto;
      font-family: monospace;
    `;
    
    // 创建弹窗标题
    const header = document.createElement('div');
    header.style.cssText = `
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 15px;
      border-bottom: 1px solid #eee;
      padding-bottom: 10px;
    `;
    header.innerHTML = `
      <h3 style="margin: 0; font-size: 18px; font-weight: 600;">GTO Decision Tree - 调试模式</h3>
      <button id="closeDebugDialog" style="background: none; border: none; cursor: pointer; font-size: 20px;">×</button>
    `;
    
    // 创建日志内容区域
    const logArea = document.createElement('div');
    logArea.id = 'debug-log-area';
    logArea.style.cssText = `
      background: #f5f5f5;
      padding: 10px;
      border-radius: 4px;
      margin-bottom: 15px;
      white-space: pre-wrap;
      font-size: 12px;
      line-height: 1.5;
      max-height: 300px;
      overflow-y: auto;
    `;
    logArea.textContent = "调试日志：\n";
    
    const addLog = (message, type = 'info') => {
      const timestamp = new Date().toLocaleTimeString();
      const typeMarker = type === 'error' ? '[错误]' : 
                        type === 'warning' ? '[警告]' : 
                        type === 'success' ? '[成功]' : '[信息]';
      
      const color = type === 'error' ? '#ef4444' : 
                   type === 'warning' ? '#f59e0b' : 
                   type === 'success' ? '#10b981' : '#6b7280';
                
      logArea.innerHTML += `<span style="color: ${color}">[${timestamp}] ${typeMarker} ${message}</span>\n`;
      logArea.scrollTop = logArea.scrollHeight;
      console.log(`[调试] ${typeMarker} ${message}`);
    };
    
    // 将弹窗添加到文档
    debugDialog.appendChild(header);
    debugDialog.appendChild(logArea);
    document.body.appendChild(debugDialog);
    
    // 关闭弹窗按钮事件
    document.getElementById('closeDebugDialog').addEventListener('click', () => {
      document.body.removeChild(debugDialog);
      if (backdrop) document.body.removeChild(backdrop);
    });
    
    // 添加遮罩层
    const backdrop = document.createElement('div');
    backdrop.style.cssText = `
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: rgba(0,0,0,0.5);
      z-index: 9998;
    `;
    backdrop.addEventListener('click', () => {
      document.body.removeChild(debugDialog);
      document.body.removeChild(backdrop);
    });
    document.body.appendChild(backdrop);
    
    // 获取Store对象
    const store = useStore();
    const config = useConfigStore();
    
    // === 统一状态诊断部分 ===
    // 记录状态信息到日志
    addLog(`当前系统状态信息`, 'info');
    addLog(`求解器状态: ${store.isSolverRunning ? '运行中' : (store.isSolverPrepared ? '就绪' : '未准备')}`, 'info');
    addLog(`历史长度: ${store.history ? store.history.length : 0}, 历史内容: ${JSON.stringify(store.history || [])}`, 'info');
    addLog(`路径长度: ${store.path ? store.path.length : 0}, 选中索引: ${store.selectedSpotIndex}`, 'info');
    addLog(`当前模式: ${store.spotMode || 'undefined'}, 状态: ${store.state || 'undefined'}`, 'info');
    
    if (store.path && store.path.length > 0) {
      addLog(`路径包含 ${store.path.length} 个点`, 'info');
      
      // 检查路径最后一个点是否与selectedSpotIndex匹配
      if (store.selectedSpotIndex !== undefined && store.selectedSpotIndex < store.path.length) {
        addLog(`当前选中点: ${JSON.stringify(store.path[store.selectedSpotIndex])}`, 'info');
      } else {
        addLog(`选中索引(${store.selectedSpotIndex})与路径长度(${store.path.length})不匹配`, 'warning');
      }
    } else {
      addLog(`路径为空或未定义`, 'warning');
    }
    
    // 获取完整行动路径按钮
    const getPathBtn = document.createElement('button');
    getPathBtn.textContent = "获取完整行动路径";
    getPathBtn.style.cssText = `
      padding: 10px 20px;
      background: #8b5cf6;
      color: white;
      border: none;
      border-radius: 4px;
      cursor: pointer;
      font-size: 14px;
      font-weight: 500;
      margin-right: 10px;
      box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    `;
    
    // 创建结果显示区域
    const resultArea = document.createElement('div');
    resultArea.style.cssText = `
      margin-top: 15px;
      background: #fff;
      border: 1px solid #ddd;
      border-radius: 4px;
      padding: 10px;
      max-height: 500px;
      overflow-y: auto;
      font-size: 12px;
    `;
    resultArea.innerHTML = "<div style='color: #888; text-align: center; padding: 20px;'>点击上方按钮获取数据...</div>";
    
    // 获取完整行动路径按钮的事件处理
    getPathBtn.onclick = async () => {
      addLog("开始获取行动路径...", 'info');
      resultArea.innerHTML = "<div style='color: #888; text-align: center; padding: 20px;'>加载中...<div class='spinner'></div></div>";
      
      // 设置超时处理，避免长时间卡死
      const timeoutPromise = new Promise((_, reject) => {
        setTimeout(() => reject(new Error("操作超时，请检查日志")), 5000);
      });
      
      try {
        // 尝试使用API获取决策路径
        addLog("调用getCompleteDecisionPath API...", 'info');
        
        // 使用Promise.race实现超时处理
        const pathData = await Promise.race([
          invokes.getCompleteDecisionPath(),
          timeoutPromise
        ]);
        
        // 记录成功信息
        addLog(`获取成功! 路径长度: ${pathData.decision_path.length}`, 'success');
        
        // 只显示关键信息，避免渲染大量JSON造成卡顿
        let info = `<div style="margin-bottom: 10px; color: green; font-weight: bold;">✓ 获取成功</div>`;
        info += `<div style="margin-bottom: 5px;"><b>路径长度:</b> ${pathData.decision_path.length}</div>`;
        
        if (pathData.decision_path.length > 0) {
          info += `<div style="margin-bottom: 5px;"><b>牌面:</b> ${pathData.board.join(' ')}</div>`;
          info += `<div style="margin-bottom: 5px;"><b>底池:</b> 起始=${pathData.betting_info.starting_pot}, 当前=${pathData.betting_info.pot_size}</div>`;
          
          // 显示路径摘要而不是完整JSON
          info += `<div style="margin-bottom: 10px;"><b>路径摘要:</b> ${pathData.path_summary}</div>`;
          
          // 添加节点类型统计
          const nodeTypes = pathData.decision_path.reduce((acc, node) => {
            const type = typeof node === 'object' && node !== null ? node.type : 'unknown';
            acc[type] = (acc[type] || 0) + 1;
            return acc;
          }, {});
          
          info += `<div style="margin-bottom: 10px;"><b>节点统计:</b> `;
          Object.entries(nodeTypes).forEach(([type, count]) => {
            info += `${type}: ${count} | `;
          });
          info = info.slice(0, -3) + '</div>';
          
          // 显示所有节点的简要信息
          info += `<div style="margin-top: 10px;"><b>所有${pathData.decision_path.length}个节点:</b></div>`;
          info += `<pre style="background: #f8f8f8; padding: 10px; overflow: auto; max-height: 400px; margin-top: 5px; white-space: pre-wrap; word-break: break-all;">`;
          
          // 默认显示所有节点
          for (let i = 0; i < pathData.decision_path.length; i++) {
            const node = pathData.decision_path[i];
            info += `<div style="margin-bottom: 10px; padding-bottom: 5px; border-bottom: 1px dashed #ddd;">`;
            info += `<b>节点 ${i}:</b> ${node.type} (${node.player})\n`;
            info += JSON.stringify(node, null, 2);
            info += `</div>`;
          }
          
          info += `</pre>`;
          
          // 添加收起/展开控制按钮
          info += `<div style="margin-top: 5px; text-align: center;">
            <button id="toggleNodes" style="padding: 5px 10px; background: #f1f5f9; border: 1px solid #e2e8f0; border-radius: 4px; cursor: pointer;">
              收起节点详情
            </button>
          </div>`;
        } else {
          info += `<div style="color: orange; margin: 10px 0;">警告: 决策路径为空</div>`;
        }
        
        resultArea.innerHTML = info;
        
        // 添加收起/展开控制的事件处理
        const toggleBtn = document.getElementById('toggleNodes');
        if (toggleBtn) {
          let isExpanded = true;
          toggleBtn.addEventListener('click', () => {
            const pre = resultArea.querySelector('pre');
            if (!pre) return;
            
            if (isExpanded) {
              // 收起详情，只显示节点标题
              const nodes = pre.querySelectorAll('div');
              for (const node of nodes) {
                const nodeContent = node.innerHTML.split('\n');
                if (nodeContent.length > 1) {
                  node.setAttribute('data-full-content', node.innerHTML);
                  node.innerHTML = nodeContent[0]; // 只保留标题行
                }
              }
              toggleBtn.textContent = "展开节点详情";
            } else {
              // 恢复完整内容
              const nodes = pre.querySelectorAll('div');
              for (const node of nodes) {
                const fullContent = node.getAttribute('data-full-content');
                if (fullContent) {
                  node.innerHTML = fullContent;
                }
              }
              toggleBtn.textContent = "收起节点详情";
            }
            isExpanded = !isExpanded;
          });
        }
      } catch (err) {
        addLog(`获取失败: ${err.message || err}`, 'error');
        
        // 显示错误信息
        resultArea.innerHTML = `<div style="color: #ef4444; text-align: center; padding: 20px;">
          获取失败: ${err.message || err}
        </div>
        <div style="margin-top: 10px; text-align: center;">
          <button id="retryBtn" class="px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600">重试</button>
        </div>`;
        
        // 添加重试按钮事件处理
        document.getElementById("retryBtn")?.addEventListener("click", getPathBtn.onclick);
      }
    };
    
    // 添加简单的加载动画样式
    const style = document.createElement('style');
    style.textContent = `
      .spinner {
        display: inline-block;
        width: 20px;
        height: 20px;
        border: 3px solid rgba(0, 0, 0, 0.1);
        border-radius: 50%;
        border-top-color: #3b82f6;
        animation: spin 1s ease-in-out infinite;
        margin-top: 10px;
      }
      
      @keyframes spin {
        to { transform: rotate(360deg); }
      }
    `;
    document.head.appendChild(style);
    
    // 添加按钮和结果区域
    const btnContainer = document.createElement('div');
    btnContainer.style.cssText = "display: flex; flex-wrap: wrap; gap: 10px; margin-top: 15px; justify-content: center;";
    btnContainer.appendChild(getPathBtn);
    
    debugDialog.appendChild(btnContainer);
    debugDialog.appendChild(resultArea);
    
    // 记录初始状态信息
    addLog("调试弹窗已打开", 'frontend');
    addLog(`当前时间: ${new Date().toLocaleString()}`, 'info');
    
  } catch (err) {
    console.error("调试对话框错误:", err);
    alert(`调试功能发生错误: ${err.message || err}`);
  }
}

// 生成决策路径摘要文本
function generateDecisionPathSummary(decisionPath, store) {
  try {
    const startingPot = store.treeConfig?.startingPot || 0;
    const stack = store.treeConfig?.effectiveStack || 0;
    const board = store.board || [];
    
    let summary = `Starting with pot ${startingPot}, effective stack ${stack} | Board: `;
    
    // 添加翻牌牌面
    if (board.length >= 3) {
      summary += board.slice(0, 3).join(' ') + ' | ';
    }
    
    // 添加每个决策点的信息
    for (let i = 0; i < decisionPath.length; i++) {
      const spot = decisionPath[i];
      
      if (i === 0 && spot.type === "root") {
        summary += "Flop: ";
        continue;
      }
      
      if (spot.type === "player") {
        summary += `${spot.player.toUpperCase()} ${spot.action.name} | `;
      } else if (spot.type === "chance") {
        if (spot.player === "turn") {
          summary += `Turn: ${spot.cardDisplay} | Turn actions: `;
        } else if (spot.player === "river") {
          summary += `River: ${spot.cardDisplay} | River actions: `;
        }
      } else if (spot.type === "terminal") {
        summary += `Final pot: ${spot.pot} | Equity: OOP ${spot.equityPercentage.oop}, IP ${spot.equityPercentage.ip}`;
      }
    }
    
    return summary;
  } catch (err) {
    console.error("生成路径摘要失败:", err);
    return "无法生成路径摘要";
  }
}

// 根据历史生成预期的节点ID
function generateExpectedNodeId(history) {
  if (!history || history.length === 0) {
    return "root";
  }
  
  return history.map(action => {
    if (action === 0) return "F"; // Fold
    if (action === 1) return "X"; // Check
    if (action === 2) return "C"; // Call
    return `A${action}`; // 其他动作
  }).join("-");
}

defineExpose({
  sortBy,
  exportSummary,
  showEnhancedDecisionPathInfo,
  showDecisionPathInfo,
  showBeforeSolvingInfo,
  showDecisionTree  // 添加这一行
});
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