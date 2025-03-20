<template>
  <div class="flex gap-8">
    <!-- 左侧：矩阵和控件 -->
    <div class="shrink-0">
      <div class="bg-white rounded-lg shadow-lg p-5">
        <!-- 矩阵表格 -->
        <table 
          class="shadow-lg select-none rounded-lg overflow-hidden" 
          @mouseleave="dragEnd"
          style="border-collapse: separate; border-spacing: 0;"
        >
          <tr v-for="row in 13" :key="row" class="h-9">
            <td
              v-for="col in 13"
              :key="col"
              class="relative w-[2.625rem] border border-gray-900 transition-all duration-150"
              :class="{
                'hover:border-blue-500': !cellValue(row, col),
                'hover:border-white': cellValue(row, col) > 0
              }"
              @mousedown="dragStart(row, col)"
              @mouseup="dragEnd"
              @mouseenter="mouseEnter(row, col)"
            >
              <div
                :class="[
                  'absolute w-full h-full left-0 top-0',
                  row === col ? 'bg-neutral-700' : 'bg-neutral-800'
                ]"
              >
                <div
                  class="absolute w-full h-full left-0 top-0 bg-bottom bg-no-repeat"
                  :style="{
                    'background-image': `linear-gradient(${row === col ? colors.pocketPairs : (row > col ? colors.suited : colors.offsuit)} 0% 100%)`,
                    'background-size': `100% ${cellValue(row, col)}%`,
                    'box-shadow': cellValue(row, col) > 0 ? 'inset 0 0 4px rgba(255, 255, 255, 0.15)' : 'none'
                  }"
                ></div>
              </div>
              <div
                :class="[
                  'absolute -top-px left-[0.1875rem] z-10 font-medium',
                  cellValue(row, col) > 0 ? 'text-white' : 'text-gray-400'
                ]"
                style="text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);"
              >
                {{ cellText(row, col) }}
              </div>
              <div
                v-if="cellValue(row, col) > 0 && cellValue(row, col) < 100"
                class="absolute bottom-px right-1 z-10 text-xs font-medium text-white"
                style="text-shadow: 0 1px 1px rgba(0, 0, 0, 0.7);"
              >
                {{ cellValue(row, col).toFixed(1) }}
              </div>
            </td>
          </tr>
        </table>

        <!-- 权重和动作控件 -->
        <div class="mt-5 space-y-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <span class="text-sm font-medium text-gray-700">Weight:</span>
              <input
                v-model="weight"
                type="range"
                class="w-32 align-middle accent-blue-500"
                min="0"
                max="100"
                step="5"
                @change="onWeightChange"
              />
              <input
                v-model="weight"
                type="number"
                :class="[
                  'w-16 px-2 py-1 rounded text-sm text-center border transition-colors duration-150',
                  weight < 0 || weight > 100 
                    ? 'border-red-500 bg-red-50' 
                    : 'border-gray-300 focus:border-blue-400 focus:ring-1 focus:ring-blue-400 focus:outline-none'
                ]"
                min="0"
                max="100"
                step="5"
                @change="onWeightChange"
              />
              <span class="text-sm text-gray-700">%</span>
            </div>
          </div>

          <!-- 范围文本输入 -->
          <div class="flex items-center gap-2 mt-3">
            <input
              v-model="rangeText"
              type="text"
              :class="[
                'flex-grow px-3 py-2 rounded-lg text-sm border transition-colors duration-150 focus:outline-none focus:ring-1',
                rangeTextError 
                  ? 'border-red-500 bg-red-50 focus:ring-red-400' 
                  : 'border-gray-300 focus:border-blue-400 focus:ring-blue-400'
              ]"
              placeholder="Enter range notation (e.g., 'AKs, QQ+')"
              @focus="($event.target as HTMLInputElement).select()"
              @change="onRangeTextChange"
            />

            <div class="flex gap-2">
              <button
                v-if="player >= 2"
                class="px-3 py-1.5 rounded-lg bg-slate-100 hover:bg-slate-200 text-slate-700 text-sm font-medium transition-colors duration-150"
                @click="invertRange"
              >
                Invert
              </button>

              <button 
                class="px-3 py-1.5 rounded-lg bg-slate-100 hover:bg-slate-200 text-slate-700 text-sm font-medium transition-colors duration-150"
                @click="clearRange"
              >
                Clear
              </button>
            </div>
          </div>

          <div v-if="rangeTextError" class="text-sm text-red-500 flex items-center mt-1">
            <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/>
            </svg>
            {{ rangeTextError }}
          </div>

          <div class="flex items-center justify-between text-sm text-gray-600 mt-2">
            <span>Combos: <span class="font-medium">{{ numCombos.toFixed(1) }}</span></span>
            <span>{{ ((numCombos * 100) / ((52 * 51) / 2)).toFixed(1) }}% of total</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 中间：范围分析 -->
    <div class="w-[240px]">
      <div class="bg-white rounded-lg shadow-lg p-5">
        <h3 class="text-base font-medium text-gray-800 mb-4">Range Analysis</h3>
        
        <!-- 手牌类型分布 -->
        <div class="space-y-3 mb-5">
          <div class="flex justify-between text-sm">
            <span class="text-gray-700">Pocket Pairs</span>
            <span class="font-medium text-blue-600">{{ calculatePocketPairs() }}%</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-gray-700">Suited</span>
            <span class="font-medium text-purple-600">{{ calculateSuited() }}%</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-gray-700">Offsuit</span>
            <span class="font-medium text-emerald-600">{{ calculateOffsuit() }}%</span>
          </div>
        </div>

        <!-- 快速模板 -->
        <div class="border-t border-gray-200 pt-4">
          <h4 class="text-sm font-medium text-gray-800 mb-3">Quick Templates</h4>
          <div class="grid grid-cols-2 gap-2">
            <button 
              class="px-3 py-2 text-sm rounded-lg bg-blue-50 hover:bg-blue-100 text-blue-700 transition-colors duration-150"
              @click="loadTemplate('RFI')"
            >
              RFI Range
            </button>
            <button 
              class="px-3 py-2 text-sm rounded-lg bg-purple-50 hover:bg-purple-100 text-purple-700 transition-colors duration-150"
              @click="loadTemplate('3BET')"
            >
              3Bet Range
            </button>
            <button 
              class="px-3 py-2 text-sm rounded-lg bg-emerald-50 hover:bg-emerald-100 text-emerald-700 transition-colors duration-150"
              @click="loadTemplate('4BET')"
            >
              4Bet Range
            </button>
            <button 
              class="px-3 py-2 text-sm rounded-lg bg-amber-50 hover:bg-amber-100 text-amber-700 transition-colors duration-150"
              @click="loadTemplate('CALL')"
            >
              Call Range
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 右侧：数据库 -->
    <div class="flex-1">
      <div class="bg-white rounded-lg shadow-lg p-5">
        <DbItemPicker
          store-name="ranges"
          :index="player"
          :value="rangeText"
          :allow-save="rangeText !== '' && rangeTextError === ''"
          :hide-import-export="player >= 2"
          @load-item="loadRange"
        />

        <!-- 操作按钮 -->
        <div v-if="player >= 2" class="mt-6 flex justify-end gap-3">
          <button 
            class="px-4 py-2 rounded-lg bg-gray-200 hover:bg-gray-300 text-gray-700 text-sm font-medium transition-colors duration-150"
            @click="emit('cancel')"
          >
            Cancel
          </button>
          <button 
            class="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium transition-colors duration-150"
            @click="emit('save')"
          >
            Save
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useStore } from "../store";
import { ranks, trimRegex, rangeRegex } from "../utils";
import * as invokes from "../invokes";

import DbItemPicker from "./DbItemPicker.vue";

// 颜色方案
const colors = {
  pocketPairs: "#3b82f6",   // 蓝色 - 口袋对子
  suited: "#8b5cf6",        // 紫色 - 同花
  offsuit: "#10b981"        // 绿色 - 非同花
};

type DraggingMode = "none" | "enabling" | "disabling";

const props = withDefaults(
  defineProps<{ player: number; defaultText?: string }>(),
  { defaultText: "" }
);

const emit = defineEmits<{
  (event: "save"): void;
  (event: "cancel"): void;
}>();

const store = useStore();
const rangeText = ref(props.defaultText);
const rangeTextError = ref("");
const weight = ref(100);
const numCombos = ref(0);

let draggingMode: DraggingMode = "none";

const cellText = (row: number, col: number) => {
  const r1 = 13 - Math.min(row, col);
  const r2 = 13 - Math.max(row, col);
  return ranks[r1] + ranks[r2] + ["s", "", "o"][Math.sign(row - col) + 1];
};

const cellIndex = (row: number, col: number) => {
  return 13 * (row - 1) + col - 1;
};

const cellValue = (row: number, col: number) => {
  return store.ranges[props.player][cellIndex(row, col)];
};

const onUpdate = async () => {
  rangeText.value = await invokes.rangeToString(props.player);
  numCombos.value = await invokes.rangeNumCombos(props.player);
  rangeTextError.value = "";
};

const update = async (row: number, col: number, weight: number) => {
  const idx = 13 * (row - 1) + col - 1;
  await invokes.rangeUpdate(props.player, row, col, weight / 100);
  store.ranges[props.player][idx] = weight;
  await onUpdate();
};

const onRangeTextChange = async () => {
  const trimmed = rangeText.value.replace(trimRegex, "$1").trim();
  const ranges = trimmed.split(",");

  if (ranges[ranges.length - 1] === "") {
    ranges.pop();
  }

  for (const range of ranges) {
    if (!rangeRegex.test(range)) {
      rangeTextError.value = `Failed to parse range: ${
        range || "(empty string)"
      }`;
      return;
    }
  }

  const errorString = await invokes.rangeFromString(props.player, trimmed);

  if (errorString) {
    rangeTextError.value = errorString;
  } else {
    const weights = await invokes.rangeGetWeights(props.player);
    for (let i = 0; i < 13 * 13; ++i) {
      store.ranges[props.player][i] = weights[i] * 100;
    }
    await onUpdate();
  }
};

const dragStart = (row: number, col: number) => {
  const idx = 13 * (row - 1) + col - 1;

  if (store.ranges[props.player][idx] !== weight.value) {
    draggingMode = "enabling";
    update(row, col, weight.value);
  } else {
    draggingMode = "disabling";
    update(row, col, 0);
  }
};

const dragEnd = () => {
  draggingMode = "none";
};

const mouseEnter = (row: number, col: number) => {
  if (draggingMode === "enabling") {
    update(row, col, weight.value);
  } else if (draggingMode === "disabling") {
    update(row, col, 0);
  }
};

const onWeightChange = () => {
  weight.value = Math.round(Math.max(0, Math.min(100, weight.value)));
};

const clearRange = async () => {
  await invokes.rangeClear(props.player);
  store.ranges[props.player].fill(0);
  rangeText.value = "";
  rangeTextError.value = "";
  weight.value = 100;
  numCombos.value = 0;
};

const invertRange = async () => {
  await invokes.rangeInvert(props.player);
  for (let i = 0; i < 13 * 13; ++i) {
    store.ranges[props.player][i] = 100 - store.ranges[props.player][i];
  }
  await onUpdate();
};

const loadRange = (rangeStr: unknown) => {
  rangeText.value = String(rangeStr);
  onRangeTextChange();
};

// 范围分析功能
const calculatePocketPairs = () => {
  let total = 0;
  for (let i = 0; i < 13; i++) {
    total += store.ranges[props.player][i * 13 + i];
  }
  return (total / 1300 * 100).toFixed(1);
};

const calculateSuited = () => {
  let total = 0;
  for (let i = 0; i < 13; i++) {
    for (let j = i + 1; j < 13; j++) {
      total += store.ranges[props.player][i * 13 + j];
    }
  }
  return (total / 1300 * 100).toFixed(1);
};

const calculateOffsuit = () => {
  let total = 0;
  for (let i = 0; i < 13; i++) {
    for (let j = 0; j < i; j++) {
      total += store.ranges[props.player][i * 13 + j];
    }
  }
  return (total / 1300 * 100).toFixed(1);
};

// 快速模板加载
const loadTemplate = (type: string) => {
  let rangeStr = "";
  switch (type) {
    case 'RFI':
      rangeStr = "22+, A2s+, K9s+, QTs+, JTs, T9s, 98s, 87s, 76s, 65s, ATo+, KJo+, QJo";
      break;
    case '3BET':
      rangeStr = "JJ+, AQs+, AKo";
      break;
    case '4BET':
      rangeStr = "QQ+, AKs";
      break;
    case 'CALL':
      rangeStr = "TT-22, AJs-A2s, KQs-K9s, QJs-QTs, JTs, T9s, 98s, 87s, 76s, AQo-ATo, KQo-KJo";
      break;
  }
  rangeText.value = rangeStr;
  onRangeTextChange();
};
</script>
