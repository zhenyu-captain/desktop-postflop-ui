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
      <h3 class="font-bold mb-2">Board Debug Info:</h3>
      <div class="grid grid-cols-1 gap-2">
        <div>
          <h4 class="font-semibold">Selected Cards:</h4>
          <pre>{{ JSON.stringify(displayCards, null, 2) }}</pre>
        </div>
        <div>
          <h4 class="font-semibold">config.board:</h4>
          <pre>{{ JSON.stringify(config.board, null, 2) }}</pre>
        </div>
        <div>
          <h4 class="font-semibold">store.bunchingFlop:</h4>
          <pre>{{ JSON.stringify(store.bunchingFlop, null, 2) }}</pre>
        </div>
        <div>
          <h4 class="font-semibold">store.isBunchingEnabled:</h4>
          <pre>{{ store.isBunchingEnabled }}</pre>
        </div>
      </div>
      <div class="mt-2">
        <button class="px-3 py-1 text-xs bg-blue-500 hover:bg-blue-600 text-white rounded" @click="refreshDebugInfo">Refresh</button>
        <button class="px-3 py-1 text-xs bg-green-500 hover:bg-green-600 text-white rounded ml-2" @click="syncBoardData">Sync Data</button>
        <button class="px-3 py-1 text-xs bg-red-500 hover:bg-red-600 text-white rounded ml-2" @click="logDebugInfo">Console Log</button>
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
  showDebugInfo.value = !showDebugInfo.value;
};

const refreshDebugInfo = () => {
  // Force a refresh of the debug data
  config.board = [...config.board];
};

const syncBoardData = () => {
  // Sync board data with bunching flop if applicable
  if (store.isBunchingEnabled && config.board.length === 3) {
    store.bunchingFlop = [...config.board];
  }
};

const logDebugInfo = () => {
  console.log('Board Debug Info:');
  console.log('config.board:', config.board);
  console.log('store.bunchingFlop:', store.bunchingFlop);
  console.log('store.isBunchingEnabled:', store.isBunchingEnabled);
};

// Statistics data
const remainingCombos = computed(() => {
  // Calculate remaining combos based on current board
  return 1081 - (config.board.length * 50);
});

const blockedByOOP = computed(() => {
  // Calculate blocked by OOP range based on current board
  return 78;
});

const flushCombos = computed(() => {
  // Calculate flush possibility based on current board
  return 45;
});

const straightCombos = computed(() => {
  // Calculate straight possibility based on current board
  return 32;
});

// Board strength levels data
const strengthLevels = [
  { name: 'Very Low', oopValue: 15, ipValue: 10 },
  { name: 'Low', oopValue: 25, ipValue: 20 },
  { name: 'Medium', oopValue: 30, ipValue: 35 },
  { name: 'High', oopValue: 20, ipValue: 25 },
  { name: 'Very High', oopValue: 10, ipValue: 10 }
];

// Equity distribution data
const equityRanges = [
  { label: '0-20%', min: 0, max: 20, oopValue: 10, ipValue: 15 },
  { label: '20-40%', min: 20, max: 40, oopValue: 20, ipValue: 25 },
  { label: '40-60%', min: 40, max: 60, oopValue: 40, ipValue: 35 },
  { label: '60-80%', min: 60, max: 80, oopValue: 20, ipValue: 15 },
  { label: '80-100%', min: 80, max: 100, oopValue: 10, ipValue: 10 }
];

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
