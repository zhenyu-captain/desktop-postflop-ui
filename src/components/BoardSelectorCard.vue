<template>
  <button
    :class="[
      'relative rounded-lg border select-none transition-all duration-200',
      isSelected 
        ? 'bg-gradient-to-br from-amber-50 to-amber-100 ring-2 ring-blue-500 border-blue-400 shadow-md transform scale-105' 
        : 'bg-white border-gray-300 hover:border-gray-400 enabled:hover:shadow-md',
      disabled ? 'opacity-60' : 'enabled:hover:scale-105'
    ]"
    :style="{
      '--width': width,
      '--font-size': fontSize,
      width: 'var(--width)',
      'padding-top': 'calc(var(--width) * 1.4 - 2px)',
    }"
    :disabled="disabled"
    :aria-pressed="isSelected"
    :aria-label="`${rank} of ${suit === '♠' ? 'spades' : suit === '♥' ? 'hearts' : suit === '♦' ? 'diamonds' : 'clubs'}`"
    @click="$emit('click')"
  >
    <!-- Card Value -->
    <span
      :class="[
        'absolute top-0 font-semibold', 
        colorClass,
        isSelected ? 'drop-shadow-sm' : ''
      ]"
      :style="{
        left: '15%',
        'font-size': 'calc(var(--font-size) * 1.25)',
      }"
    >
      {{ rank }}
    </span>
    
    <!-- Card Suit -->
    <span
      :class="[
        'absolute', 
        colorClass,
        isSelected ? 'drop-shadow-sm' : ''
      ]"
      :style="{
        bottom: '5%',
        right: '10%',
        'font-size': 'var(--font-size)',
      }"
    >
      {{ suit }}
    </span>
  </button>
</template>

<script setup lang="ts">
import { cardText } from "../utils";

interface Props {
  cardId: number;
  isSelected?: boolean;
  disabled?: boolean;
  width?: string;
  fontSize?: string;
}

const props = withDefaults(defineProps<Props>(), {
  isSelected: false,
  disabled: false,
  width: "40px",
  fontSize: "1rem",
});

// 确保使用cardText函数将cardId转换为可读的卡牌表示
const { rank, suit, colorClass } = cardText(props.cardId);
</script> 