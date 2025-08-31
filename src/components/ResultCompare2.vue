<template>
  <div class="flex flex-col w-full h-full px-3 py-2 gap-3 bg-white rounded-lg shadow-sm border border-gray-200">
    <!-- 标题部分 -->
    <div class="flex items-center justify-between mb-2">
      <div class="flex items-center">
        <div class="h-4 w-4 rounded-sm bg-blue-500 mr-1.5"></div>
        <span class="text-blue-700 font-semibold mr-0.5">OOP</span>
        <StarIcon
          v-if="player === 'oop'"
          class="inline-block ml-0.5 w-4 h-4 text-yellow-500"
        />
      </div>
      <div class="text-gray-400 font-medium text-sm">vs</div>
      <div class="flex items-center">
        <StarIcon
          v-if="player === 'ip'"
          class="inline-block mr-0.5 w-4 h-4 text-yellow-500"
        />
        <span class="text-emerald-700 font-semibold ml-0.5">IP</span>
        <div class="h-4 w-4 rounded-sm bg-emerald-500 ml-1.5"></div>
      </div>
    </div>

    <!-- Combos -->
    <div class="flex flex-col gap-1 border-b border-gray-100 pb-2">
      <div class="flex justify-between items-center">
        <Adaptive
          :value="combos[0]"
          :class="{
            'font-semibold text-blue-600': combos[0] && combos[0] >= (1 - EPS) * combos[1],
            'text-blue-500': !(combos[0] && combos[0] >= (1 - EPS) * combos[1]),
          }"
        />
        <span class="text-sm font-medium text-gray-600 uppercase tracking-wide">Combos</span>
        <Adaptive
          :value="combos[1]"
          :class="{
            'text-right': true,
            'font-semibold text-emerald-600': combos[1] && (1 - EPS) * combos[0] <= combos[1],
            'text-emerald-500': !(combos[1] && (1 - EPS) * combos[0] <= combos[1]),
          }"
        />
      </div>
      <BarChart :values="combos" />
    </div>

    <!-- Equity -->
    <div class="flex flex-col gap-1 border-b border-gray-100 pb-2">
      <div class="flex justify-between items-center">
        <Percentage
          :value="equity[0]"
          :class="{
            'font-semibold text-blue-600':
              !isNaN(equity[0]) && equity[0] >= (1 - EPS) * equity[1],
            'text-blue-500': !(!isNaN(equity[0]) && equity[0] >= (1 - EPS) * equity[1]),
          }"
        />
        <span class="text-sm font-medium text-gray-600 uppercase tracking-wide">Equity</span>
        <Percentage
          :value="equity[1]"
          :class="{
            'text-right': true,
            'font-semibold text-emerald-600':
              !isNaN(equity[1]) && (1 - EPS) * equity[0] <= equity[1],
            'text-emerald-500': !(!isNaN(equity[1]) && (1 - EPS) * equity[0] <= equity[1]),
          }"
        />
      </div>
      <BarChart :values="equity" />
    </div>

    <!-- EV -->
    <div class="flex flex-col gap-1 border-b border-gray-100 pb-2">
      <div class="flex justify-between items-center">
        <Ev
          :value="ev[0]"
          :digits="evDigits"
          :class="{
            'font-semibold text-blue-600': !isNaN(ev[0]) && ev[0] >= (1 - EPS) * ev[1],
            'text-blue-500': !(!isNaN(ev[0]) && ev[0] >= (1 - EPS) * ev[1]),
          }"
        />
        <span class="text-sm font-medium text-gray-600 uppercase tracking-wide">EV</span>
        <Ev
          :value="ev[1]"
          :digits="evDigits"
          :class="{
            'text-right': true,
            'font-semibold text-emerald-600': !isNaN(ev[1]) && (1 - EPS) * ev[0] <= ev[1],
            'text-emerald-500': !(!isNaN(ev[1]) && (1 - EPS) * ev[0] <= ev[1]),
          }"
        />
      </div>
      <BarChart :values="ev" />
    </div>

    <!-- EQR -->
    <div class="flex flex-col gap-1">
      <div class="flex justify-between items-center">
        <Percentage
          :value="eqr[0]"
          :class="{
            'font-semibold text-blue-600': !isNaN(eqr[0]) && eqr[0] >= (1 - EPS) * eqr[1],
            'text-blue-500': !(!isNaN(eqr[0]) && eqr[0] >= (1 - EPS) * eqr[1]),
          }"
        />
        <span class="text-sm font-medium text-gray-600 uppercase tracking-wide">EQR</span>
        <Percentage
          :value="eqr[1]"
          :class="{
            'text-right': true,
            'font-semibold text-emerald-600': !isNaN(eqr[1]) && (1 - EPS) * eqr[0] <= eqr[1],
            'text-emerald-500': !(!isNaN(eqr[1]) && (1 - EPS) * eqr[0] <= eqr[1]),
          }"
        />
      </div>
      <BarChart :values="eqr" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, h } from "vue";
import { average, toFixed1, toFixed, toFixedAdaptive } from "../utils";
import { Results, Spot, SpotChance } from "../result-types";

import { StarIcon } from "@heroicons/vue/24/solid";

const EPS = 2e-6;
const blue500 = "#3b82f6";  // 蓝色 (blue-500)
const green500 = "#10b981";  // 绿色 (emerald-500)

const Adaptive = (props: { value: number; class: object }) => {
  const split = computed(() => toFixedAdaptive(props.value).split("."));
  if (isNaN(props.value)) return h("div", { class: "px-1" }, "-");
  return h("div", { class: props.class }, [
    h("span", {}, split.value[0] + "."),
    h("span", { class: "text-sm" }, split.value[1]),
  ]);
};

const Percentage = (props: { value: number; class: object }) => {
  const str = computed(() => toFixed1(props.value * 100));
  if (isNaN(props.value)) return h("div", { class: "px-1" }, "-");
  return h("div", { class: props.class }, [
    h("span", {}, str.value.slice(0, -1)),
    h("span", { class: "text-sm" }, str.value.slice(-1) + "%"),
  ]);
};

const Ev = (props: { value: number; digits: number; class: object }) => {
  const str = computed(() => toFixed[props.digits - 1](props.value));
  if (isNaN(props.value)) return h("div", { class: "px-1" }, "-");
  return h("span", { class: props.class }, [
    h("span", {}, str.value.slice(0, -props.digits)),
    h("span", { class: "text-xs" }, str.value.slice(-props.digits)),
  ]);
};

const BarChart = (props: { values: number[] }) => {
  if (
    (props.values[0] === 0 && props.values[1] === 0) ||
    isNaN(props.values[0]) ||
    isNaN(props.values[1])
  ) {
    return h("div", { class: "h-4 rounded-lg bg-gray-100" });
  }
  const total = props.values[0] + props.values[1];
  const sep = `${(props.values[0] * 100) / total}%`;
  const bg = `linear-gradient(to right, ${blue500} ${sep}, ${green500} ${sep})`;
  return h("div", { 
    class: "h-4 rounded-lg shadow-sm transition-all duration-300", 
    style: `background: ${bg}` 
  });
};

const props = defineProps<{
  selectedSpot: Spot;
  selectedChance: SpotChance | null;
  results: Results;
}>();

const player = computed(() => {
  if (props.selectedChance) return "chance";
  return props.selectedSpot.player;
});

const combos = computed(() => {
  const results = props.results;

  const ret: number[] = [];
  for (let i = 0; i < 2; ++i) {
    let sum = 0;
    const n = results.weights[i].length;
    for (let j = 0; j < n; ++j) {
      const weight = results.weights[i][j];
      const normalizer = results.normalizer[i][j];
      sum += normalizer && weight;
    }
    ret.push(sum);
  }

  return ret;
});

const equity = computed(() => {
  const results = props.results;
  if (results.isEmpty) return [Number.NaN, Number.NaN];
  return [
    average(results.equity[0], results.normalizer[0]),
    average(results.equity[1], results.normalizer[1]),
  ];
});

const ev = computed(() => {
  const results = props.results;
  if (results.isEmpty) return [Number.NaN, Number.NaN];
  return [
    average(results.ev[0], results.normalizer[0]),
    average(results.ev[1], results.normalizer[1]),
  ];
});

const eqr = computed(() => {
  const results = props.results;
  if (results.isEmpty) return [Number.NaN, Number.NaN];
  const ret = [
    ev.value[0] / (results.eqrBase[0] * equity.value[0]),
    ev.value[1] / (results.eqrBase[1] * equity.value[1]),
  ];
  if (!isFinite(ret[0])) ret[0] = Number.NaN;
  if (!isFinite(ret[1])) ret[1] = Number.NaN;
  return ret;
});

const evDigits = computed(() => {
  const results = props.results;
  if (results.isEmpty) return 3;
  const maxEv = Math.max(...ev.value.map((x) => Math.abs(x)));
  return maxEv < 9.9995 ? 3 : maxEv < 99.995 ? 2 : 1;
});
</script>
