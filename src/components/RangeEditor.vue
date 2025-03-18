<template>
  <div class="flex gap-8">
    <!-- Left Section: Matrix and Controls -->
    <div class="shrink-0 ml-1">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-5">
        <!-- Matrix Table -->
        <table 
          class="shadow-md select-none snug rounded-lg overflow-hidden" 
          @mouseleave="dragEnd"
          style="border-collapse: separate; border-spacing: 0;"
        >
          <tr v-for="row in 13" :key="row" class="h-9">
            <td
              v-for="col in 13"
              :key="col"
              class="relative w-[2.625rem] border border-black"
              :class="{
                'hover:border-yellow-500/50': !cellValue(row, col),
                'hover:border-yellow-400/50': cellValue(row, col) > 0
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
                    'background-image': `linear-gradient(${yellow500} 0% 100%)`,
                    'background-size': `100% ${cellValue(row, col)}%`
                  }"
                ></div>
              </div>
              <div
                :class="[
                  'absolute -top-px left-[0.1875rem] z-10 text-shadow',
                  cellValue(row, col) > 0 ? 'text-white' : 'text-neutral-500'
                ]"
              >
                {{ cellText(row, col) }}
              </div>
              <div
                v-if="cellValue(row, col) > 0 && cellValue(row, col) < 100"
                class="absolute bottom-px right-1 z-10 text-sm text-shadow text-white"
              >
                {{ cellValue(row, col).toFixed(1) }}
              </div>
            </td>
          </tr>
        </table>

        <!-- Weight and Action Controls -->
        <div class="flex items-center justify-between mt-4 mb-4">
          <div class="flex items-center gap-3">
            <span class="text-sm font-medium">Weight:</span>
            <input
              v-model="weight"
              type="number"
              class="w-16 px-2 py-1 rounded text-sm text-center border border-gray-300 dark:border-gray-600"
              min="0"
              max="100"
              step="5"
              @change="onWeightChange"
            />
            <span class="text-sm">%</span>
          </div>
          <button 
            class="p-1.5 rounded bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600"
            @click="clearRange"
            title="Clear Range"
          >
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </button>
        </div>

        <!-- Range Text Input -->
        <div class="mt-4 space-y-3">
          <div class="flex items-center gap-2">
            <input
              v-model="rangeText"
              type="text"
              :class="[
                'w-full px-3 py-2 rounded-lg text-sm border',
                'transition-colors duration-150',
                rangeTextError 
                  ? 'border-red-500/50 bg-red-50/50' 
                  : 'border-gray-300 dark:border-gray-600'
              ]"
              placeholder="Enter range notation (e.g., 'AKs, QQ+')"
              @focus="onFocus"
              @change="onRangeTextChange"
            />
          </div>

          <div v-if="rangeTextError" class="text-sm text-red-500 flex items-center">
            <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/>
            </svg>
            {{ rangeTextError }}
          </div>

          <div class="flex items-center justify-between text-sm text-gray-600 dark:text-gray-400">
            <span>Combos: {{ numCombos.toFixed(1) }}</span>
            <span>{{ ((numCombos * 100) / ((52 * 51) / 2)).toFixed(1) }}% of total</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Middle Section: Range Analysis -->
    <div class="w-[240px]">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-5">
        <h3 class="text-base font-medium mb-4">Range Analysis</h3>
        
        <!-- Hand Type Distribution -->
        <div class="space-y-3 mb-5">
          <div class="flex justify-between text-sm">
            <span>Pocket Pairs</span>
            <span>{{ calculatePocketPairs() }}%</span>
          </div>
          <div class="flex justify-between text-sm">
            <span>Suited</span>
            <span>{{ calculateSuited() }}%</span>
          </div>
          <div class="flex justify-between text-sm">
            <span>Offsuit</span>
            <span>{{ calculateOffsuit() }}%</span>
          </div>
        </div>

        <!-- Quick Templates -->
        <div class="border-t border-gray-200 dark:border-gray-700 pt-4">
          <h4 class="text-sm font-medium mb-3">Quick Templates</h4>
          <div class="grid grid-cols-2 gap-2">
            <button 
              class="px-3 py-2 text-sm rounded bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600"
              @click="loadTemplate('RFI')"
            >
              RFI Range
            </button>
            <button 
              class="px-3 py-2 text-sm rounded bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600"
              @click="loadTemplate('3BET')"
            >
              3Bet Range
            </button>
            <button 
              class="px-3 py-2 text-sm rounded bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600"
              @click="loadTemplate('4BET')"
            >
              4Bet Range
            </button>
            <button 
              class="px-3 py-2 text-sm rounded bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600"
              @click="loadTemplate('CALL')"
            >
              Call Range
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Right Section: Database -->
    <div class="flex-1">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-5">
        <h3 class="text-base font-medium mb-4">Saved Ranges</h3>
        <DbItemPicker
          store-name="ranges"
          :index="player"
          :value="rangeText"
          :allow-save="rangeText !== '' && rangeTextError === ''"
          :hide-import-export="player >= 2"
          @load-item="loadRange"
        />

        <!-- Action Buttons -->
        <div v-if="player >= 2" class="mt-6 flex justify-end gap-3">
          <button 
            class="px-4 py-2 rounded-lg bg-red-600 hover:bg-red-700 text-white text-sm font-medium"
            @click="emit('cancel')"
          >
            Cancel
          </button>
          <button 
            class="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium"
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

const yellow500 = "#eab308";

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

const onFocus = (e: Event) => {
  const input = e.target as HTMLInputElement;
  input.select();
};

// Add new functions for range analysis
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

const loadTemplate = (type: string) => {
  // Implementation for loading range templates
  // This would need to be implemented based on your requirements
};
</script>
