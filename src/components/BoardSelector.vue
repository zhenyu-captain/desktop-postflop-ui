<template>
  <div class="flex flex-col space-y-4">
    <!-- 牌选择器 - 使用现代网格设计替代旧的FlexBox布局 -->
    <div class="grid grid-cols-13 gap-1">
      <template v-for="suit in ['s', 'h', 'd', 'c']" :key="suit">
        <div v-for="rank in ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']" 
             :key="rank + suit"
             class="w-8 h-8 flex items-center justify-center rounded cursor-pointer text-sm"
             :class="[
               getCardColor(suit),
               isCardSelected(rank + suit) ? 'ring-2 ring-blue-500' : 'hover:bg-gray-100'
             ]"
             @click="toggleCardByString(rank + suit)">
          {{ rank }}{{ getSuitSymbol(suit) }}
        </div>
      </template>
    </div>

    <!-- 已选牌展示 -->
    <div class="flex items-center space-x-4">
      <div class="flex space-x-2">
        <div v-for="(card, index) in displayCards" :key="index" 
             class="w-12 h-16 bg-white rounded shadow-md flex items-center justify-center text-xl"
             :class="getCardColor(card.suit)">
          {{ card.rank }}{{ card.suit }}
        </div>
      </div>
      
      <!-- 输入框 - 保留旧版本的功能 -->
      <input
        v-model="boardText"
        type="text"
        class="w-40 px-2 py-1 rounded-lg text-sm border border-gray-300"
        @focus="($event.target as HTMLInputElement).select()"
        @change="onBoardTextChange"
      />
      
      <!-- 按钮分组 -->
      <div class="flex space-x-2">
        <button 
          @click="clearBoard"
          class="px-3 py-1 text-sm button-base button-blue">
          Clear
        </button>
        <button 
          @click="generateRandomBoard"
          class="px-3 py-1 text-sm button-base button-blue">
          Random Flop
        </button>
      </div>
    </div>

    <!-- 警告信息 - 保留原有功能 -->
    <div
      v-if="
        config.board.length >= 3 &&
        config.expectedBoardLength > 0 &&
        config.board.length !== config.expectedBoardLength
      "
      class="mt-5 text-orange-500 font-semibold"
    >
      <span class="underline">Warning:</span>
      The edited tree assumes a {{ config.expectedBoardLength }}-card board.
    </div>
    
    <!-- 统计信息卡片 - 添加新UI元素 -->
    <div class="grid grid-cols-2 gap-4">
      <!-- 左侧：组合统计 -->
      <div class="p-4 bg-gray-50 rounded-lg">
        <h3 class="text-sm font-medium mb-3">组合统计</h3>
        <div class="space-y-2">
          <div class="flex justify-between">
            <span class="text-sm text-gray-600">剩余组合数:</span>
            <span class="text-sm font-medium">{{ remainingCombos }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-sm text-gray-600">被OOP范围阻挡:</span>
            <span class="text-sm font-medium">{{ blockedByOOP }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-sm text-gray-600">同花组合:</span>
            <span class="text-sm font-medium">{{ flushCombos }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-sm text-gray-600">顺子组合:</span>
            <span class="text-sm font-medium">{{ straightCombos }}</span>
          </div>
        </div>
      </div>

      <!-- 右侧：牌面强度 -->
      <div class="p-4 bg-gray-50 rounded-lg">
        <h3 class="text-sm font-medium mb-3">牌面强度</h3>
        <div class="space-y-3">
          <div v-for="(level, index) in strengthLevels" :key="index"
               class="flex items-center">
            <span class="text-sm w-12">{{ level.name }}</span>
            <div class="flex-1 grid grid-cols-2 gap-2">
              <div class="flex items-center space-x-2">
                <div class="w-20 h-4 rounded bg-blue-100 overflow-hidden">
                  <div class="h-full bg-blue-500"
                       :style="{ width: level.oopValue + '%' }"></div>
                </div>
                <span class="text-xs">{{ level.oopValue }}%</span>
              </div>
              <div class="flex items-center space-x-2">
                <div class="w-20 h-4 rounded bg-green-100 overflow-hidden">
                  <div class="h-full bg-green-500"
                       :style="{ width: level.ipValue + '%' }"></div>
                </div>
                <span class="text-xs">{{ level.ipValue }}%</span>
              </div>
            </div>
          </div>
          <div class="flex justify-end space-x-4 text-xs text-gray-500 mt-2">
            <span>■ OOP</span>
            <span>■ IP</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { useStore, useConfigStore } from "../store";
import { cardText, parseCardString } from "../utils";

// 保留原版逻辑的Store引用
const store = useStore();
const config = useConfigStore();
const boardText = ref("");

// 从config.board派生的卡牌显示数据
const displayCards = computed(() => {
  if (config.board.length === 0) {
    return Array(3).fill({ rank: '', suit: '' });
  }
  
  const cards = config.board.map(cardId => {
    const info = cardText(cardId);
    return {
      rank: info.rank,
      suit: info.suit
    };
  });
  
  // 填充至少3个位置
  while (cards.length < 3) {
    cards.push({ rank: '', suit: '' });
  }
  
  return cards;
});

// 检查卡牌是否已选中
const isCardSelected = (cardString: string) => {
  const cardId = parseCardString(cardString.toLowerCase());
  return cardId !== null && config.board.includes(cardId);
};

// 切换卡牌状态 - 通过字符串
const toggleCardByString = (cardString: string) => {
  const cardId = parseCardString(cardString.toLowerCase());
  if (cardId === null) return;
  
  toggleCard(cardId);
};

// 原有逻辑 - 切换卡牌状态
const toggleCard = (cardId: number, updateText = true) => {
  if (config.board.includes(cardId)) {
    config.board = config.board.filter((card) => card !== cardId);
  } else if (config.board.length < 5) {
    config.board.push(cardId);
    if (config.board.length <= 3) {
      config.board.sort((a, b) => b - a);
    }
  }

  if (updateText) {
    setBoardTextFromButtons();
  }
  
  // 更新bunching flop
  if (store.isBunchingEnabled && config.board.length === 3) {
    store.bunchingFlop = [...config.board];
  }
};

// 原有逻辑 - 根据按钮设置文本
const setBoardTextFromButtons = () => {
  boardText.value = config.board
    .map(cardText)
    .map(({ rank, suitLetter }) => rank.toLowerCase() + suitLetter)
    .join(", ");
};

// 原有逻辑 - 处理文本变化
const onBoardTextChange = () => {
  config.board = [];

  const cardIds = boardText.value
    // Allow pasting in things like [Ah Kd Qc], by reformatting to Ah,Kd,Qc
    .trim()
    .replace(/[^A-Za-z0-9\s,]/g, "")
    .replace(/\s+/g, ",")
    .split(",")
    .map(parseCardString)
    .filter((cardId): cardId is number => cardId !== null);

  new Set(cardIds).forEach((cardId) => toggleCard(cardId, false));
  setBoardTextFromButtons();
  
  // 更新bunching flop
  if (store.isBunchingEnabled && config.board.length === 3) {
    store.bunchingFlop = [...config.board];
  }
};

// 原有逻辑 - 清空牌面
const clearBoard = () => {
  config.board = [];
  setBoardTextFromButtons();
  
  // 清空bunching flop
  if (store.isBunchingEnabled) {
    store.bunchingFlop = [];
  }
};

// 原有逻辑 - 生成随机翻牌
const generateRandomBoard = () => {
  config.board = [];

  while (config.board.length < 3) {
    const randomCard = Math.floor(Math.random() * 52);
    if (!config.board.includes(randomCard)) {
      config.board.push(randomCard);
    }
  }

  config.board.sort((a, b) => b - a);
  setBoardTextFromButtons();
  
  // 更新bunching flop
  if (store.isBunchingEnabled) {
    store.bunchingFlop = [...config.board];
  }
};

// 根据花色获取颜色CSS类
const getCardColor = (suit: string) => {
  if (suit === '♥' || suit === '♦' || suit === 'h' || suit === 'd') {
    return 'text-pink-600';
  }
  return 'text-gray-900';
};

// 获取花色符号
const getSuitSymbol = (suit: string) => {
  const symbols: Record<string, string> = {
    's': '♠',
    'h': '♥',
    'd': '♦',
    'c': '♣'
  };
  return symbols[suit] || suit;
};

// 统计数据 - 示例数据，可以根据实际需求进行计算
const remainingCombos = computed(() => {
  // 实际应该根据当前牌计算剩余组合
  return 1081 - (config.board.length * 50);
});

const blockedByOOP = computed(() => {
  // 实际应该根据当前OOP范围和牌计算
  return 78;
});

const flushCombos = computed(() => {
  // 实际应该根据当前牌计算同花可能性
  return 45;
});

const straightCombos = computed(() => {
  // 实际应该根据当前牌计算顺子可能性
  return 32;
});

// 示例牌力等级数据
const strengthLevels = [
  { name: '极弱', oopValue: 15, ipValue: 10 },
  { name: '较弱', oopValue: 25, ipValue: 20 },
  { name: '中等', oopValue: 30, ipValue: 35 },
  { name: '较强', oopValue: 20, ipValue: 25 },
  { name: '极强', oopValue: 10, ipValue: 10 }
];
</script>

<style scoped>
.grid-cols-13 {
  grid-template-columns: repeat(13, minmax(0, 1fr));
}
</style>
