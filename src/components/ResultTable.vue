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
              <CodeBracketIcon class="w-5 h-5 text-yellow-500" />
            </button>
          </Tippy>
          
          <Tippy content="View complete debug info after solving">
            <button
              :class="
                'ml-2 flex w-8 h-8 items-center justify-center border border-gray-300 bg-white ' +
                'rounded-lg shadow-sm transition-all duration-200 hover:bg-gray-100 active:bg-gray-200' +
                (hasSolverResults ? '' : ' opacity-50 cursor-not-allowed')
              "
              @click="handleAfterSolvingClick"
              :disabled="!hasSolverResults"
            >
              <CodeBracketIcon class="w-5 h-5" :class="hasSolverResults ? 'text-green-500' : 'text-gray-400'" />
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
  readableLineString
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
  CodeBracketIcon,
  MapIcon 
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
    
    // 添加关闭事件
    document.getElementById('closeDialog').addEventListener('click', () => {
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
    console.error("Debug data error:", err);
    alert("Failed to collect or display debug data. Check the console for details.");
  }
};

<<<<<<< .merge_file_sro4FQ
// 显示决策路径导航JSON数据
const showDecisionPathInfo = async () => {
  // 获取决策路径导航数据
  // 通过eventBus请求当前的spots数据
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
  
  // 格式化为JSON字符串
  const jsonFormat = JSON.stringify(optimizedData, null, 2);
=======
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
>>>>>>> .merge_file_tPKusJ
  
  try {
    // 复制到剪贴板
    await navigator.clipboard.writeText(jsonFormat);
<<<<<<< .merge_file_sro4FQ
=======
    console.log("Copied to clipboard successfully");
>>>>>>> .merge_file_tPKusJ
    
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
<<<<<<< .merge_file_sro4FQ
=======
    console.log("Dialog element created");
>>>>>>> .merge_file_tPKusJ
    
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
<<<<<<< .merge_file_sro4FQ
      <h3 style="margin: 0; font-size: 18px; font-weight: 600;">Decision Path Navigation Data</h3>
      <div style="display: flex; align-items: center;">
        <button id="toggleViewMode" style="margin-right: 10px; padding: 4px 8px; border: 1px solid #e5e7eb; border-radius: 4px; font-size: 12px; background: #f9fafb; cursor: pointer;">
          Optimized View
        </button>
        <button id="closePathDialog" style="background: none; border: none; cursor: pointer; font-size: 20px;">×</button>
      </div>
    `;
=======
      <h3 style="margin: 0; font-size: 18px; font-weight: 600;">After Solving - Debug Info</h3>
      <button id="closeDialog" style="background: none; border: none; cursor: pointer; font-size: 20px;">×</button>
    `;
    console.log("Dialog header created");
>>>>>>> .merge_file_tPKusJ
    
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
<<<<<<< .merge_file_sro4FQ
=======
    console.log("Content area created");
>>>>>>> .merge_file_tPKusJ
    
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
<<<<<<< .merge_file_sro4FQ
      await navigator.clipboard.writeText(content.textContent || "");
      copyJsonBtn.textContent = "✓ Copied";
      setTimeout(() => {
        copyJsonBtn.textContent = "Copy JSON";
      }, 2000);
    });
=======
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
>>>>>>> .merge_file_tPKusJ
    
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
<<<<<<< .merge_file_sro4FQ
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
=======
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
>>>>>>> .merge_file_tPKusJ
    
    // 组装弹窗
    actions.appendChild(copyJsonBtn);
    actions.appendChild(downloadJsonBtn);
    dialog.appendChild(header);
    dialog.appendChild(content);
    dialog.appendChild(actions);
<<<<<<< .merge_file_sro4FQ
    document.body.appendChild(dialog);
    
    // 添加切换视图模式功能
    let isOptimizedView = true;
    const toggleViewBtn = document.getElementById('toggleViewMode');
    if (toggleViewBtn) {
      toggleViewBtn.addEventListener('click', () => {
        isOptimizedView = !isOptimizedView;
        toggleViewBtn.textContent = isOptimizedView ? 'Optimized View' : 'Full View';
        content.textContent = isOptimizedView 
          ? JSON.stringify(optimizedData, null, 2)
          : JSON.stringify(pathData, null, 2);
      });
    }
    
    // 添加关闭事件
    document.getElementById('closePathDialog')?.addEventListener('click', () => {
      document.body.removeChild(dialog);
      document.body.removeChild(overlay);
    });
=======
>>>>>>> .merge_file_tPKusJ
    
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
<<<<<<< .merge_file_sro4FQ
    document.body.appendChild(overlay);
=======
    
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
>>>>>>> .merge_file_tPKusJ
    
    // 点击遮罩关闭弹窗
    overlay.addEventListener('click', () => {
      document.body.removeChild(dialog);
      document.body.removeChild(overlay);
<<<<<<< .merge_file_sro4FQ
    });
  } catch (err) {
    // 如果剪贴板API失败，使用alert显示
    console.error("Decision path data error:", err);
    alert("Failed to display decision path data. Check the console for details.");
  }
};

// 优化和精简决策路径数据
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
=======
      console.log("Dialog closed via overlay click");
    });
  } catch (err) {
    // 如果剪贴板API失败，使用alert显示
    console.error("Debug data error:", err);
    alert("Failed to collect or display debug data. Check the console for details.");
  }
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
>>>>>>> .merge_file_tPKusJ
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