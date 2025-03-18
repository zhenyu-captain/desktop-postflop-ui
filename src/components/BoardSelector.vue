<template>
  <div class="flex flex-col space-y-4">
    <!-- 原有的4排选择器 -->
    <div class="grid grid-cols-13 gap-1">
      <template v-for="suit in ['s', 'h', 'd', 'c']" :key="suit">
        <div v-for="rank in ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']" 
             :key="rank + suit"
             class="w-8 h-8 flex items-center justify-center rounded cursor-pointer text-sm"
             :class="[
               getCardColor(suit),
               isCardSelected(rank + suit) ? 'ring-2 ring-blue-500' : 'hover:bg-gray-100'
             ]"
             @click="toggleCard(rank + suit)">
          {{ rank }}{{ getSuitSymbol(suit) }}
        </div>
      </template>
    </div>

    <!-- 当前选择的翻牌 -->
    <div class="flex items-center space-x-4">
      <div class="flex space-x-2">
        <div v-for="(card, index) in selectedCards" :key="index" 
             class="w-12 h-16 bg-white rounded shadow-md flex items-center justify-center text-xl"
             :class="getCardColor(card[1] || '')">
          {{ card ? `${card[0]}${getSuitSymbol(card[1])}` : '?' }}
        </div>
      </div>
      <button 
        @click="clearSelection"
        class="px-3 py-1 text-sm text-gray-600 hover:bg-gray-100 rounded">
        清空选择
      </button>
      <button 
        @click="toggleDebugInfo"
        class="px-3 py-1 text-sm bg-yellow-500 hover:bg-yellow-600 text-white rounded">
        Debug
      </button>
    </div>

    <!-- Debug信息框 -->
    <div v-if="showDebugInfo" class="p-4 bg-gray-100 rounded-lg text-xs overflow-auto max-h-80">
      <h3 class="font-bold mb-2">Board Debug信息:</h3>
      <div class="grid grid-cols-1 gap-2">
        <div>
          <h4 class="font-semibold">selectedCards状态:</h4>
          <pre>{{ JSON.stringify(selectedCards, null, 2) }}</pre>
        </div>
        <div>
          <h4 class="font-semibold">config.board状态:</h4>
          <pre>{{ JSON.stringify(config.board, null, 2) }}</pre>
        </div>
        <div>
          <h4 class="font-semibold">store.bunchingFlop状态:</h4>
          <pre>{{ JSON.stringify(store.bunchingFlop, null, 2) }}</pre>
        </div>
        <div>
          <h4 class="font-semibold">store.isBunchingEnabled:</h4>
          <pre>{{ store.isBunchingEnabled }}</pre>
        </div>
      </div>
      <div class="mt-2">
        <button class="px-3 py-1 text-xs bg-blue-500 hover:bg-blue-600 text-white rounded" @click="refreshDebugInfo">刷新调试信息</button>
        <button class="px-3 py-1 text-xs bg-green-500 hover:bg-green-600 text-white rounded ml-2" @click="syncBoardData">同步Board数据</button>
        <button class="px-3 py-1 text-xs bg-red-500 hover:bg-red-600 text-white rounded ml-2" @click="logDebugInfo">控制台输出</button>
      </div>
    </div>

    <!-- 组合统计 -->
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

    <!-- Equity分布 -->
    <div class="p-4 bg-gray-50 rounded-lg">
      <h3 class="text-sm font-medium mb-3">Equity分布</h3>
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
        <div class="flex justify-center space-x-8 text-xs text-gray-500 mt-2">
          <span>← OOP Equity</span>
          <span>IP Equity →</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useStore, useConfigStore, useSavedConfigStore, useTmpConfigStore } from '../store'
import { cardStringToNumber, cardNumberToString } from '../utils'

const store = useStore()
const config = useConfigStore()
const savedConfig = useSavedConfigStore()
const tmpConfig = useTmpConfigStore()
const selectedCards = ref(['', '', ''])

// 调试相关状态
const showDebugInfo = ref(false)
const debugInfo = ref({})

// 切换调试信息显示状态
const toggleDebugInfo = () => {
  showDebugInfo.value = !showDebugInfo.value
  if (showDebugInfo.value) {
    refreshDebugInfo()
  }
}

// 刷新调试信息
const refreshDebugInfo = () => {
  debugInfo.value = {
    selectedCards: selectedCards.value,
    configBoard: config.board,
    savedConfigBoard: savedConfig.board,
    tmpConfigBoard: tmpConfig.board,
    bunchingFlop: store.bunchingFlop,
    isBunchingEnabled: store.isBunchingEnabled
  }
}

// 同步Board数据
const syncBoardData = () => {
  // 确保所有相关状态一致
  const validCards = selectedCards.value.filter(card => card !== '')
  
  // 更新config
  config.board = [...validCards]
  
  // 更新savedConfig和tmpConfig
  savedConfig.board = [...validCards]
  tmpConfig.board = [...validCards]
  
  // 如果是翻牌且bunching功能启用，则更新bunchingFlop
  if (validCards.length === 3 && store.isBunchingEnabled) {
    store.bunchingFlop = [...validCards]
  }
  
  refreshDebugInfo()
  console.log('已同步Board数据:', {
    selectedCards: selectedCards.value,
    configBoard: config.board,
    savedConfigBoard: savedConfig.board,
    tmpConfigBoard: tmpConfig.board,
    bunchingFlop: store.bunchingFlop
  })
}

// 在控制台输出更详细的调试信息
const logDebugInfo = () => {
  console.log("=== BOARD DEBUG INFO ===")
  console.log("selectedCards:", selectedCards.value)
  console.log("config.board:", config.board)
  console.log("savedConfig.board:", savedConfig.board)
  console.log("tmpConfig.board:", tmpConfig.board)
  console.log("store.bunchingFlop:", store.bunchingFlop)
  console.log("store.isBunchingEnabled:", store.isBunchingEnabled)
  console.log("=======================")
}

// 监听selectedCards变化，更新store中的board
watch(selectedCards, (newCards) => {
  // 过滤掉空字符串，只保留有效的卡牌
  const validCards = newCards.filter(card => card !== '')
  
  // 直接使用字符串类型的卡牌，暂时跳过转换
  config.board = [...validCards]
  
  console.log("BoardSelector updating board:", {
    validCards,
    configBoard: config.board
  });
  
  // 如果是翻牌，且bunching功能启用，则更新bunchingFlop
  if (validCards.length === 3 && store.isBunchingEnabled) {
    store.bunchingFlop = [...validCards]
    console.log("Updated bunchingFlop:", store.bunchingFlop);
  }
}, { deep: true })

// 初始化时，如果config中已有board数据，则加载到selectedCards
onMounted(() => {
  if (config.board && config.board.length > 0) {
    // 确保selectedCards数组长度为3
    selectedCards.value = [...config.board.slice(0, 3), '', '', ''].slice(0, 3)
  }
  
  // 调试信息
  console.log('BoardSelector mounted:', {
    configBoard: config.board,
    selectedCards: selectedCards.value,
    isBunchingEnabled: store.isBunchingEnabled,
    bunchingFlop: store.bunchingFlop
  })
})

// 牌力等级数据
const strengthLevels = [
  { name: '极弱', oopValue: 15, ipValue: 10 },
  { name: '较弱', oopValue: 25, ipValue: 20 },
  { name: '中等', oopValue: 30, ipValue: 35 },
  { name: '较强', oopValue: 20, ipValue: 25 },
  { name: '极强', oopValue: 10, ipValue: 10 }
]

// Equity区间数据
const equityRanges = [
  { label: '0-20%', min: 0, max: 20, oopValue: 10, ipValue: 8 },
  { label: '20-40%', min: 20, max: 40, oopValue: 20, ipValue: 15 },
  { label: '40-60%', min: 40, max: 60, oopValue: 40, ipValue: 45 },
  { label: '60-80%', min: 60, max: 80, oopValue: 20, ipValue: 22 },
  { label: '80-100%', min: 80, max: 100, oopValue: 10, ipValue: 10 }
]

// 组合统计
const remainingCombos = computed(() => {
  // TODO: 实现组合计算逻辑
  return 1081 // 示例数值
})

const blockedByOOP = computed(() => {
  // TODO: 实现阻挡组合计算逻辑
  return 78 // 示例数值
})

const flushCombos = computed(() => {
  // TODO: 实现同花组合计算逻辑
  return 45 // 示例数值
})

const straightCombos = computed(() => {
  // TODO: 实现顺子组合计算逻辑
  return 32 // 示例数值
})

// 工具方法
const getCardColor = (suit: string) => {
  return suit === 'h' || suit === 'd' ? 'text-red-600' : 'text-gray-900'
}

const getSuitSymbol = (suit: string) => {
  const symbols: Record<string, string> = {
    's': '♠',
    'h': '♥',
    'd': '♦',
    'c': '♣'
  }
  return symbols[suit] || suit
}

const isCardSelected = (card: string) => {
  return selectedCards.value.includes(card)
}

const toggleCard = (card: string) => {
  const index = selectedCards.value.indexOf(card)
  if (index !== -1) {
    // 如果卡牌已经被选中，则移除
    selectedCards.value[index] = ''
    // 重新排列卡牌，确保没有空位
    const validCards = selectedCards.value.filter(c => c !== '')
    selectedCards.value = [...validCards, ...Array(3 - validCards.length).fill('')]
  } else {
    // 如果卡牌未被选中，则添加到第一个空位
    const emptyIndex = selectedCards.value.findIndex(c => !c)
    if (emptyIndex !== -1) {
      selectedCards.value[emptyIndex] = card
    }
  }
  
  // 每次修改后立即更新store中的数据
  const validCards = selectedCards.value.filter(card => card !== '')
  config.board = validCards
  
  // 如果是翻牌，且bunching功能启用，则更新bunchingFlop
  if (validCards.length === 3 && store.isBunchingEnabled) {
    store.bunchingFlop = [...validCards]
  }
}

const clearSelection = () => {
  selectedCards.value = ['', '', '']
  config.board = []
  if (store.isBunchingEnabled) {
    store.bunchingFlop = []
  }
}

const getEquityColor = (value: number) => {
  // 根据equity值返回从红到绿的渐变色
  const red = Math.floor(255 * (1 - value / 100))
  const green = Math.floor(255 * (value / 100))
  return `rgb(${red}, ${green}, 100)`
}
</script>

<style scoped>
.grid-cols-13 {
  grid-template-columns: repeat(13, minmax(0, 1fr));
}
</style>
