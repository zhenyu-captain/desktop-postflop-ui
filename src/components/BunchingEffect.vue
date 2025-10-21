<template>
  <div v-if="editingPlayer === -1" class="bunching-effect">
    <!-- 信息提示区域 -->
    <div class="mb-4">
      <div class="flex items-start p-4 bg-cyan-50 border-2 border-cyan-600 rounded-lg">
        <InformationCircleIcon class="w-5 h-5 text-cyan-600 mt-0.5 mr-3 shrink-0" />
        <div class="text-cyan-600">
          <p class="font-medium">
            Enabling the bunching effect will significantly slow down the
            solver.<br />
            If you are not sure, we recommend leaving it disabled.
          </p>
        </div>
      </div>
    </div>

    <!-- 主要控制区域 -->
    <div class="bg-white rounded-lg p-5">
      <div class="flex items-center gap-6">
        <label class="flex items-center cursor-pointer select-none">
          <input
            type="checkbox"
            v-model="isBunchingEnabledUI"
            @change="updateBunchingState"
            class="w-4 h-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500 mr-2"
          />
          <span class="text-gray-700">Enable bunching effect</span>
        </label>

        <div class="flex items-center">
          <Tippy
            class="inline-flex items-center cursor-help text-gray-600 hover:text-indigo-600"
            max-width="500px"
            trigger="mouseenter click"
            placement="bottom"
            :delay="[200, 0]"
            :interactive="true"
          >
            <QuestionMarkCircleIcon class="w-5 h-5" />
            <span class="ml-1 text-sm underline">What is the bunching effect?</span>
            <template #content>
              <div class="px-3 py-2 text-sm">
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

      <div v-if="isBunchingEnabledUI" class="mt-6">
        <div class="flex gap-4">
          <div class="grid grid-cols-1 md:grid-cols-4 gap-4 flex-1">
            <div v-for="i in 4" :key="i" class="range-card">
              <div class="text-base font-medium mb-2">Fold Range {{ i }}</div>
              <RangeMiniViewer
                class="w-full h-44 cursor-pointer mb-3"
                :player="i + 1"
                @click="editRange(i - 1)"
              />
              <input
                v-model="rangeTexts[i - 1]"
                type="text"
                :class="[
                  'w-full px-3 py-2 rounded-md text-sm border transition-colors mb-2',
                  isRangeTextError[i - 1]
                    ? 'border-red-300 bg-red-50 focus:ring-red-500 focus:border-red-500'
                    : 'border-gray-300 focus:ring-indigo-500 focus:border-indigo-500'
                ]"
                @focus="($event.target as HTMLInputElement).select()"
                @change="onRangeTextChange(i - 1)"
              />
              <div class="text-sm text-gray-600 text-center mb-3">
                {{ numCombos[i - 1].toFixed(1) }} combos ({{
                  numCombos[i - 1] >= 0.9995 * ((52 * 51) / 2)
                    ? "100"
                    : ((numCombos[i - 1] * 100) / ((52 * 51) / 2)).toFixed(1)
                }}%)
              </div>
              <div class="flex gap-2">
                <button 
                  class="flex-1 px-3 py-1.5 text-sm font-medium rounded-md border border-gray-300
                         bg-white text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2
                         focus:ring-indigo-500"
                  @click="invertRange(i - 1)"
                >
                  Invert
                </button>
                <button 
                  class="flex-1 px-3 py-1.5 text-sm font-medium rounded-md border border-gray-300
                         bg-white text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2
                         focus:ring-indigo-500"
                  @click="clearRange(i - 1)"
                >
                  Clear
                </button>
              </div>
            </div>
          </div>

          <!-- 右侧信息面板 -->
          <div class="hidden md:block w-64 flex-shrink-0 space-y-4">
            <!-- 统计信息卡片 -->
            <div class="bg-gray-50 rounded-lg p-4 border border-gray-200">
              <h3 class="text-base font-medium mb-3">Statistics</h3>
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

            <!-- 快速提示卡片 -->
            <div class="bg-indigo-50 rounded-lg p-4 border border-indigo-200">
              <h3 class="text-base font-medium mb-3 text-indigo-900">Quick Tips</h3>
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
        <div class="mt-6 bg-gray-50 rounded-lg border border-gray-200 p-4">
          <div class="flex flex-wrap items-center gap-3">
            <button
              class="px-4 py-2 text-sm font-medium rounded-md text-white bg-blue-600 
                     hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500
                     disabled:opacity-50 disabled:cursor-not-allowed"
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
              class="px-4 py-2 text-sm font-medium rounded-md text-white bg-red-600
                     hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500
                     disabled:opacity-50 disabled:cursor-not-allowed"
              :disabled="!hasBunchingRun"
              @click="clearPrecomputation"
            >
              Clear
            </button>
            <!-- Debug按钮 -->
            <button
              class="px-4 py-2 text-sm font-medium rounded-md text-white bg-yellow-500
                     hover:bg-yellow-600 focus:outline-none focus:ring-2 focus:ring-yellow-500"
              @click="showDebugInfo"
            >
              Debug
            </button>
            <button
              v-if="!isBunchingPaused"
              class="px-4 py-2 text-sm font-medium rounded-md text-white bg-green-600
                     hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500
                     disabled:opacity-50 disabled:cursor-not-allowed"
              :disabled="!store.isBunchingRunning"
              @click="() => (pauseFlag = true)"
            >
              Pause
            </button>
            <button
              v-else
              class="px-4 py-2 text-sm font-medium rounded-md text-white bg-green-600
                     hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500
                     disabled:opacity-50 disabled:cursor-not-allowed"
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

            <div class="flex items-center ml-2">
              <span class="text-sm text-gray-700 mr-2">Number of threads:</span>
              <input
                v-model="numThreads"
                type="number"
                :class="[
                  'w-20 px-3 py-1.5 text-sm text-center rounded-md border',
                  numThreads < 1 || numThreads > 64 || numThreads % 1 !== 0
                    ? 'border-red-300 bg-red-50'
                    : 'border-gray-300'
                ]"
                min="1"
                max="64"
              />
            </div>
            <div class="flex items-center ml-2">
              <span class="text-sm text-gray-700 mr-2">RAM usage:</span>
              <span class="text-sm font-medium text-gray-900">
                {{ ["-", "60MB", "60MB", "190MB", "3.5GB"][numCombos.filter((x) => x > 0).length] }}
              </span>
            </div>
          </div>

          <div class="mt-4 flex items-center text-sm">
            <span class="text-gray-700">Status:</span>
            <span
              v-if="store.isBunchingRunning"
              class="inline-block w-4 h-4 ml-2 mr-2.5 border-2 border-t-blue-600 border-gray-300 rounded-full animate-spin"
            ></span>
            <span :class="[
              'ml-2',
              statusText.includes('Error') ? 'text-red-600' : 
              statusText.includes('ready') ? 'text-green-600' : 'text-gray-900'
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
import { ref, onMounted, onUnmounted } from "vue";
import { useStore, useConfigStore } from "../store";
import { trimRegex, rangeRegex, cardText } from "../utils";
import * as invokes from "../invokes";
import eventBus from "../eventBus";

import RangeEditor from "./RangeEditor.vue";
import RangeMiniViewer from "./RangeMiniViewer.vue";
import { Tippy } from "vue-tippy";
import {
  InformationCircleIcon,
  QuestionMarkCircleIcon,
} from "@heroicons/vue/20/solid";

const store = useStore();
const configStore = useConfigStore();

// UI和功能状态
const isBunchingEnabledUI = ref(false);

// 组件挂载时设置默认UI状态
onMounted(() => {
  // 默认勾选UI，但不改变功能状态
  isBunchingEnabledUI.value = true;
  
  // 如果已有bunching数据，UI状态和功能状态保持一致
  if (store.bunchingFlop.length > 0 && hasBunchingRun.value) {
    isBunchingEnabledUI.value = store.isBunchingEnabled;
  } else {
    // 否则，功能状态总是false
    store.isBunchingEnabled = false;
  }
  
  // 添加事件总线监听器，用于响应Debug数据请求
  eventBus.on('request-debug-data', handleRequestDebugData);
});

// 组件卸载时移除事件监听
onUnmounted(() => {
  eventBus.off('request-debug-data', handleRequestDebugData);
});

// 响应debug数据请求 - 与showDebugInfo函数逻辑相同
const handleRequestDebugData = () => {
  // 收集当前 Bunching 配置的所有信息
  const bunchingData = {
    status: {
      enabled: store.isBunchingEnabled,
      uiEnabled: isBunchingEnabledUI.value,
      isRunning: store.isBunchingRunning,
      isPaused: isBunchingPaused.value,
      statusText: statusText.value,
      hasBunchingRun: hasBunchingRun.value,
      elapsedTimeMs: elapsedTimeMs.value,
      numThreads: numThreads.value
    },
    foldRanges: rangeTexts.value.map((text, index) => ({
      index: index + 1,
      text: text,
      hasError: isRangeTextError.value[index],
      combos: numCombos.value[index],
      percentage: numCombos.value[index] >= 0.9995 * ((52 * 51) / 2)
        ? 100
        : ((numCombos.value[index] * 100) / ((52 * 51) / 2))
    })),
    statistics: {
      totalRanges: numCombos.value.filter(x => x > 0).length,
      totalCombos: numCombos.value.reduce((a, b) => a + b, 0),
      coverage: ((numCombos.value.reduce((a, b) => a + b, 0) * 100) / (52 * 51 / 2))
    },
    flop: store.bunchingFlop.length > 0 ? store.bunchingFlop.map(cardId => {
      const { rank, suitLetter } = cardText(cardId);
      return rank.toLowerCase() + suitLetter;
    }) : [],
    board: configStore.board.length > 0 ? configStore.board.map(cardId => {
      const { rank, suitLetter } = cardText(cardId);
      return rank.toLowerCase() + suitLetter;
    }) : [],
    memory: {
      estimatedUsage: ["-", "60MB", "60MB", "190MB", "3.5GB"][numCombos.value.filter((x) => x > 0).length]
    }
  };
  
  // 通过事件总线发送Bunching数据
  eventBus.emit('bunching-debug-data', bunchingData);
};

// 更新bunching状态
const updateBunchingState = () => {
  if (isBunchingEnabledUI.value && store.bunchingFlop.length === 0) {
    // 如果UI勾选但没有预计算数据，不修改store中的值
    // 这样在Run Solver时会显示警告而不是尝试收集数据
    store.isBunchingEnabled = false;
  } else {
    // 如果有预计算数据或用户取消勾选，直接同步状态
    store.isBunchingEnabled = isBunchingEnabledUI.value;
  }
};

const numThreads = ref(navigator.hardwareConcurrency || 1);

const editingPlayer = ref(-1);
const rangeTexts = ref(["", "", "", ""]);
const isRangeTextError = ref([false, false, false, false]);
const numCombos = ref([0, 0, 0, 0]);
const rangeTextCopy = ref("");

const statusText = ref("No bunching data");
const flopCopy = ref<number[]>([]);
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

const runPrecomputation = async () => {
  store.bunchingFlop = [];

  const errorString = await invokes.bunchingInit(configStore.board);
  if (errorString) {
    statusText.value = `Error: ${errorString}`;
    return;
  }

  statusText.value = "Phase 1/3 - Preparing...";
  flopCopy.value = configStore.board.slice(0, 3);
  hasBunchingRun.value = true;
  elapsedTimeMs.value = 0;
  
  await resumePrecomputation();
  
  // 预计算完成后，如果UI勾选则启用功能
  if (isBunchingEnabledUI.value && statusText.value.includes("ready")) {
    store.isBunchingEnabled = true;
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
    
    // 清除预计算数据时，禁用功能状态但保持UI状态
    store.isBunchingEnabled = false;
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

const showDebugInfo = async () => {
  // 收集当前 Bunching 配置的所有信息
  const bunchingData = {
    status: {
      enabled: store.isBunchingEnabled,
      uiEnabled: isBunchingEnabledUI.value,
      isRunning: store.isBunchingRunning,
      isPaused: isBunchingPaused.value,
      statusText: statusText.value,
      hasBunchingRun: hasBunchingRun.value,
      elapsedTimeMs: elapsedTimeMs.value,
      numThreads: numThreads.value
    },
    foldRanges: rangeTexts.value.map((text, index) => ({
      index: index + 1,
      text: text,
      hasError: isRangeTextError.value[index],
      combos: numCombos.value[index],
      percentage: numCombos.value[index] >= 0.9995 * ((52 * 51) / 2)
        ? 100
        : ((numCombos.value[index] * 100) / ((52 * 51) / 2))
    })),
    statistics: {
      totalRanges: numCombos.value.filter(x => x > 0).length,
      totalCombos: numCombos.value.reduce((a, b) => a + b, 0),
      coverage: ((numCombos.value.reduce((a, b) => a + b, 0) * 100) / (52 * 51 / 2))
    },
    flop: store.bunchingFlop.length > 0 ? store.bunchingFlop.map(cardId => {
      const { rank, suitLetter } = cardText(cardId);
      return rank.toLowerCase() + suitLetter;
    }) : [],
    board: configStore.board.length > 0 ? configStore.board.map(cardId => {
      const { rank, suitLetter } = cardText(cardId);
      return rank.toLowerCase() + suitLetter;
    }) : [],
    memory: {
      estimatedUsage: ["-", "60MB", "60MB", "190MB", "3.5GB"][numCombos.value.filter((x) => x > 0).length]
    }
  };
  
  // 格式化为JSON字符串
  const jsonFormat = JSON.stringify(bunchingData, null, 2);
  
  try {
    // 复制到剪贴板
    await navigator.clipboard.writeText(jsonFormat);
    
    // 创建弹窗显示JSON数据
    const dialog = document.createElement('div');
    dialog.style.cssText = `
      position: fixed;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: 80%;
      max-width: 800px;
      max-height: 80vh;
      background: white;
      border-radius: 8px;
      box-shadow: 0 4px 15px rgba(0,0,0,0.2);
      z-index: 9999;
      padding: 20px;
      display: flex;
      flex-direction: column;
    `;
    
    // 创建弹窗标题
    const header = document.createElement('div');
    header.style.cssText = `
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 15px;
      border-bottom: 1px solid #eee;
      padding-bottom: 10px;
    `;
    header.innerHTML = `
      <h3 style="margin: 0; font-size: 16px; font-weight: 600;">Bunching 数据 (JSON已复制到剪贴板)</h3>
      <button id="closeDialog" style="background: none; border: none; cursor: pointer; font-size: 16px;">×</button>
    `;
    
    // 创建JSON内容显示区域
    const content = document.createElement('div');
    content.style.cssText = `
      overflow-y: auto;
      font-family: monospace;
      white-space: pre;
      background: #f5f5f5;
      padding: 15px;
      border-radius: 5px;
      flex-grow: 1;
      font-size: 14px;
    `;
    content.textContent = jsonFormat;
    
    // 创建按钮区域
    const actions = document.createElement('div');
    actions.style.cssText = `
      display: flex;
      justify-content: flex-end;
      gap: 10px;
      margin-top: 15px;
    `;
    
    // 复制按钮
    const copyJsonBtn = document.createElement('button');
    copyJsonBtn.textContent = "复制JSON";
    copyJsonBtn.style.cssText = `
      padding: 6px 12px;
      background: #e6f7ff;
      border: 1px solid #91d5ff;
      border-radius: 4px;
      cursor: pointer;
      font-size: 14px;
      color: #1890ff;
    `;
    copyJsonBtn.addEventListener('click', async () => {
      await navigator.clipboard.writeText(jsonFormat);
      copyJsonBtn.textContent = "✓ 已复制";
      setTimeout(() => {
        copyJsonBtn.textContent = "复制JSON";
      }, 2000);
    });
    
    // 组装弹窗
    actions.appendChild(copyJsonBtn);
    dialog.appendChild(header);
    dialog.appendChild(content);
    dialog.appendChild(actions);
    document.body.appendChild(dialog);
    
    // 添加关闭事件
    document.getElementById('closeDialog').addEventListener('click', () => {
      document.body.removeChild(dialog);
      document.body.removeChild(overlay);
    });
    
    // 创建背景遮罩
    const overlay = document.createElement('div');
    overlay.style.cssText = `
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: rgba(0,0,0,0.5);
      z-index: 9998;
    `;
    document.body.appendChild(overlay);
    
    // 点击遮罩关闭弹窗
    overlay.addEventListener('click', () => {
      document.body.removeChild(dialog);
      document.body.removeChild(overlay);
    });
  } catch (err) {
    // 如果剪贴板API失败，使用alert显示
    alert("无法使用剪贴板API，请尝试手动复制调试数据。");
    console.log("Bunching 调试数据:", jsonFormat);
  }
};
</script>

<style scoped>
.bunching-effect {
  @apply text-gray-800;
}

/* 修复RangeMiniViewer的响应式布局问题 */
@media (min-width: 768px) {
  .range-card RangeMiniViewer {
    height: 180px;
  }
}

/* 修复输入数字字段的外观 */
input[type="number"] {
  @apply appearance-none;
  -moz-appearance: textfield;
}

input[type="number"]::-webkit-outer-spin-button,
input[type="number"]::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
</style>
