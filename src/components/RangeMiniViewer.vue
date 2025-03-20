<template>
  <table class="shadow-md">
    <tr v-for="row in 13" :key="row" class="h-2.5">
      <td
        v-for="col in 13"
        :key="col"
        class="relative w-2.5 border-[0.5px] border-gray-300"
      >
        <div
          :class="[
            'absolute w-full h-full left-0 top-0',
            row === col ? 'bg-blue-50' : (row > col ? 'bg-fuchsia-50' : 'bg-emerald-50')
          ]"
        >
          <div
            class="absolute w-full h-full left-0 top-0 bg-bottom bg-no-repeat"
            :style="{
              'background-image': `linear-gradient(${row === col ? colors.pocketPairs : (row > col ? colors.suited : colors.offsuit)} 0% 100%)`,
              'background-size': `100% ${cellValue(row, col)}%`,
            }"
          ></div>
        </div>
      </td>
    </tr>
  </table>
</template>

<script setup lang="ts">
import { useStore } from "../store";

// 颜色方案
const colors = {
  pocketPairs: "#4f46e5",   // 靛蓝色 - 口袋对子 (indigo-600)
  suited: "#c026d3",        // 洋红色 - 同花 (fuchsia-600)
  offsuit: "#059669"         // 绿色 - 非同花 (emerald-600)
};

const props = defineProps<{ player: number }>();
const store = useStore();

const cellValue = (row: number, col: number) => {
  const cellIndex = (row - 1) * 13 + (col - 1);
  return store.ranges[props.player][cellIndex];
};
</script>
