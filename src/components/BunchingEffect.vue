<template>
  <div v-if="editingPlayer === -1" class="bunching-effect">
    <!-- 信息提示区域 -->
    <div class="info-panel mb-6">
      <div class="flex items-start p-4 bg-cyan-50 border-2 border-cyan-600 rounded-lg">
        <InformationCircleIcon class="w-5 h-5 text-cyan-600 mt-0.5 mr-3 shrink-0" />
        <div class="text-cyan-600">
          <p class="font-medium mb-1">Performance Notice</p>
          <p class="text-sm">
            Enabling the bunching effect will significantly slow down the solver.
            If you are not sure, we recommend leaving it disabled.
          </p>
        </div>
      </div>
    </div>

    <!-- 主要控制区域 -->
    <div class="control-panel bg-white rounded-lg border border-gray-200 shadow-sm p-5">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-6">
          <label class="flex items-center cursor-pointer select-none">
            <input
              type="checkbox"
              v-model="store.isBunchingEnabled"
              class="w-4 h-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
            />
            <span class="ml-2 text-gray-700 font-medium">Enable bunching effect</span>
          </label>

          <div class="flex items-center">
            <Tippy
              class="inline-flex items-center cursor-help text-gray-500 hover:text-indigo-600"
              max-width="500px"
              trigger="mouseenter click"
              placement="bottom"
              :delay="[200, 0]"
              :interactive="true"
            >
              <QuestionMarkCircleIcon class="w-5 h-5" />
              <span class="ml-1 text-sm underline">What is the bunching effect?</span>
              <template #content>
                <div class="p-3 text-sm">
                  <p class="mb-3">
                    The bunching effect refers to the card removal effect caused by
                    the ranges of folded players. Since most players fold with a
                    weak hand, the remaining players will have a higher chance of
                    holding a strong hand.
                  </p>
                  <p class="mb-3">
                    This option supports up to 4 folded players (6-max game). By
                    entering the ranges of the folded players and performing a
                    precomputation, the solver can account for the bunching effect.
                  </p>
                  <p>
                    The simulation is exact and precise. This means that the solver
                    does not make any abstractions or approximations. However,
                    please note that enabling the bunching effect will significantly
                    slow down the solver due to the heavy additional computation
                    required.
                  </p>
                </div>
              </template>
            </Tippy>
          </div>
        </div>
      </div>

      <div v-if="store.isBunchingEnabled" class="mt-6">
        <div class="flex gap-8">
          <div class="grid grid-cols-4 gap-4">
            <div v-for="i in 4" :key="i" class="w-[200px]">
              <div class="text-[1.0625rem]">Fold Range {{ i }}</div>
              <RangeMiniViewer
                class="w-[200px] h-[200px] mt-2 cursor-pointer"
                :player="i + 1"
                @click="editRange(i - 1)"
              />
              <input
                v-model="rangeTexts[i - 1]"
                type="text"
                :class="[
                  'w-full mt-3 px-3 py-2 rounded-md text-sm border transition-colors',
                  isRangeTextError[i - 1]
                    ? 'border-red-300 bg-red-50 focus:ring-red-500 focus:border-red-500'
                    : 'border-gray-300 focus:ring-indigo-500 focus:border-indigo-500'
                ]"
                @focus="($event.target as HTMLInputElement).select()"
                @change="onRangeTextChange(i - 1)"
              />
              <div class="mt-2 text-sm text-gray-600 text-center">
                {{ numCombos[i - 1].toFixed(1) }} combos ({{
                  numCombos[i - 1] >= 0.9995 * ((52 * 51) / 2)
                    ? "100"
                    : ((numCombos[i - 1] * 100) / ((52 * 51) / 2)).toFixed(1)
                }}%)
              </div>
              <div class="flex mt-3 gap-2">
                <button 
                  class="flex-1 px-3 py-1.5 text-sm font-medium rounded-md border border-gray-300
                         bg-white text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2
                         focus:ring-indigo-500 focus:ring-offset-1"
                  @click="invertRange(i - 1)"
                >
                  Invert
                </button>
                <button 
                  class="flex-1 px-3 py-1.5 text-sm font-medium rounded-md border border-gray-300
                         bg-white text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2
                         focus:ring-indigo-500 focus:ring-offset-1"
                  @click="clearRange(i - 1)"
                >
                  Clear
                </button>
              </div>
            </div>
          </div>
          
          <div class="w-64 flex-shrink-0">
            <div class="bg-gray-50 rounded-lg p-4 border border-gray-200">
              <h3 class="text-[1.0625rem] font-medium mb-3">Statistics</h3>
              <div class="space-y-2">
                <div class="text-sm text-gray-600">
                  <div class="flex justify-between items-center mb-1">
                    <span>Total Ranges:</span>
                    <span class="font-medium text-gray-900">{{ numCombos.filter(x => x > 0).length }}/4</span>
                  </div>
                  <div class="flex justify-between items-center mb-1">
                    <span>Total Combos:</span>
                    <span class="font-medium text-gray-900">
                      {{ numCombos.reduce((a, b) => a + b, 0).toFixed(1) }}
                    </span>
                  </div>
                  <div class="flex justify-between items-center">
                    <span>Coverage:</span>
                    <span class="font-medium text-gray-900">
                      {{ ((numCombos.reduce((a, b) => a + b, 0) * 100) / (52 * 51 / 2)).toFixed(1) }}%
                    </span>
                  </div>
                </div>
              </div>
            </div>

            <div class="bg-indigo-50 rounded-lg p-4 border border-indigo-200 mt-4">
              <h3 class="text-[1.0625rem] font-medium mb-3 text-indigo-900">Quick Tips</h3>
              <ul class="text-sm text-indigo-700 space-y-2">
                <li class="flex items-start gap-2">
                  <span class="text-indigo-500 mt-0.5">•</span>
                  <span>Click on any range matrix to edit</span>
                </li>
                <li class="flex items-start gap-2">
                  <span class="text-indigo-500 mt-0.5">•</span>
                  <span>Use text input for quick range entry</span>
                </li>
                <li class="flex items-start gap-2">
                  <span class="text-indigo-500 mt-0.5">•</span>
                  <span>Invert button flips selection</span>
                </li>
              </ul>
            </div>
          </div>
        </div>

        <!-- 控制按钮和状态区域 -->
        <div class="mt-8 bg-white rounded-lg border border-gray-200 p-4">
          <div class="flex items-center gap-4">
            <div class="flex gap-2">
              <button
                class="px-4 py-2 text-sm font-medium rounded-md text-white bg-indigo-600 
                       hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500
                       focus:ring-offset-1 disabled:opacity-50 disabled:cursor-not-allowed"
                :disabled="
                  numCombos.every((x) => x === 0) ||
                  store.isBunchingRunning ||
                  store.isSolverRunning ||
                  numThreads < 1 ||
                  numThreads > 64 ||
                  numThreads % 1 !== 0
                "
                @click="runPrecomputation"
              >
                Run Precomputation
              </button>
              <button
                class="px-4 py-2 text-sm font-medium rounded-md text-white bg-indigo-600
                       hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500
                       focus:ring-offset-1"
                @click="store.currentView = 'run-solver'"
              >
                Run Solver
              </button>
              <button
                class="px-4 py-2 text-sm font-medium rounded-md text-white bg-red-600
                       hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500
                       focus:ring-offset-1 disabled:opacity-50 disabled:cursor-not-allowed"
                :disabled="!hasBunchingRun"
                @click="clearPrecomputation"
              >
                Clear
              </button>
              <button
                v-if="!isBunchingPaused"
                class="px-4 py-2 text-sm font-medium rounded-md text-white bg-emerald-600
                       hover:bg-emerald-700 focus:outline-none focus:ring-2 focus:ring-emerald-500
                       focus:ring-offset-1 disabled:opacity-50 disabled:cursor-not-allowed"
                :disabled="!store.isBunchingRunning"
                @click="() => (pauseFlag = true)"
              >
                Pause
              </button>
              <button
                v-else
                class="px-4 py-2 text-sm font-medium rounded-md text-white bg-emerald-600
                       hover:bg-emerald-700 focus:outline-none focus:ring-2 focus:ring-emerald-500
                       focus:ring-offset-1 disabled:opacity-50 disabled:cursor-not-allowed"
                :disabled="
                  store.isSolverRunning ||
                  numThreads < 1 ||
                  numThreads > 64 ||
                  numThreads % 1 !== 0
                "
                @click="resumePrecomputation"
              >
                Resume
              </button>
              <button 
                @click="toggleDebugInfo"
                class="px-3 py-1 text-sm bg-yellow-500 hover:bg-yellow-600 text-white rounded">
                Debug
              </button>
            </div>

            <div class="flex items-center gap-6 ml-4">
              <div class="flex items-center">
                <span class="text-sm text-gray-700">Number of threads:</span>
                <input
                  v-model="numThreads"
                  type="number"
                  class="w-20 ml-2 px-3 py-1.5 text-sm text-center rounded-md border border-gray-300
                         focus:ring-indigo-500 focus:border-indigo-500"
                  :class="{ 'border-red-300': numThreads < 1 || numThreads > 64 || numThreads % 1 !== 0 }"
                  min="1"
                  max="64"
                />
              </div>
              <div class="flex items-center">
                <span class="text-sm text-gray-700">RAM usage:</span>
                <span class="ml-2 text-sm font-medium text-gray-900">
                  {{ ["-", "60MB", "60MB", "190MB", "3.5GB"][numCombos.filter((x) => x > 0).length] }}
                </span>
              </div>
            </div>
          </div>

          <!-- Debug信息框 -->
          <div v-if="showDebugInfo" class="p-4 bg-gray-100 rounded-lg text-xs overflow-auto max-h-80 mt-4">
            <h3 class="font-bold mb-2">Bunching Debug信息:</h3>
            <div class="grid grid-cols-1 gap-2">
              <div>
                <h4 class="font-semibold">store状态:</h4>
                <pre>{{ {
                  isBunchingEnabled: store.isBunchingEnabled,
                  isBunchingRunning: store.isBunchingRunning,
                  isSolverRunning: store.isSolverRunning,
                  bunchingFlop: store.bunchingFlop,
                  ranges: store.ranges.slice(2, 6) // 只显示fold ranges
                } }}</pre>
              </div>
              <div>
                <h4 class="font-semibold">运行时状态:</h4>
                <pre>{{ {
                  editingPlayer: editingPlayer,
                  rangeTexts,
                  isRangeTextError,
                  numCombos,
                  statusText,
                  hasBunchingRun,
                  isBunchingPaused,
                  numThreads,
                  elapsedTimeMs
                } }}</pre>
              </div>
              <div>
                <h4 class="font-semibold">配置状态:</h4>
                <pre>{{ {
                  board: configStore.board,
                  flopCopy
                } }}</pre>
              </div>
            </div>
            <div class="mt-2">
              <button class="px-3 py-1 text-xs bg-blue-500 hover:bg-blue-600 text-white rounded" @click="refreshDebugInfo">刷新调试信息</button>
              <button class="px-3 py-1 text-xs bg-green-500 hover:bg-green-600 text-white rounded ml-2" @click="syncBunchingData">同步Bunching数据</button>
              <button class="px-3 py-1 text-xs bg-red-500 hover:bg-red-600 text-white rounded ml-2" @click="logDebugInfo">控制台输出</button>
              <button class="px-3 py-1 text-xs bg-purple-500 hover:bg-purple-600 text-white rounded ml-2" @click="diagnoseBunchingStatus">诊断Bunching</button>
            </div>
          </div>

          <div class="mt-4 flex items-center text-sm">
            <span class="text-gray-700">Status:</span>
            <span
              v-if="store.isBunchingRunning"
              class="spinner ml-2 mr-2.5"
            ></span>
            <span :class="[
              'ml-2',
              statusText.includes('Error') ? 'text-red-600' : 
              statusText.includes('ready') ? 'text-emerald-600' : 'text-gray-900'
            ]">{{ statusText }}</span>
            <span v-if="store.bunchingFlop.length > 0" class="ml-3">
              [Flop:
              <span
                v-for="item in store.bunchingFlop.map(cardText)"
                :key="item.rank + item.suit"
                :class="'inline-block mx-px ' + item.colorClass"
              >{{ item.rank + item.suit }}</span
              >]
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>

  <RangeEditor
    v-else
    :player="editingPlayer + 2"
    :default-text="rangeTextCopy"
    @save="saveEdit"
    @cancel="cancelEdit"
  />
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useStore, useConfigStore } from "../store";
import { trimRegex, rangeRegex, cardText, cardStringToNumber } from "../utils";
import * as invokes from "../invokes";

import RangeEditor from "./RangeEditor.vue";
import RangeMiniViewer from "./RangeMiniViewer.vue";
import { Tippy } from "vue-tippy";
import {
  InformationCircleIcon,
  QuestionMarkCircleIcon,
} from "@heroicons/vue/20/solid";

const store = useStore();
const configStore = useConfigStore();

// 设置默认值为true
if (store.isBunchingEnabled === undefined) {
  store.isBunchingEnabled = true;
}

const numThreads = ref(navigator.hardwareConcurrency || 1);

const editingPlayer = ref(-1);
const rangeTexts = ref(["", "", "", ""]);
const isRangeTextError = ref([false, false, false, false]);
const numCombos = ref([0, 0, 0, 0]);
const rangeTextCopy = ref("");

const statusText = ref("No bunching data");
const flopCopy = ref<number[]>([]);  // 改为number[]类型，与旧版一致
const hasBunchingRun = ref(false);
const isBunchingPaused = ref(false);
const terminateFlag = ref(false);
const pauseFlag = ref(false);
const elapsedTimeMs = ref(-1);

let startTime = 0;

const onUpdate = async (player: number) => {
  const weights = await invokes.rangeGetWeights(player + 2);
  for (let i = 0; i < 13 * 13; ++i) {
    store.ranges[player + 2][i] = weights[i] * 100;
  }
  isRangeTextError.value[player] = false;
};

const onUpdateLocal = async (player: number) => {
  rangeTexts.value[player] = await invokes.rangeToString(player + 2);
  numCombos.value[player] = await invokes.rangeNumCombos(player + 2);
};

const editRange = async (player: number) => {
  rangeTextCopy.value = await invokes.rangeToString(player + 2);
  store.headers["bunching"].push(`Fold Range ${player + 1}`);
  editingPlayer.value = player;
};

const onRangeTextChange = async (player: number) => {
  const trimmed = rangeTexts.value[player].replace(trimRegex, "$1").trim();
  const ranges = trimmed.split(",");

  if (ranges[ranges.length - 1] === "") {
    ranges.pop();
  }

  for (const range of ranges) {
    if (!rangeRegex.test(range)) {
      isRangeTextError.value[player] = true;
      return;
    }
  }

  const errorString = await invokes.rangeFromString(player + 2, trimmed);

  if (errorString) {
    isRangeTextError.value[player] = true;
  } else {
    await onUpdate(player);
    await onUpdateLocal(player);
  }
};

const invertRange = async (player: number) => {
  await invokes.rangeInvert(player + 2);
  await onUpdate(player);
  await onUpdateLocal(player);
};

const clearRange = async (player: number) => {
  await invokes.rangeClear(player + 2);
  await onUpdate(player);
  await onUpdateLocal(player);
};

// 添加一个备用的卡牌转换函数（如果cardStringToNumber不可用）
const convertCardToNumber = (card: string): number => {
  const ranks: Record<string, number> = {
    '2': 0, '3': 1, '4': 2, '5': 3, '6': 4, '7': 5, '8': 6, '9': 7,
    'T': 8, 'J': 9, 'Q': 10, 'K': 11, 'A': 12
  };
  const suits: Record<string, number> = { 'c': 0, 'd': 1, 'h': 2, 's': 3 };
  
  if (card.length !== 2) {
    console.error(`Invalid card format: ${card}`);
    return 0;
  }
  
  const rank = card[0];
  const suit = card[1];
  
  if (!(rank in ranks) || !(suit in suits)) {
    console.error(`Invalid card: ${card}, rank: ${rank}, suit: ${suit}`);
    return 0;
  }
  
  return ranks[rank] * 4 + suits[suit];
};

// 提供一个安全的卡牌转换函数
const getCardNumber = (card: string): number => {
  try {
    if (typeof cardStringToNumber === 'function') {
      return cardStringToNumber(card);
    } else {
      return convertCardToNumber(card);
    }
  } catch (e) {
    console.error(`Error converting card ${card}:`, e);
    return convertCardToNumber(card);
  }
};

const runPrecomputation = async () => {
  // 清空现有bunching数据
  store.bunchingFlop = [];

  try {
    // 转换卡牌从字符串到数字，使用getCardNumber替代cardStringToNumber
    const numericBoard = configStore.board.map(card => getCardNumber(card));
    console.log("预计算使用卡牌:", configStore.board);
    console.log("转换后的数字卡牌:", numericBoard);

    // 使用转换后的数字ID初始化
    const errorString = await invokes.bunchingInit(numericBoard);
    if (errorString) {
      statusText.value = `Error: ${errorString}`;
      return;
    }

    statusText.value = "Phase 1/3 - Preparing...";
    flopCopy.value = configStore.board.slice(0, 3);
    hasBunchingRun.value = true;
    elapsedTimeMs.value = 0;
    await resumePrecomputation();
  } catch (err) {
    console.error("预计算初始化错误:", err);
    statusText.value = `Error: ${err.message || "Unknown error during precomputation"}`;
  }
};

const clearPrecomputation = async () => {
  if (store.isBunchingRunning) {
    terminateFlag.value = true;
  } else {
    await invokes.bunchingClear();
    store.bunchingFlop = [];
    statusText.value = "No bunching data";
    hasBunchingRun.value = false;
    isBunchingPaused.value = false;
  }
};

const resumePrecomputation = async () => {
  store.isBunchingRunning = true;
  isBunchingPaused.value = false;
  startTime = performance.now();
  await invokes.setNumThreads(numThreads.value);

  while (true) {
    if (terminateFlag.value) {
      await invokes.bunchingClear();
      store.bunchingFlop = [];
      statusText.value = "No bunching data";
      hasBunchingRun.value = false;
      break;
    }

    if (pauseFlag.value) {
      isBunchingPaused.value = true;
      break;
    }

    const [phase, percent] = await invokes.bunchingProgress();

    if (phase === 3 && percent === 100) {
      store.bunchingFlop = flopCopy.value;
      statusText.value = "Bunching data ready!";
      break;
    }

    statusText.value = `Phase ${phase}/3 - ${percent}% completed...`;
  }

  elapsedTimeMs.value += performance.now() - startTime;

  if (!terminateFlag.value && !pauseFlag.value) {
    statusText.value += ` (Time: ${(elapsedTimeMs.value / 1000).toFixed(2)}s)`;
  }

  store.isBunchingRunning = false;
  terminateFlag.value = false;
  pauseFlag.value = false;
};

const saveEdit = async () => {
  isRangeTextError.value[editingPlayer.value] = false;
  await onUpdateLocal(editingPlayer.value);
  store.headers["bunching"].pop();
  editingPlayer.value = -1;
};

const cancelEdit = async () => {
  rangeTexts.value[editingPlayer.value] = rangeTextCopy.value;
  await invokes.rangeFromString(editingPlayer.value + 2, rangeTextCopy.value);
  await onUpdate(editingPlayer.value);
  store.headers["bunching"].pop();
  editingPlayer.value = -1;
};

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
    store: {
      isBunchingEnabled: store.isBunchingEnabled,
      isBunchingRunning: store.isBunchingRunning,
      isSolverRunning: store.isSolverRunning,
      bunchingFlop: store.bunchingFlop,
      ranges: store.ranges.slice(2, 6)
    },
    runtime: {
      editingPlayer: editingPlayer.value,
      rangeTexts: rangeTexts.value,
      isRangeTextError: isRangeTextError.value,
      numCombos: numCombos.value,
      statusText: statusText.value,
      hasBunchingRun: hasBunchingRun.value,
      isBunchingPaused: isBunchingPaused.value,
      numThreads: numThreads.value,
      elapsedTimeMs: elapsedTimeMs.value
    },
    config: {
      board: configStore.board,
      flopCopy: flopCopy.value
    }
  }
}

// 同步Bunching数据
const syncBunchingData = async () => {
  console.log("Syncing bunching data...");
  
  try {
    // 确保board数据有效
    if (!configStore.board || configStore.board.length < 3) {
      console.error("Board data invalid:", configStore.board);
      statusText.value = "Error: Board data is invalid or missing";
      return;
    }
    
    // 清理现有数据
    await invokes.bunchingClear();
    
    // 直接设置bunching flop数据
    store.bunchingFlop = [...configStore.board];
    flopCopy.value = [...configStore.board];
    
    // 重要：转换成数字ID，使用getCardNumber
    const numericBoard = configStore.board.map(card => getCardNumber(card));
    console.log("Syncing with numeric board:", numericBoard);
    
    // 使用数字ID初始化
    const errorString = await invokes.bunchingInit(numericBoard);
    if (errorString) {
      console.error("bunchingInit failed:", errorString);
      statusText.value = `Error: ${errorString}`;
      return;
    }
    
    statusText.value = "Bunching data initialized";
    refreshDebugInfo();
  } catch (err) {
    console.error("Sync error:", err);
    statusText.value = `Error: ${err.message || "Unknown error during sync"}`;
  }
};

// 在控制台输出更详细的调试信息
const logDebugInfo = () => {
  console.log("=== BUNCHING DEBUG INFO ===")
  console.log("Store状态:", {
    isBunchingEnabled: store.isBunchingEnabled,
    isBunchingRunning: store.isBunchingRunning,
    isSolverRunning: store.isSolverRunning,
    bunchingFlop: store.bunchingFlop,
    ranges: store.ranges.slice(2, 6)
  })
  console.log("运行时状态:", {
    editingPlayer: editingPlayer.value,
    rangeTexts: rangeTexts.value,
    isRangeTextError: isRangeTextError.value,
    numCombos: numCombos.value,
    statusText: statusText.value,
    hasBunchingRun: hasBunchingRun.value,
    isBunchingPaused: isBunchingPaused.value,
    numThreads: numThreads.value,
    elapsedTimeMs: elapsedTimeMs.value
  })
  console.log("配置状态:", {
    board: configStore.board,
    flopCopy: flopCopy.value
  })
  console.log("=======================")
}

// 增强诊断功能，特别针对卡牌转换
const diagnoseBunchingStatus = async () => {
  try {
    console.log("===== BUNCHING DIAGNOSTIC =====");
    
    // 测试卡牌转换
    console.log("Testing card conversion:");
    const cards = ["Td", "Tc", "9c"]; // 使用当前牌面
    const cardIds = cards.map(card => {
      const id = getCardNumber(card);
      console.log(`Card ${card} -> ID ${id}`);
      return id;
    });
    
    // 验证ID是否在有效范围内 (0-51)
    const validIds = cardIds.every(id => id >= 0 && id < 52);
    console.log(`Card IDs valid: ${validIds}, IDs: ${cardIds.join(', ')}`);
    
    // 测试bunchingInit
    console.log("Testing bunchingInit with numeric IDs...");
    await invokes.bunchingClear();
    const testResult = await invokes.bunchingInit(cardIds);
    console.log("Test result:", testResult || "Success");
    
    // 恢复清理状态
    await invokes.bunchingClear();
    
    // 更新UI
    statusText.value = `Diagnostic complete${!validIds ? ": Card conversion issue detected" : ""}. See console for details.`;
  } catch (err) {
    console.error("Diagnostic error:", err);
    statusText.value = `Diagnostic error: ${err.message}`;
  }
};

// 修改验证函数，使用兼容的验证逻辑
const validateFlop = (board: any[]): string | null => {
  if (!board || board.length < 3) {
    return "Board must have at least 3 cards";
  }
  
  // 检查卡牌有效性，采用简单方式
  const uniqueCards = new Set(board);
  
  if (uniqueCards.size !== board.length) {
    return "Invalid flop: Cannot have duplicate cards";
  }
  
  return null;
};

onMounted(async () => {
  console.log("BunchingEffect mounted with board:", configStore.board);
  
  // 如果已有board数据，则进行同步
  if (configStore.board && configStore.board.length >= 3) {
    try {
      // 先清理
      await invokes.bunchingClear();
      
      // 设置数据
      store.bunchingFlop = [...configStore.board];
      flopCopy.value = [...configStore.board];
      
      // 转换为数字ID
      const numericBoard = configStore.board.map(card => cardStringToNumber(card));
      console.log("Mount init with numeric board:", numericBoard);
      
      // 使用数字ID初始化
      const errorString = await invokes.bunchingInit(numericBoard);
      if (errorString) {
        console.error("Mount init failed:", errorString);
      } else {
        console.log("Mount initialization successful");
      }
      
      // 更新range状态
      for (let i = 0; i < 4; i++) {
        await onUpdateLocal(i);
      }
    } catch (err) {
      console.error("Error initializing bunching:", err);
    }
  }
});
</script>

<style scoped>
.bunching-effect {
  @apply text-gray-800;
}

.info-panel {
  @apply transition-all;
}

.control-panel {
  @apply transition-all;
}

.range-card {
  @apply transition-all hover:shadow-sm;
}

.spinner {
  @apply w-4 h-4 border-2 border-gray-300 border-t-indigo-600 rounded-full animate-spin;
}

/* Tippy样式优化 */
:deep(.tippy-box) {
  @apply shadow-lg rounded-lg border border-gray-200;
}

:deep(.tippy-content) {
  @apply bg-white text-gray-700;
}

/* 输入框样式 */
input[type="number"] {
  @apply appearance-none;
  -moz-appearance: textfield;
}

input[type="number"]::-webkit-outer-spin-button,
input[type="number"]::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

/* 复选框样式 */
input[type="checkbox"] {
  @apply rounded border-gray-300 text-indigo-600 focus:ring-indigo-500;
}
</style>
