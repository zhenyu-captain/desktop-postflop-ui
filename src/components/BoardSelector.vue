<template>
  <div class="flex flex-col space-y-4">
    <!-- Card Selector - Modern grid design -->
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

    <!-- Selected Cards Display -->
    <div class="flex items-center space-x-4">
      <div class="flex space-x-2">
        <div v-for="(card, index) in displayCards" :key="index" 
             class="w-12 h-16 bg-white rounded shadow-md flex items-center justify-center text-xl"
             :class="getCardColor(card.suit)">
          {{ card.rank }}{{ card.suit }}
        </div>
      </div>
      
      <!-- Input Field -->
      <input
        v-model="boardText"
        type="text"
        class="w-40 px-2 py-1 rounded-lg text-sm border border-gray-300"
        @focus="($event.target as HTMLInputElement).select()"
        @change="onBoardTextChange"
      />
      
      <!-- Button Group -->
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
        <button 
          @click="toggleDebugInfo"
          class="px-3 py-1 text-sm button-base button-yellow">
          Debug
        </button>
      </div>
    </div>

    <!-- Debug Info Panel -->
    <div v-if="showDebugInfo" class="p-4 bg-gray-100 rounded-lg text-xs overflow-auto max-h-80">
      <div class="flex justify-between items-center mb-3">
        <h3 class="font-bold">Board Analysis Data:</h3>
        <button 
          class="px-3 py-1 text-xs bg-blue-500 hover:bg-blue-600 text-white rounded" 
          @click="copyDebugDataToClipboard">
          Copy JSON
        </button>
      </div>
      
      <pre class="bg-white p-2 rounded text-xs overflow-auto max-h-64">{{ formattedDebugData }}</pre>
      
      <div class="mt-3 flex justify-end">
        <button 
          class="px-3 py-1 text-xs bg-purple-500 hover:bg-purple-600 text-white rounded" 
          @click="showDebugInfo = false">Close
        </button>
      </div>
    </div>

    <!-- Warning Message -->
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
    
    <!-- Statistics Cards -->
    <div class="grid grid-cols-2 gap-4">
      <!-- Left: Combo Statistics -->
      <div class="p-4 bg-gray-50 rounded-lg">
        <h3 class="text-sm font-medium mb-3">Combo Statistics</h3>
        <div class="space-y-2">
          <div class="flex justify-between">
            <span class="text-sm text-gray-600">Remaining Combos:</span>
            <span class="text-sm font-medium">{{ remainingCombos }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-sm text-gray-600">Blocked by OOP:</span>
            <span class="text-sm font-medium">{{ blockedByOOP }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-sm text-gray-600">Flush Combos:</span>
            <span class="text-sm font-medium">{{ flushCombos }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-sm text-gray-600">Straight Combos:</span>
            <span class="text-sm font-medium">{{ straightCombos }}</span>
          </div>
        </div>
      </div>

      <!-- Right: Board Strength -->
      <div class="p-4 bg-gray-50 rounded-lg">
        <h3 class="text-sm font-medium mb-3">Board Strength</h3>
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

    <!-- Equity Distribution -->
    <div class="p-4 bg-gray-50 rounded-lg">
      <h3 class="text-sm font-medium mb-3">Equity Distribution</h3>
      <div class="space-y-3">
        <div v-for="(range, index) in equityRanges" :key="index"
             class="grid grid-cols-2 gap-4">
          <div class="flex items-center space-x-2">
            <span class="text-sm w-16">{{ range.label }}</span>
            <div class="flex-1 h-4 rounded bg-gray-100 overflow-hidden">
              <div class="h-full"
                   :style="{ 
                     width: range.oopValue + '%',
                     backgroundColor: getEquityColor(range.min) 
                   }"></div>
            </div>
            <span class="text-xs w-8 text-right">{{ range.oopValue }}%</span>
          </div>
          <div class="flex items-center space-x-2">
            <span class="text-sm w-16">{{ range.label }}</span>
            <div class="flex-1 h-4 rounded bg-gray-100 overflow-hidden">
              <div class="h-full"
                   :style="{ 
                     width: range.ipValue + '%',
                     backgroundColor: getEquityColor(range.min) 
                   }"></div>
            </div>
            <span class="text-xs w-8 text-right">{{ range.ipValue }}%</span>
          </div>
        </div>
        <div class="flex justify-end space-x-4 text-xs text-gray-500 mt-2">
          <span>■ OOP</span>
          <span>■ IP</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { useStore, useConfigStore } from "../store";
import { cardText, parseCardString } from "../utils";

// Store references
const store = useStore();
const config = useConfigStore();
const boardText = ref("");
const showDebugInfo = ref(false);

// Card display data derived from config.board
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
  
  // Fill to have at least 3 positions
  while (cards.length < 3) {
    cards.push({ rank: '', suit: '' });
  }
  
  return cards;
});

// Check if card is selected
const isCardSelected = (cardString: string) => {
  const cardId = parseCardString(cardString.toLowerCase());
  return cardId !== null && config.board.includes(cardId);
};

// Toggle card state by string
const toggleCardByString = (cardString: string) => {
  const cardId = parseCardString(cardString.toLowerCase());
  if (cardId === null) return;
  
  toggleCard(cardId);
};

// Toggle card state
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
  
  // Update bunching flop
  if (store.isBunchingEnabled && config.board.length === 3) {
    store.bunchingFlop = [...config.board];
  }
};

// Set text from buttons
const setBoardTextFromButtons = () => {
  boardText.value = config.board
    .map(cardText)
    .map(({ rank, suitLetter }) => rank.toLowerCase() + suitLetter)
    .join(", ");
};

// Handle text change
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
  
  // Update bunching flop
  if (store.isBunchingEnabled && config.board.length === 3) {
    store.bunchingFlop = [...config.board];
  }
};

// Clear board
const clearBoard = () => {
  config.board = [];
  setBoardTextFromButtons();
  
  // Clear bunching flop
  if (store.isBunchingEnabled) {
    store.bunchingFlop = [];
  }
};

// Generate random flop
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
  
  // Update bunching flop
  if (store.isBunchingEnabled) {
    store.bunchingFlop = [...config.board];
  }
};

// Get card color by suit
const getCardColor = (suit: string) => {
  if (suit === '♥' || suit === '♦' || suit === 'h' || suit === 'd') {
    return 'text-pink-600';
  }
  return 'text-gray-900';
};

// Get suit symbol
const getSuitSymbol = (suit: string) => {
  const symbols: Record<string, string> = {
    's': '♠',
    'h': '♥',
    'd': '♦',
    'c': '♣'
  };
  return symbols[suit] || suit;
};

// Debug functions
const toggleDebugInfo = () => {
  // 只有当board有至少3张牌时，才显示debug信息
  if (config.board.length >= 3) {
    showDebugInfo.value = !showDebugInfo.value;
    
    // 如果开启了debug面板，生成分析数据
    if (showDebugInfo.value) {
      updateDebugInfo();
    }
  } else if (config.board.length > 0) {
    // 如果有牌但不到3张，提示用户需要至少3张牌
    alert("Need at least 3 cards on board for detailed analysis.");
  } else {
    // 如果没有牌，提示用户需要先选择牌
    alert("Please select cards on the board first.");
  }
};

// 创建格式化的JSON数据展示
const formattedDebugData = computed(() => {
  if (config.board.length < 3) return "{}";
  
  // 获取所有分析数据
  const boardFeatures = analyzeBoardFeatures();
  
  // 创建一个包含所有计算数据的对象
  const data = {
    board: config.board.map(cardId => {
      const { rank, suit } = cardText(cardId);
      return rank + suit;
    }),
    comboStatistics: {
      remainingCombos: remainingCombos.value,
      blockedByOOP: blockedByOOP.value,
      flushCombos: flushCombos.value,
      straightCombos: straightCombos.value
    },
    boardFeatures: {
      hasPair: boardFeatures.hasPair,
      flushDraw: boardFeatures.flushDraw,
      straightDraw: boardFeatures.straightDraw,
      highCards: boardFeatures.highCards,
      mediumCards: boardFeatures.mediumCards,
      lowCards: boardFeatures.lowCards,
      avgGap: parseFloat(boardFeatures.avgGap.toFixed(1))
    },
    strengthDistribution: {
      oop: {
        veryLow: strengthLevels.value[0].oopValue,
        low: strengthLevels.value[1].oopValue,
        medium: strengthLevels.value[2].oopValue,
        high: strengthLevels.value[3].oopValue,
        veryHigh: strengthLevels.value[4].oopValue
      },
      ip: {
        veryLow: strengthLevels.value[0].ipValue,
        low: strengthLevels.value[1].ipValue,
        medium: strengthLevels.value[2].ipValue,
        high: strengthLevels.value[3].ipValue,
        veryHigh: strengthLevels.value[4].ipValue
      }
    },
    equityDistribution: {
      oop: {
        "0-20%": equityRanges.value[0].oopValue,
        "20-40%": equityRanges.value[1].oopValue,
        "40-60%": equityRanges.value[2].oopValue, 
        "60-80%": equityRanges.value[3].oopValue,
        "80-100%": equityRanges.value[4].oopValue
      },
      ip: {
        "0-20%": equityRanges.value[0].ipValue,
        "20-40%": equityRanges.value[1].ipValue,
        "40-60%": equityRanges.value[2].ipValue,
        "60-80%": equityRanges.value[3].ipValue,
        "80-100%": equityRanges.value[4].ipValue
      }
    }
  };
  
  return JSON.stringify(data, null, 2);
});

// 复制Debug数据到剪贴板
const copyDebugDataToClipboard = () => {
  navigator.clipboard.writeText(formattedDebugData.value).then(() => {
    alert("JSON data copied to clipboard!");
  }).catch(err => {
    console.error("Failed to copy data: ", err);
    alert("Failed to copy data. Please try selecting and copying manually.");
  });
};

const refreshDebugInfo = () => {
  // Force a refresh of the debug data
  config.board = [...config.board];
  updateDebugInfo();
};

const syncBoardData = () => {
  // Sync board data with bunching flop if applicable
  if (store.isBunchingEnabled && config.board.length === 3) {
    store.bunchingFlop = [...config.board];
  }
  updateDebugInfo();
};

const logDebugInfo = () => {
  const debugData = JSON.parse(formattedDebugData.value);
  console.log('Board Analysis Data:', debugData);
};

// 辅助函数：更新Debug信息面板
const updateDebugInfo = () => {
  if (!showDebugInfo.value) return;
  
  // 在formattedDebugData中已经创建了数据对象，不需要重复创建
  console.log("Board Analytics Data Ready");
};

// Statistics data
const remainingCombos = computed(() => {
  // 牌组总数是52张，从中移除已经在board上的牌
  const totalCombos = 1326; // 52 * 51 / 2 (所有可能的2张手牌组合)
  const usedCards = config.board.length;
  
  if (usedCards === 0) return totalCombos;
  
  // 计算剩余组合数
  // 每张board牌会阻断 (52-1) = 51 种可能的手牌组合
  // 注意：这个公式是近似的，更精确的计算需要考虑特定的牌
  return totalCombos - (usedCards * 51) + (usedCards * (usedCards - 1) / 2);
});

const blockedByOOP = computed(() => {
  // 如果没有board牌，返回0
  if (config.board.length === 0) return 0;
  
  // 从store中获取OOP的范围
  const oopRange = store.ranges[store.selectedPlayer === 'ip' ? 'oop' : 'ip'] || [];
  
  // 计算由board阻断的OOP范围组合数
  let blockedCombos = 0;
  const boardCards = new Set(config.board);
  
  // 遍历OOP范围中的每一个手牌组合
  for (let i = 0; i < 13; i++) {
    for (let j = 0; j < 13; j++) {
      const index = i * 13 + j;
      const weight = oopRange[index] || 0;
      
      if (weight > 0) {
        // 计算这个组合在当前board上被阻断的可能性
        // 这是一个简化的计算，实际上应该检查具体的牌
        const rankI = Math.floor(i / 4);
        const rankJ = Math.floor(j / 4);
        const suitI = i % 4;
        const suitJ = j % 4;
        
        // 检查board上是否有阻断这个组合的牌
        const blockProbability = Array.from(boardCards).reduce((acc, card) => {
          const cardRank = Math.floor(card / 4);
          const cardSuit = card % 4;
          
          // 如果board上的牌与这个组合的牌相匹配
          if ((cardRank === rankI && cardSuit === suitI) || 
              (cardRank === rankJ && cardSuit === suitJ)) {
            return acc + 1;
          }
          return acc;
        }, 0) / boardCards.size;
        
        blockedCombos += weight * blockProbability;
      }
    }
  }
  
  return Math.round(blockedCombos);
});

const flushCombos = computed(() => {
  if (config.board.length < 2) return 0;
  
  // 计算board上的同花可能性
  // 统计各个花色的数量
  const suitCounts = { 0: 0, 1: 0, 2: 0, 3: 0 }; // spades, hearts, diamonds, clubs
  
  config.board.forEach(card => {
    const suit = card % 4;
    suitCounts[suit]++;
  });
  
  // 计算同花组合的可能性
  let flushPossibilities = 0;
  
  for (const suit in suitCounts) {
    const count = suitCounts[suit];
    
    // 如果board上已经有3张或更多同花，计算形成同花的组合数
    if (count >= 3) {
      const remainingCardsOfSuit = 13 - count; // 同花色的剩余牌
      flushPossibilities += remainingCardsOfSuit * (remainingCardsOfSuit - 1) / 2;
    }
    // 如果board上有2张同花，计算形成同花听牌的组合数
    else if (count === 2) {
      const remainingCardsOfSuit = 13 - count;
      flushPossibilities += remainingCardsOfSuit;
    }
  }
  
  return flushPossibilities;
});

const straightCombos = computed(() => {
  if (config.board.length < 3) return 0;
  
  // 计算顺子可能性
  // 提取board上的牌的点数(去重)
  const ranks = [...new Set(config.board.map(card => Math.floor(card / 4)))];
  ranks.sort((a, b) => a - b);
  
  // 检查是否有顺子或顺子听牌的可能
  let straightPossibilities = 0;
  
  // 构建点数分布图
  const rankDist = Array(13).fill(0);
  ranks.forEach(rank => rankDist[rank] = 1);
  
  // A也可以作为1使用
  if (rankDist[12] === 1) {
    rankDist[0] = 1;
  }
  
  // 检查所有可能的5连牌
  for (let i = 0; i <= 9; i++) {
    const window = rankDist.slice(i, i + 5);
    const windowSum = window.reduce((sum, val) => sum + val, 0);
    
    // 如果已经有3张或更多连续牌
    if (windowSum >= 3) {
      // 计算需要补齐的牌的数量
      const neededCards = 5 - windowSum;
      
      // 计算完成顺子的可能组合数
      let possibilities = 1;
      for (let j = 0; j < 5; j++) {
        if (window[j] === 0) {
          // 每个缺失的点数有4张牌
          possibilities *= 4;
        }
      }
      
      // 如果需要的牌少于等于2张，则属于顺子组合
      if (neededCards <= 2) {
        straightPossibilities += possibilities;
      }
    }
  }
  
  return straightPossibilities;
});

// Board strength levels data
const strengthLevels = computed(() => {
  // 如果没有board牌，返回默认值
  if (config.board.length < 3) {
    return [
      { name: 'Very Low', oopValue: 20, ipValue: 20 },
      { name: 'Low', oopValue: 20, ipValue: 20 },
      { name: 'Medium', oopValue: 20, ipValue: 20 },
      { name: 'High', oopValue: 20, ipValue: 20 },
      { name: 'Very High', oopValue: 20, ipValue: 20 }
    ];
  }
  
  // 分析牌面特征
  const boardFeatures = analyzeBoardFeatures();
  
  // 获取两方的范围
  const oopRange = store.ranges.oop || [];
  const ipRange = store.ranges.ip || [];
  
  // 计算各个强度水平的分布
  const distribution = calculateStrengthDistribution(boardFeatures, oopRange, ipRange);
  
  return [
    { name: 'Very Low', oopValue: distribution.oop.veryLow, ipValue: distribution.ip.veryLow },
    { name: 'Low', oopValue: distribution.oop.low, ipValue: distribution.ip.low },
    { name: 'Medium', oopValue: distribution.oop.medium, ipValue: distribution.ip.medium },
    { name: 'High', oopValue: distribution.oop.high, ipValue: distribution.ip.high },
    { name: 'Very High', oopValue: distribution.oop.veryHigh, ipValue: distribution.ip.veryHigh }
  ];
});

// 分析牌面特征
const analyzeBoardFeatures = () => {
  const board = [...config.board].sort((a, b) => Math.floor(b / 4) - Math.floor(a / 4));
  
  // 提取点数和花色
  const ranks = board.map(card => Math.floor(card / 4));
  const suits = board.map(card => card % 4);
  
  // 检查是否有对子
  const hasPair = new Set(ranks).size < ranks.length;
  
  // 检查是否有同花的可能
  const suitCounts = suits.reduce((acc, suit) => {
    acc[suit] = (acc[suit] || 0) + 1;
    return acc;
  }, {});
  const flushDraw = Object.values(suitCounts).some(count => count >= 3);
  
  // 检查是否有顺子的可能
  const uniqueRanks = [...new Set(ranks)].sort((a, b) => a - b);
  let straightDraw = false;
  
  // 构建点数分布图
  const rankDist = Array(13).fill(0);
  uniqueRanks.forEach(rank => rankDist[rank] = 1);
  
  // A也可以作为1使用
  if (rankDist[12] === 1) {
    rankDist[0] = 1;
  }
  
  // 检查连续点数
  for (let i = 0; i <= 9; i++) {
    const window = rankDist.slice(i, i + 5);
    const consecutive = window.reduce((sum, val) => sum + val, 0);
    if (consecutive >= 3) {
      straightDraw = true;
      break;
    }
  }
  
  // 检查牌面点数的分布
  const highCards = ranks.filter(rank => rank >= 8).length; // T, J, Q, K, A
  const mediumCards = ranks.filter(rank => rank >= 5 && rank < 8).length; // 7, 8, 9
  const lowCards = ranks.filter(rank => rank < 5).length; // 2, 3, 4, 5, 6
  
  // 检查牌面点数的距离
  const rankGaps = [];
  for (let i = 1; i < uniqueRanks.length; i++) {
    rankGaps.push(uniqueRanks[i] - uniqueRanks[i - 1]);
  }
  const avgGap = rankGaps.length > 0 ? rankGaps.reduce((sum, gap) => sum + gap, 0) / rankGaps.length : 0;
  
  return {
    hasPair,
    flushDraw,
    straightDraw,
    highCards,
    mediumCards,
    lowCards,
    avgGap,
    highestRank: ranks[0]
  };
};

// 计算牌面强度分布
const calculateStrengthDistribution = (features, oopRange, ipRange) => {
  // 根据牌面特征估算各个强度等级的分布
  
  // 基础分布 - 随机牌面的典型分布
  const baseDistribution = {
    veryLow: 20,   // 底对/弱高牌/抽牌
    low: 20,       // 中对/弱抽牌
    medium: 20,    // 顶对弱踢脚/强抽牌
    high: 20,      // 顶对好踢脚/两对
    veryHigh: 20   // 三条及以上
  };
  
  // 根据牌面特征调整分布
  let adjustedOOP = { ...baseDistribution };
  let adjustedIP = { ...baseDistribution };
  
  // 调整系数
  const adjustFactors = {
    pair: { veryLow: -5, low: -5, medium: 0, high: +5, veryHigh: +5 },
    flushDraw: { veryLow: +5, low: +5, medium: 0, high: -5, veryHigh: -5 },
    straightDraw: { veryLow: +5, low: +5, medium: 0, high: -5, veryHigh: -5 },
    highCards: { veryLow: -3, low: -2, medium: +2, high: +2, veryHigh: +1 },
    mediumCards: { veryLow: 0, low: +2, medium: +2, high: 0, veryHigh: -4 },
    lowCards: { veryLow: +3, low: +2, medium: 0, high: -2, veryHigh: -3 },
    wideGap: { veryLow: +3, low: +2, medium: 0, high: -2, veryHigh: -3 }
  };
  
  // 应用特征调整
  if (features.hasPair) {
    for (const level in adjustedOOP) {
      adjustedOOP[level] += adjustFactors.pair[level];
      adjustedIP[level] += adjustFactors.pair[level];
    }
  }
  
  if (features.flushDraw) {
    for (const level in adjustedOOP) {
      adjustedOOP[level] += adjustFactors.flushDraw[level];
      adjustedIP[level] += adjustFactors.flushDraw[level];
    }
  }
  
  if (features.straightDraw) {
    for (const level in adjustedOOP) {
      adjustedOOP[level] += adjustFactors.straightDraw[level];
      adjustedIP[level] += adjustFactors.straightDraw[level];
    }
  }
  
  // 点数分布的影响
  const highCardAdjust = features.highCards * adjustFactors.highCards.high;
  const mediumCardAdjust = features.mediumCards * adjustFactors.mediumCards.medium;
  const lowCardAdjust = features.lowCards * adjustFactors.lowCards.low;
  
  adjustedOOP.high += highCardAdjust;
  adjustedOOP.medium += mediumCardAdjust;
  adjustedOOP.low += lowCardAdjust;
  
  adjustedIP.high += highCardAdjust;
  adjustedIP.medium += mediumCardAdjust;
  adjustedIP.low += lowCardAdjust;
  
  // 牌间距离的影响
  if (features.avgGap > 2) {
    for (const level in adjustedOOP) {
      adjustedOOP[level] += adjustFactors.wideGap[level];
      adjustedIP[level] += adjustFactors.wideGap[level];
    }
  }
  
  // 考虑范围与牌面的交互
  // 高牌密集的牌面对包含高牌多的范围更有利
  // 这里假设IP范围通常比OOP范围更强
  // 估算范围牌力
  const oopRangeStrength = estimateRangeStrength(oopRange);
  const ipRangeStrength = estimateRangeStrength(ipRange);
  
  // 位置优势调整 - 更合理的实现
  // IP位置通常对高牌类型更有利，而OOP对连牌、同花类型有更好的实现
  
  // 基础位置优势调整
  const positionAdjustment = {
    oop: { veryLow: +2, low: +1, medium: 0, high: -1, veryHigh: -2 },
    ip: { veryLow: -2, low: -1, medium: 0, high: +1, veryHigh: +2 }
  };
  
  // 应用基础位置优势调整
  for (const level in adjustedOOP) {
    adjustedOOP[level] += positionAdjustment.oop[level];
    adjustedIP[level] += positionAdjustment.ip[level];
  }
  
  // 根据特定牌面特征与范围交互调整位置优势
  if (features.highCards >= 2 && ipRangeStrength.highCards > oopRangeStrength.highCards) {
    // IP范围中的高牌在高牌牌面上有更多优势
    adjustedIP.high += 3;
    adjustedIP.veryHigh += 2;
    adjustedOOP.high -= 1;
  }
  
  if (features.straightDraw && oopRangeStrength.connectors > ipRangeStrength.connectors) {
    // OOP范围中的连牌在连牌牌面上有更多优势
    adjustedOOP.medium += 2;
    adjustedOOP.high += 1;
    adjustedIP.medium -= 1;
  }
  
  // 确保值在合理范围内并总和为100
  const normalizeDistribution = (dist) => {
    let total = 0;
    for (const level in dist) {
      dist[level] = Math.max(5, dist[level]); // 确保每个等级至少有5%
      total += dist[level];
    }
    
    for (const level in dist) {
      dist[level] = Math.round(dist[level] / total * 100);
    }
    
    // 确保总和为100
    let adjustedTotal = 0;
    for (const level in dist) {
      adjustedTotal += dist[level];
    }
    
    if (adjustedTotal !== 100) {
      // 找到最大的值进行调整
      const levels = Object.keys(dist);
      const maxLevel = levels.reduce((a, b) => dist[a] > dist[b] ? a : b);
      dist[maxLevel] += (100 - adjustedTotal);
    }
    
    return dist;
  };
  
  return {
    oop: normalizeDistribution(adjustedOOP),
    ip: normalizeDistribution(adjustedIP)
  };
};

// 估算范围强度
const estimateRangeStrength = (range) => {
  if (!range || range.length === 0) {
    return { overall: 0, highCards: 0, pairs: 0, connectors: 0 };
  }
  
  let highCards = 0;
  let pairs = 0;
  let connectors = 0;
  let total = 0;
  
  for (let i = 0; i < 13; i++) {
    for (let j = 0; j < 13; j++) {
      const index = i * 13 + j;
      const weight = range[index] || 0;
      
      if (weight > 0) {
        total += weight;
        
        // 检查是否是对子
        if (i === j) {
          pairs += weight;
          
          // 高对
          if (i <= 4) { // AT+
            highCards += weight * 1.5;
          }
        }
        // 检查是否包含高牌
        else if (i <= 4 || j <= 4) { // A, K, Q, J, T
          highCards += weight;
        }
        
        // 检查是否是连牌
        if (Math.abs(i - j) === 1 || Math.abs(i - j) === 2) {
          connectors += weight;
        }
      }
    }
  }
  
  return {
    overall: total > 0 ? (highCards * 0.6 + pairs * 0.3 + connectors * 0.1) / total : 0,
    highCards: total > 0 ? highCards / total : 0,
    pairs: total > 0 ? pairs / total : 0,
    connectors: total > 0 ? connectors / total : 0
  };
};

// Equity distribution data
const equityRanges = computed(() => {
  // 如果没有board牌，返回默认分布
  if (config.board.length < 3) {
    return [
      { label: '0-20%', min: 0, max: 20, oopValue: 20, ipValue: 20 },
      { label: '20-40%', min: 20, max: 40, oopValue: 20, ipValue: 20 },
      { label: '40-60%', min: 40, max: 60, oopValue: 20, ipValue: 20 },
      { label: '60-80%', min: 60, max: 80, oopValue: 20, ipValue: 20 },
      { label: '80-100%', min: 80, max: 100, oopValue: 20, ipValue: 20 }
    ];
  }
  
  // 获取两方的范围
  const oopRange = store.ranges.oop || [];
  const ipRange = store.ranges.ip || [];
  
  // 分析牌面特征
  const boardFeatures = analyzeBoardFeatures();
  
  // 计算权益分布
  const distribution = calculateEquityDistribution(boardFeatures, oopRange, ipRange);
  
  return [
    { label: '0-20%', min: 0, max: 20, oopValue: distribution.oop[0], ipValue: distribution.ip[0] },
    { label: '20-40%', min: 20, max: 40, oopValue: distribution.oop[1], ipValue: distribution.ip[1] },
    { label: '40-60%', min: 40, max: 60, oopValue: distribution.oop[2], ipValue: distribution.ip[2] },
    { label: '60-80%', min: 60, max: 80, oopValue: distribution.oop[3], ipValue: distribution.ip[3] },
    { label: '80-100%', min: 80, max: 100, oopValue: distribution.oop[4], ipValue: distribution.ip[4] }
  ];
});

// 计算权益分布
const calculateEquityDistribution = (boardFeatures, oopRange, ipRange) => {
  // 定义各个手牌类型在不同牌面下的权益分布模式
  const equityPatterns = {
    // [0-20%, 20-40%, 40-60%, 60-80%, 80-100%]
    
    // 对子牌面
    pairBoard: {
      // 手牌命中三条 = 高权益
      trips: [5, 10, 20, 30, 35],
      // 顶对好踢脚 = 中高权益
      topPairGoodKicker: [5, 15, 25, 35, 20],
      // 顶对差踢脚 = 中权益
      topPairBadKicker: [10, 20, 35, 25, 10],
      // 中对 = 中低权益
      middlePair: [15, 30, 30, 20, 5],
      // 底对 = 低权益
      bottomPair: [30, 35, 25, 7, 3],
      // 高牌 = 低权益
      highCard: [40, 35, 15, 8, 2]
    },
    
    // 连接牌面
    connectedBoard: {
      // 顺子 = 高权益
      straight: [5, 10, 20, 30, 35],
      // 顺子听牌 = 中权益
      straightDraw: [15, 25, 35, 20, 5],
      // 对子 = 中权益
      pair: [15, 25, 35, 20, 5],
      // 高牌 = 低权益
      highCard: [35, 30, 25, 8, 2]
    },
    
    // 同花可能牌面
    flushPossibleBoard: {
      // 同花 = 高权益
      flush: [5, 10, 20, 30, 35],
      // 同花听牌 = 中权益
      flushDraw: [15, 25, 35, 20, 5],
      // 对子 = 中权益
      pair: [15, 25, 35, 20, 5],
      // 高牌 = 低权益
      highCard: [35, 30, 25, 8, 2]
    },
    
    // 高牌牌面
    highCardBoard: {
      // 对子 = 中高权益
      pair: [10, 20, 30, 25, 15],
      // 高牌Ace = 中权益
      highCardAce: [20, 25, 35, 15, 5],
      // 高牌King = 中低权益
      highCardKing: [25, 30, 30, 10, 5],
      // 中低牌 = 低权益
      lowCard: [40, 35, 15, 8, 2]
    },
    
    // 高中低混合牌面 (如Q72这种)
    mixedBoard: {
      // 顶对 = 较高权益
      topPairGoodKicker: [5, 15, 30, 35, 15],
      topPairBadKicker: [10, 20, 35, 25, 10],
      // 中对 = 中等权益
      middlePair: [15, 25, 35, 20, 5],
      // 底对 = 较低权益
      bottomPair: [25, 35, 30, 8, 2],
      // 高牌A = 中低权益
      highCardAce: [25, 30, 30, 10, 5],
      // 其他高牌 = 低权益
      highCardKing: [30, 35, 25, 8, 2],
      // 低牌 = 很低权益
      lowCard: [45, 35, 15, 4, 1]
    }
  };
  
  // 根据牌面特征确定使用哪种模式
  let boardType;
  let mainPattern;
  
  if (boardFeatures.hasPair) {
    boardType = 'pairBoard';
  } else if (boardFeatures.straightDraw) {
    boardType = 'connectedBoard';
  } else if (boardFeatures.flushDraw) {
    boardType = 'flushPossibleBoard';
  } else if (boardFeatures.highCards === 1 && boardFeatures.mediumCards === 1 && boardFeatures.lowCards === 1) {
    // 像Q72这样的分散牌面 - 一高一中一低
    boardType = 'mixedBoard';
  } else {
    boardType = 'highCardBoard';
  }
  
  mainPattern = equityPatterns[boardType];
  
  // 估算各种手牌类型在范围中的比例
  const oopHandTypes = estimateHandTypes(oopRange, boardFeatures);
  const ipHandTypes = estimateHandTypes(ipRange, boardFeatures);
  
  // 计算OOP和IP的权益分布
  const oopDistribution = calculateRangeDistribution(oopHandTypes, mainPattern, boardType);
  const ipDistribution = calculateRangeDistribution(ipHandTypes, mainPattern, boardType);
  
  // 考虑位置优势 - 更合理的调整方式
  // 不同牌面类型的位置优势调整系数
  const positionAdjustment = {
    pairBoard: {
      oop: [-2, -1, 0, 1, 2],
      ip: [2, 1, 0, -1, -2]
    },
    connectedBoard: {
      oop: [-1, -1, 0, 1, 1],
      ip: [1, 1, 0, -1, -1]
    },
    flushPossibleBoard: {
      oop: [-1, -1, 0, 1, 1],
      ip: [1, 1, 0, -1, -1]
    },
    highCardBoard: {
      oop: [-3, -2, 0, 2, 3],
      ip: [3, 2, 0, -2, -3]
    },
    mixedBoard: {
      oop: [-3, -2, 0, 2, 3],
      ip: [3, 2, 0, -2, -3]
    }
  };
  
  // 应用位置优势调整
  const currentAdjustment = positionAdjustment[boardType];
  for (let i = 0; i < 5; i++) {
    oopDistribution[i] += currentAdjustment.oop[i];
    ipDistribution[i] += currentAdjustment.ip[i];
  }
  
  // 平衡OOP和IP的分布 - 确保两者的中位数（第三个区间）大致相同
  const balanceDistributions = (oopDist, ipDist) => {
    // 计算中位数区域的差异
    const midDiff = oopDist[2] - ipDist[2];
    
    // 如果差异较大，进行调整
    if (Math.abs(midDiff) > 10) {
      const adjustment = Math.sign(midDiff) * Math.min(5, Math.abs(midDiff) / 2);
      oopDist[2] -= adjustment;
      ipDist[2] += adjustment;
      
      // 调整其他区域保持平衡
      if (adjustment > 0) {
        // OOP向下调整，IP向上调整
        oopDist[1] += adjustment / 2;
        oopDist[3] += adjustment / 2;
        ipDist[1] -= adjustment / 2;
        ipDist[3] -= adjustment / 2;
      } else {
        // OOP向上调整，IP向下调整
        oopDist[1] -= adjustment / 2;
        oopDist[3] -= adjustment / 2;
        ipDist[1] += adjustment / 2;
        ipDist[3] += adjustment / 2;
      }
    }
    
    return { oopDist, ipDist };
  };
  
  // 应用平衡
  const balanced = balanceDistributions([...oopDistribution], [...ipDistribution]);
  const oopBalanced = balanced.oopDist;
  const ipBalanced = balanced.ipDist;
  
  // 确保分布总和为100且值合理
  const normalizeDistribution = (dist) => {
    // 确保没有负值且值不会过低
    dist = dist.map(v => Math.max(5, v));
    
    // 计算总和
    const total = dist.reduce((sum, v) => sum + v, 0);
    
    // 归一化到总和为100
    if (total > 0) {
      dist = dist.map(v => Math.round(v / total * 100));
    } else {
      dist = [20, 20, 20, 20, 20]; // 默认平均分布
    }
    
    // 确保总和为100
    let adjustedTotal = dist.reduce((sum, v) => sum + v, 0);
    if (adjustedTotal !== 100) {
      // 找到最大的值进行调整，这样不会破坏小值
      const maxIndex = dist.indexOf(Math.max(...dist));
      dist[maxIndex] += (100 - adjustedTotal);
    }
    
    return dist;
  };
  
  return {
    oop: normalizeDistribution(oopBalanced),
    ip: normalizeDistribution(ipBalanced)
  };
};

// 估算范围中各种手牌类型的比例
const estimateHandTypes = (range, boardFeatures) => {
  if (!range || range.length === 0) {
    return {
      trips: 0,
      topPairGoodKicker: 0,
      topPairBadKicker: 0,
      middlePair: 0,
      bottomPair: 0,
      straight: 0,
      straightDraw: 0,
      flush: 0,
      flushDraw: 0,
      pair: 0,
      highCardAce: 0,
      highCardKing: 0,
      lowCard: 0
    };
  }
  
  // 提取board的点数和花色
  const board = [...config.board].sort((a, b) => Math.floor(b / 4) - Math.floor(a / 4));
  const boardRanks = board.map(card => Math.floor(card / 4));
  const boardSuits = board.map(card => card % 4);
  const topBoardRank = boardRanks[0]; // 牌面上最高的牌
  const secondBoardRank = boardRanks.length > 1 ? boardRanks[1] : -1;
  const thirdBoardRank = boardRanks.length > 2 ? boardRanks[2] : -1;
  
  // 计算不同类型手牌的权重
  let handTypes = {
    trips: 0,
    topPairGoodKicker: 0,
    topPairBadKicker: 0,
    middlePair: 0,
    bottomPair: 0,
    straight: 0,
    straightDraw: 0,
    flush: 0,
    flushDraw: 0,
    pair: 0,
    highCardAce: 0,
    highCardKing: 0,
    lowCard: 0
  };
  
  let total = 0;
  
  // 主要花色 (用于估计同花可能性)
  const suitCounts = {};
  boardSuits.forEach(suit => {
    suitCounts[suit] = (suitCounts[suit] || 0) + 1;
  });
  
  let dominantSuit = -1;
  let maxSuitCount = 0;
  
  for (const suit in suitCounts) {
    if (suitCounts[suit] > maxSuitCount) {
      maxSuitCount = suitCounts[suit];
      dominantSuit = Number(suit);
    }
  }
  
  // 检查顺子可能性
  const straightPossibilities = [];
  
  if (boardFeatures.straightDraw) {
    // 将牌面点数按升序排列
    const sortedRanks = [...boardRanks].sort((a, b) => a - b);
    
    // 检查各种可能的顺子组合
    for (let start = 0; start <= 9; start++) {
      const window = Array(5).fill(0);
      
      // 标记牌面上已有的点数
      sortedRanks.forEach(rank => {
        if (rank >= start && rank < start + 5) {
          window[rank - start] = 1;
        }
      });
      
      // 如果窗口中有至少2个点数，记录为可能的顺子
      const sum = window.reduce((a, b) => a + b, 0);
      if (sum >= 2) {
        straightPossibilities.push({
          start,
          window,
          neededCards: 5 - sum
        });
      }
    }
    
    // 考虑A作为1的特殊情况
    if (boardRanks.includes(12)) { // A
      const window = Array(5).fill(0);
      window[0] = 1; // A
      
      // 检查是否有2, 3, 4, 5
      for (let rank = 0; rank <= 3; rank++) {
        if (boardRanks.includes(rank)) {
          window[rank + 1] = 1;
        }
      }
      
      const sum = window.reduce((a, b) => a + b, 0);
      if (sum >= 2) {
        straightPossibilities.push({
          start: -1, // 表示A作为1
          window,
          neededCards: 5 - sum
        });
      }
    }
  }
  
  // 迭代范围中的每个手牌
  for (let i = 0; i < 13; i++) {
    for (let j = 0; j <= i; j++) {
      // 由于我们只考虑半个矩阵（因为AA和AA是相同的），
      // 我们需要正确地计算权重
      let weight = 0;
      
      if (i === j) {
        // 对子只有一个索引
        const index = i * 13 + j;
        weight = range[index] || 0;
      } else {
        // 非对子有两个索引
        const index1 = i * 13 + j;
        const index2 = j * 13 + i;
        weight = (range[index1] || 0) + (range[index2] || 0);
      }
      
      if (weight > 0) {
        total += weight;
        
        // 计算这个手牌与牌面的交互
        const cardRanks = [i, j].sort((a, b) => b - a);
        const highCardRank = cardRanks[0];
        const lowCardRank = cardRanks[1];
        
        // 检查对子
        const isPocketPair = i === j;
        
        // 检查是否能形成三条
        if (boardFeatures.hasPair) {
          // 检查牌面对子的点数
          const pairRank = boardRanks.find(rank => 
            boardRanks.filter(r => r === rank).length >= 2);
            
          if (pairRank !== undefined && (highCardRank === pairRank || lowCardRank === pairRank)) {
            handTypes.trips += weight;
            continue; // 已分类为三条，跳过后续检查
          }
        }
        
        // 检查是否形成顶对
        if (highCardRank === topBoardRank || lowCardRank === topBoardRank) {
          // 检查踢脚质量
          const kicker = highCardRank === topBoardRank ? lowCardRank : highCardRank;
          
          if (kicker >= 8) { // T+
            handTypes.topPairGoodKicker += weight;
          } else {
            handTypes.topPairBadKicker += weight;
          }
          continue; // 已分类为顶对，跳过后续检查
        }
        
        // 检查中对和底对
        if (isPocketPair) {
          const pairRank = i;
          
          if (pairRank >= secondBoardRank && secondBoardRank !== -1) {
            // 比第二高的牌大
            handTypes.topPairGoodKicker += weight;
          } else if (pairRank >= 8) { // T+
            handTypes.middlePair += weight;
          } else if (pairRank >= 5) { // 7-9
            handTypes.middlePair += weight;
          } else { // 2-6
            handTypes.bottomPair += weight;
          }
          continue; // 已分类为某种对子，跳过后续检查
        }
        
        // 检查是否形成顺子或顺子听牌
        let isStraigtDraw = false;
        
        if (boardFeatures.straightDraw && straightPossibilities.length > 0) {
          for (const possibility of straightPossibilities) {
            // 检查手牌是否能补全顺子
            const { start, window, neededCards } = possibility;
            let handHelps = 0;
            
            // 检查每张手牌是否能补全顺子
            for (const rank of cardRanks) {
              // 处理A作为1的特殊情况
              const adjustedRank = start === -1 && rank === 12 ? 0 : rank;
              
              if (start === -1) {
                // A-5顺子
                if (adjustedRank >= 0 && adjustedRank <= 4 && window[adjustedRank] === 0) {
                  handHelps++;
                  window[adjustedRank] = 1; // 标记为已使用
                }
              } else if (rank >= start && rank < start + 5 && window[rank - start] === 0) {
                handHelps++;
                window[rank - start] = 1; // 标记为已使用
              }
            }
            
            // 如果手牌能补齐顺子所需的牌
            if (handHelps > 0 && handHelps + (5 - neededCards) >= 5) {
              handTypes.straight += weight;
              isStraigtDraw = true;
              break;
            } else if (handHelps > 0) {
              handTypes.straightDraw += weight;
              isStraigtDraw = true;
              break;
            }
          }
          
          if (isStraigtDraw) continue; // 已分类为顺子或顺子听牌，跳过后续检查
        }
        
        // 检查是否形成同花或同花听牌
        if (boardFeatures.flushDraw && dominantSuit !== -1 && maxSuitCount >= 3) {
          // 检查手牌是否与主要花色匹配
          const cardSuits = [i % 4, j % 4];
          
          if (cardSuits.includes(dominantSuit)) {
            // 如果牌面上已经有4张同花
            if (maxSuitCount === 4) {
              handTypes.flush += weight;
            } else {
              handTypes.flushDraw += weight;
            }
            continue; // 已分类为同花或同花听牌，跳过后续检查
          }
        }
        
        // 如果还未分类，检查高牌类型
        if (highCardRank === 12) { // A
          handTypes.highCardAce += weight;
        } else if (highCardRank === 11) { // K
          handTypes.highCardKing += weight;
        } else if (highCardRank >= 8) { // T, J, Q
          handTypes.highCardKing += weight * 0.7;
          handTypes.lowCard += weight * 0.3;
        } else {
          handTypes.lowCard += weight;
        }
      }
    }
  }
  
  // 归一化
  if (total > 0) {
    for (const type in handTypes) {
      handTypes[type] = handTypes[type] / total;
    }
  }
  
  return handTypes;
};

// 计算范围的权益分布
const calculateRangeDistribution = (handTypes, patterns, boardType) => {
  // 初始化分布 - 默认基线分布更接近正态分布
  const distribution = [20, 25, 30, 15, 10];
  
  // 根据不同牌面类型应用相应的计算
  if (boardType === 'pairBoard') {
    for (let i = 0; i < 5; i++) {
      // 重置为基础比例
      distribution[i] = 0;
      
      // 根据各手牌类型的比例和其权益分布模式计算
      distribution[i] += handTypes.trips * patterns.trips[i];
      distribution[i] += handTypes.topPairGoodKicker * patterns.topPairGoodKicker[i];
      distribution[i] += handTypes.topPairBadKicker * patterns.topPairBadKicker[i];
      distribution[i] += handTypes.middlePair * patterns.middlePair[i];
      distribution[i] += handTypes.bottomPair * patterns.bottomPair[i];
      distribution[i] += handTypes.highCardAce * patterns.highCard[i];
      distribution[i] += handTypes.highCardKing * patterns.highCard[i];
      distribution[i] += handTypes.lowCard * patterns.highCard[i];
    }
  } else if (boardType === 'connectedBoard') {
    for (let i = 0; i < 5; i++) {
      // 重置为基础比例
      distribution[i] = 0;
      
      distribution[i] += handTypes.straight * patterns.straight[i];
      distribution[i] += handTypes.straightDraw * patterns.straightDraw[i];
      distribution[i] += handTypes.pair * patterns.pair[i];
      distribution[i] += handTypes.highCardAce * patterns.highCard[i];
      distribution[i] += handTypes.highCardKing * patterns.highCard[i];
      distribution[i] += handTypes.lowCard * patterns.highCard[i];
    }
  } else if (boardType === 'flushPossibleBoard') {
    for (let i = 0; i < 5; i++) {
      // 重置为基础比例
      distribution[i] = 0;
      
      distribution[i] += handTypes.flush * patterns.flush[i];
      distribution[i] += handTypes.flushDraw * patterns.flushDraw[i];
      distribution[i] += handTypes.pair * patterns.pair[i];
      distribution[i] += handTypes.highCardAce * patterns.highCard[i];
      distribution[i] += handTypes.highCardKing * patterns.highCard[i];
      distribution[i] += handTypes.lowCard * patterns.highCard[i];
    }
  } else if (boardType === 'mixedBoard') {
    for (let i = 0; i < 5; i++) {
      // 重置为基础比例
      distribution[i] = 0;
      
      distribution[i] += handTypes.topPairGoodKicker * patterns.topPairGoodKicker[i];
      distribution[i] += handTypes.topPairBadKicker * patterns.topPairBadKicker[i];
      distribution[i] += handTypes.middlePair * patterns.middlePair[i];
      distribution[i] += handTypes.bottomPair * patterns.bottomPair[i];
      distribution[i] += handTypes.highCardAce * patterns.highCardAce[i];
      distribution[i] += handTypes.highCardKing * patterns.highCardKing[i];
      distribution[i] += handTypes.lowCard * patterns.lowCard[i];
    }
  } else {
    // highCardBoard
    for (let i = 0; i < 5; i++) {
      // 重置为基础比例
      distribution[i] = 0;
      
      distribution[i] += handTypes.pair * patterns.pair[i];
      distribution[i] += handTypes.highCardAce * patterns.highCardAce[i];
      distribution[i] += handTypes.highCardKing * patterns.highCardKing[i];
      distribution[i] += handTypes.lowCard * patterns.lowCard[i];
    }
  }
  
  // 如果没有足够的数据计算分布，使用默认分布
  const sum = distribution.reduce((a, b) => a + b, 0);
  if (sum < 1) {
    // 根据牌面类型使用不同的默认分布
    if (boardType === 'mixedBoard') {
      return [30, 25, 20, 15, 10]; // 混合牌面默认偏向低权益
    } else {
      return [20, 25, 30, 15, 10]; // 标准默认分布
    }
  }
  
  return distribution;
};

// Get color for equity range
const getEquityColor = (minValue: number) => {
  if (minValue >= 80) return '#34d399'; // Green for high equity
  if (minValue >= 60) return '#60a5fa'; // Blue for medium-high
  if (minValue >= 40) return '#a78bfa'; // Purple for medium
  if (minValue >= 20) return '#f59e0b'; // Orange for medium-low
  return '#ef4444'; // Red for low equity
};
</script>

<style scoped>
.grid-cols-13 {
  grid-template-columns: repeat(13, minmax(0, 1fr));
}

.button-base {
  @apply rounded-lg transition-colors;
}

.button-blue {
  @apply bg-blue-500 hover:bg-blue-600 text-white;
}

.button-yellow {
  @apply bg-yellow-500 hover:bg-yellow-600 text-white;
}
</style>
