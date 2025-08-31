<template>
  <div
    v-if="!store.isSolverFinished"
    class="flex w-full max-w-screen-xl mx-auto px-4 py-6 items-center"
  >
    <span
      v-if="store.isSolverRunning || store.isFinalizing"
      class="spinner inline-block mr-3"
    ></span>
    {{
      !store.hasSolverRun
        ? "Solver has not run."
        : store.isSolverRunning
        ? "Solver running..."
        : store.isFinalizing
        ? "Finalizing..."
        : store.isSolverError
        ? "Solver error."
        : "Solver paused."
    }}
  </div>

  <div v-else class="flex flex-col h-full">
    <!-- Version 2 Header -->
    <div class="bg-gradient-to-r from-green-500 to-blue-600 text-white px-4 py-2 text-sm font-semibold flex items-center justify-between">
      <div class="flex items-center space-x-2">
        <span class="bg-white text-green-600 px-2 py-1 rounded text-xs font-bold">V2.0</span>
        <span>升级版结果查看器 - 多标签页增强版本</span>
      </div>
      <div class="flex items-center space-x-4">
        <button 
          @click="toggleEnhancedMode" 
          :class="[
            'px-3 py-1 rounded text-xs transition-colors',
            enhancedMode ? 'bg-yellow-400 text-black' : 'bg-white bg-opacity-20 text-white'
          ]"
        >
          {{ enhancedMode ? '增强模式: 开启' : '增强模式: 关闭' }}
        </button>
        <button 
          @click="showVersionInfo = true"
          class="bg-white bg-opacity-20 text-white px-3 py-1 rounded text-xs hover:bg-opacity-30 transition-colors"
        >
          新功能介绍
        </button>
      </div>
    </div>

    <!-- 多标签页系统 -->
    <TabManager
      ref="tabManager"
      class="flex-1"
      @tab-switched="onTabSwitched"
      @tab-created="onTabCreated"
      @tab-closed="onTabClosed"
      @tab-duplicated="onTabDuplicated"
    >
      <template #default="{ tab, isActive }">
        <div v-if="isActive" class="flex flex-col h-full">
          <!-- 当前标签页的导航组件 -->
          <ResultNav2
            :key="`nav-${tab.id}`"
            :tab-id="tab.id"
            :is-handler-updated="getTabData(tab.id, 'isHandlerUpdated', false)"
            :is-locked="getTabData(tab.id, 'isLocked', false)"
            :cards="getTabData(tab.id, 'cards', [[], []])"
            :dealt-card="getTabData(tab.id, 'dealtCard', -1)"
            @update:is-handler-updated="(value) => setTabData(tab.id, 'isHandlerUpdated', value)"
            @update:is-locked="(value) => setTabData(tab.id, 'isLocked', value)"
            @trigger-update="(selectedSpot, selectedChance, currentBoard, results, chanceReports, totalBetAmount) => 
              onUpdateSpot(tab.id, selectedSpot, selectedChance, currentBoard, results, chanceReports, totalBetAmount)"
          />

          <!-- 当前标签页的中间控制栏 -->
          <ResultMiddle2
            :key="`middle-${tab.id}`"
            :display-mode="getTabData(tab.id, 'displayMode', 'basics')"
            :chance-mode="getTabData(tab.id, 'chanceMode', '')"
            :auto-player-basics="getTabAutoPlayerBasics(tab.id)"
            :auto-player-chance="getTabAutoPlayerChance(tab.id)"
            :copy-success="getTabData(tab.id, 'copySuccess', 0)"
            @update:display-mode="(mode) => updateTabDisplayMode(tab.id, mode)"
            @update:display-options="(options) => updateTabDisplayOptions(tab.id, options)"
            @copy-to-clipboard="() => copyRangeTextToClipboard(tab.id)"
            @reset-copy-success="() => resetTabCopySuccess(tab.id)"
          />

          <!-- 当前标签页的内容区域 -->
          <div
            v-if="store.navView === 'results2' && getTabData(tab.id, 'selectedSpot') && getTabData(tab.id, 'results')"
            class="flex flex-grow min-h-0"
          >
            <template v-if="getTabData(tab.id, 'displayMode', 'basics') === 'basics'">
              <ResultBasics2
                :key="`basics-${tab.id}`"
                style="flex: 4"
                :cards="getTabData(tab.id, 'cards', [[], []])"
                :selected-spot="getTabData(tab.id, 'selectedSpot')"
                :selected-chance="getTabData(tab.id, 'selectedChance')"
                :current-board="getTabData(tab.id, 'currentBoard', [])"
                :total-bet-amount="getTabData(tab.id, 'totalBetAmount', [0, 0])"
                :results="getTabData(tab.id, 'results')"
                :display-options="getTabData(tab.id, 'displayOptions', defaultDisplayOptions)"
                :display-player="getTabDisplayPlayerBasics(tab.id)"
                :is-compare-mode="false"
                @update-hover-content="(content) => onUpdateHoverContent(tab.id, content)"
              />

              <ResultTable2
                :key="`table-${tab.id}`"
                style="flex: 3"
                table-mode="basics"
                :cards="getTabData(tab.id, 'cards', [[], []])"
                :selected-spot="getTabData(tab.id, 'selectedSpot')"
                :results="getTabData(tab.id, 'results')"
                :display-player="getTabDisplayPlayerBasics(tab.id)"
                :hover-content="getTabData(tab.id, 'basicsHoverContent')"
                :table-bar-width="getTabData(tab.id, 'displayOptions', defaultDisplayOptions).barWidth"
                :table-display-content="getTabData(tab.id, 'displayOptions', defaultDisplayOptions).actionContent"
              />
            </template>

            <template v-else-if="getTabData(tab.id, 'displayMode', 'basics') === 'graphs'">
              <ResultGraphs2
                :key="`graphs-${tab.id}`"
                :cards="getTabData(tab.id, 'cards', [[], []])"
                :selected-spot="getTabData(tab.id, 'selectedSpot')"
                :selected-chance="getTabData(tab.id, 'selectedChance')"
                :results="getTabData(tab.id, 'results')"
                :chance-reports="getTabData(tab.id, 'chanceReports')"
                :display-options="getTabData(tab.id, 'displayOptions', defaultDisplayOptions)"
                :display-player="getTabDisplayPlayerBasics(tab.id)"
              />
            </template>

            <template v-else-if="getTabData(tab.id, 'displayMode', 'basics') === 'compare'">
              <ResultBasics2
                :key="`compare-oop-${tab.id}`"
                style="flex: 5"
                :cards="getTabData(tab.id, 'cards', [[], []])"
                :selected-spot="getTabData(tab.id, 'selectedSpot')"
                :selected-chance="getTabData(tab.id, 'selectedChance')"
                :current-board="getTabData(tab.id, 'currentBoard', [])"
                :total-bet-amount="getTabData(tab.id, 'totalBetAmount', [0, 0])"
                :results="getTabData(tab.id, 'results')"
                :display-options="getTabData(tab.id, 'displayOptions', defaultDisplayOptions)"
                display-player="oop"
                :is-compare-mode="true"
              />

              <ResultCompare2
                :key="`compare-middle-${tab.id}`"
                style="flex: 2"
                :selected-spot="getTabData(tab.id, 'selectedSpot')"
                :selected-chance="getTabData(tab.id, 'selectedChance')"
                :results="getTabData(tab.id, 'results')"
              />

              <ResultBasics2
                :key="`compare-ip-${tab.id}`"
                style="flex: 5"
                :cards="getTabData(tab.id, 'cards', [[], []])"
                :selected-spot="getTabData(tab.id, 'selectedSpot')"
                :selected-chance="getTabData(tab.id, 'selectedChance')"
                :current-board="getTabData(tab.id, 'currentBoard', [])"
                :total-bet-amount="getTabData(tab.id, 'totalBetAmount', [0, 0])"
                :results="getTabData(tab.id, 'results')"
                :display-options="getTabData(tab.id, 'displayOptions', defaultDisplayOptions)"
                display-player="ip"
                :is-compare-mode="true"
              />
            </template>

            <template v-else-if="getTabData(tab.id, 'displayMode', 'basics') === 'chance' && getTabData(tab.id, 'selectedChance')">
              <ResultChance2
                :key="`chance-${tab.id}`"
                :selected-spot="getTabData(tab.id, 'selectedSpot')"
                :selected-chance="getTabData(tab.id, 'selectedChance')"
                :chance-reports="getTabData(tab.id, 'chanceReports')"
                :display-options="getTabData(tab.id, 'displayOptions', defaultDisplayOptions)"
                :display-player="getTabDisplayPlayerChance(tab.id)"
                @deal-card="(card) => onDealCard(tab.id, card)"
              />
            </template>
          </div>
        </div>
      </template>
    </TabManager>

    <!-- Debug Button -->
    <div class="debug-button-container">
      <button 
        class="debug-button"
        @click="showDebugModal = true"
        title="Debug Data"
      >
        Debug
      </button>
      
      <button 
        class="path-button"
        @click="printDecisionPathToConsole"
        title="打印GTO决策路径并保存到文件"
      >
        决策路径
      </button>
      <span v-if="pathConsoleStatus === 1" class="path-success-status">已发送到控制台并保存!</span>
      <span v-if="pathConsoleStatus === 2" class="path-loading-status">处理中...</span>
      <span v-if="pathConsoleStatus === -1" class="path-error-status">发送失败</span>
    </div>
  </div>

  <!-- Debug Modal -->
  <div v-if="showDebugModal" class="debug-modal-overlay">
    <div class="debug-modal">
      <div class="debug-modal-header">
        <h3>Debug Data</h3>
        <button @click="showDebugModal = false" class="close-button">×</button>
      </div>
      <div class="debug-modal-body">
        <div class="debug-actions">
          <button @click="copyDebugToClipboard" class="copy-button">Copy to clipboard</button>
          <button @click="printToRustConsole" class="console-button">打印并保存到文件</button>
          <span v-if="debugCopyStatus === 1" class="copy-success">Copied!</span>
          <span v-if="debugCopyStatus === -1" class="copy-error">Copy failed</span>
          <span v-if="debugConsoleStatus === 1" class="copy-success">已发送到控制台并保存!</span>
          <span v-if="debugConsoleStatus === 2" class="copy-loading">处理中...</span>
          <span v-if="debugConsoleStatus === -1" class="copy-error">操作失败</span>
        </div>
        <pre class="debug-content">{{ prettyDebugData }}</pre>
      </div>
    </div>
  </div>

  <!-- Version Info Modal -->
  <div v-if="showVersionInfo" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg shadow-xl w-4/5 max-w-2xl max-h-[80vh] overflow-hidden flex flex-col">
      <div class="p-4 border-b border-gray-200 flex justify-between items-center bg-gradient-to-r from-green-500 to-blue-600">
        <h3 class="text-lg font-semibold text-white">🚀 View Results 2.0 - 新功能介绍</h3>
        <button @click="showVersionInfo = false" class="text-white hover:text-gray-200">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
      <div class="p-6 overflow-y-auto flex-1">
        <div class="space-y-4">
          <div class="bg-green-50 border-l-4 border-green-400 p-4">
            <h4 class="font-semibold text-green-800 mb-2">🔥 多标签页系统</h4>
            <p class="text-green-700">全新的多标签页功能！现在可以同时打开多个决策路径，在不同的FLOP、TURN、RIVER阶段之间快速切换对比。每个标签页都有独立的导航状态和用户选择记录。</p>
          </div>
          
          <div class="bg-blue-50 border-l-4 border-blue-400 p-4">
            <h4 class="font-semibold text-blue-800 mb-2">📊 并行决策分析</h4>
            <p class="text-blue-700">支持在多个标签页中同时进行不同的决策路径分析，轻松对比OOP和IP在不同情况下的最优策略。可以复制当前标签页状态到新标签页进行分支分析。</p>
          </div>
          
          <div class="bg-purple-50 border-l-4 border-purple-400 p-4">
            <h4 class="font-semibold text-purple-800 mb-2">🎛️ 独立状态管理</h4>
            <p class="text-purple-700">每个标签页都有完全独立的状态管理，包括选择的行动、卡片发放、显示模式等。切换标签页时自动保存和恢复状态，不会互相干扰。</p>
          </div>
          
          <div class="bg-yellow-50 border-l-4 border-yellow-400 p-4">
            <h4 class="font-semibold text-yellow-800 mb-2">⚡ 增强用户体验</h4>
            <p class="text-yellow-700">标签页可以重命名、复制、关闭，支持快捷键操作。右键菜单提供更多管理选项，包括关闭其他标签页、关闭所有标签页等便捷功能。</p>
          </div>
        </div>
      </div>
      <div class="p-4 border-t border-gray-200 flex justify-end">
        <button @click="showVersionInfo = false" class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700">
          开始使用
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useStore, useSavedConfigStore } from "../store";
import * as invokes from "../invokes";

// Tauri imports
import { invoke } from "@tauri-apps/api/tauri";
import { save } from '@tauri-apps/api/dialog';
import { writeTextFile } from '@tauri-apps/api/fs';

import {
  Results,
  ChanceReports,
  Spot,
  SpotChance,
  SpotPlayer,
  DisplayMode,
  DisplayOptions,
  HoverContent,
} from "../result-types";

import TabManager, { TabData } from "./TabManager.vue";
import ResultNav2 from "./ResultNav2.vue";
import ResultMiddle2 from "./ResultMiddle2.vue";
import ResultBasics2 from "./ResultBasics2.vue";
import ResultTable2 from "./ResultTable2.vue";
import ResultCompare2 from "./ResultCompare2.vue";
import ResultGraphs2 from "./ResultGraphs2.vue";
import ResultChance2 from "./ResultChance2.vue";

const store = useStore();

/* Version 2 Enhanced Features */
const enhancedMode = ref(false);
const showVersionInfo = ref(false);

const toggleEnhancedMode = () => {
  enhancedMode.value = !enhancedMode.value;
  console.log(`Enhanced mode ${enhancedMode.value ? 'enabled' : 'disabled'}`);
};

/* 多标签页状态管理 */
const tabManager = ref<InstanceType<typeof TabManager>>();
const tabsData = ref<Map<string, any>>(new Map());

// 默认显示选项
const defaultDisplayOptions: DisplayOptions = {
  playerBasics: "auto",
  playerChance: "auto",
  barHeight: "normalized",
  suit: "grouped",
  strategy: "show",
  contentBasics: "default",
  contentGraphs: "eq",
  chartChance: "strategy-combos",
};

const isSolverFinished = ref(false);
store.$subscribe(async (_, store) => {
  if (isSolverFinished.value !== store.isSolverFinished) {
    if ((isSolverFinished.value = store.isSolverFinished)) {
      await initAllTabs();
    } else {
      clearAllTabs();
    }
  }
});

// 添加对 dataRefreshTrigger 的监听，当它变化时重新初始化数据
watch(() => store.dataRefreshTrigger, async () => {
  if (store.isSolverFinished) {
    await initAllTabs();
  }
});

// 初始化所有标签页
const initAllTabs = async () => {
  try {
    // 获取游戏配置信息
    const configInfo = await invokes.gameGetConfigInfo();
    
    console.log("初始化游戏状态:", {
      board: configInfo.board.map(card => card),
      startingPot: configInfo.starting_pot,
      effectiveStack: configInfo.effective_stack
    });
    
    // 获取卡牌数据
    const cards = await invokes.gamePrivateCards();
    
    // 应用历史为空数组，返回到游戏树的根节点
    await invokes.gameApplyHistory([]);
    
    // 确保配置数据是最新的
    const useConfigStore = useSavedConfigStore();
    useConfigStore.board = configInfo.board;
    useConfigStore.startingPot = configInfo.starting_pot;
    useConfigStore.effectiveStack = configInfo.effective_stack;
    
    // 为所有现有标签页初始化数据
    if (tabManager.value?.tabs) {
      for (const tab of tabManager.value.tabs) {
        await initTabData(tab.id, cards);
      }
    }
  } catch (error) {
    console.error("Error initializing tabs:", error);
  }
};

// 初始化单个标签页数据
const initTabData = async (tabId: string, cards?: number[][]) => {
  try {
    if (!cards) {
      cards = await invokes.gamePrivateCards();
    }
    
    const tabData = {
      isHandlerUpdated: true,
      isLocked: false,
      cards: cards,
      dealtCard: -1,
      selectedSpot: null,
      selectedChance: null,
      currentBoard: [],
      results: null,
      chanceReports: null,
      totalBetAmount: [0, 0],
      displayMode: 'basics' as DisplayMode,
      chanceMode: '',
      displayOptions: { ...defaultDisplayOptions },
      copySuccess: 0,
      basicsHoverContent: null,
    };
    
    tabsData.value.set(tabId, tabData);
  } catch (error) {
    console.error(`Error initializing tab ${tabId}:`, error);
  }
};

// 清除所有标签页数据
const clearAllTabs = () => {
  tabsData.value.clear();
};

// 标签页数据访问方法
const getTabData = (tabId: string, key: string, defaultValue?: any) => {
  const tabData = tabsData.value.get(tabId);
  return tabData?.[key] ?? defaultValue;
};

const setTabData = (tabId: string, key: string, value: any) => {
  const tabData = tabsData.value.get(tabId);
  if (tabData) {
    tabData[key] = value;
  }
};

// 标签页事件处理
const onTabSwitched = (tabId: string, tab: TabData) => {
  console.log(`Switched to tab: ${tabId}`);
  // 如果标签页没有数据，初始化它
  if (!tabsData.value.has(tabId)) {
    initTabData(tabId);
  }
};

const onTabCreated = async (tabId: string, tab: TabData) => {
  console.log(`Created new tab: ${tabId}`);
  // 为新标签页初始化数据
  await initTabData(tabId);
};

const onTabClosed = (tabId: string) => {
  console.log(`Closed tab: ${tabId}`);
  // 清除标签页数据
  tabsData.value.delete(tabId);
};

const onTabDuplicated = (originalTabId: string, newTabId: string) => {
  console.log(`Duplicated tab ${originalTabId} to ${newTabId}`);
  // 复制原标签页的数据到新标签页
  const originalData = tabsData.value.get(originalTabId);
  if (originalData) {
    const newData = JSON.parse(JSON.stringify(originalData));
    tabsData.value.set(newTabId, newData);
  }
};

// 更新标签页的导航状态
const onUpdateSpot = (
  tabId: string,
  newSelectedSpot: Spot | null,
  newSelectedChance: SpotChance | null,
  newCurrentBoard: number[],
  newResults: Results,
  newChanceReports: ChanceReports | null,
  newTotalBetAmount: number[]
) => {
  const tabData = tabsData.value.get(tabId);
  if (!tabData) return;

  tabData.dealtCard = -1;
  tabData.selectedSpot = newSelectedSpot;
  tabData.selectedChance = newSelectedChance;
  tabData.currentBoard = newCurrentBoard;
  tabData.results = newResults;
  tabData.chanceReports = newChanceReports;
  tabData.totalBetAmount = newTotalBetAmount;
  tabData.isLocked = false;
  tabData.chanceMode = newSelectedChance?.player ?? "";
};

/* 标签页显示模式和选项管理 */

const updateTabDisplayMode = (tabId: string, mode: DisplayMode) => {
  setTabData(tabId, 'displayMode', mode);
};

const updateTabDisplayOptions = (tabId: string, options: DisplayOptions) => {
  setTabData(tabId, 'displayOptions', options);
};

const copyRangeTextToClipboard = async (tabId: string) => {
  const text = "Hello World";
  try {
    await navigator.clipboard.writeText(text);
    setTabData(tabId, 'copySuccess', 1);
  } catch {
    setTabData(tabId, 'copySuccess', -1);
  }
};

const resetTabCopySuccess = (tabId: string) => {
  setTabData(tabId, 'copySuccess', 0);
};

/* 标签页计算属性 */

const getTabAutoPlayerBasics = (tabId: string) => {
  const spot = getTabData(tabId, 'selectedSpot');
  const chance = getTabData(tabId, 'selectedChance');
  if (!spot) return "oop";

  if (chance) {
    return chance.prevPlayer;
  } else if (spot.type === "terminal") {
    return spot.prevPlayer;
  } else {
    return (spot as SpotPlayer).player;
  }
};

const getTabAutoPlayerChance = (tabId: string) => {
  const spot = getTabData(tabId, 'selectedSpot');
  if (!spot) return "oop";
  if (spot.type === "terminal") {
    return spot.prevPlayer;
  } else {
    return (spot as SpotPlayer).player;
  }
};

const getTabDisplayPlayerBasics = (tabId: string) => {
  const displayOptions = getTabData(tabId, 'displayOptions', defaultDisplayOptions);
  const optionPlayer = displayOptions.playerBasics;
  if (optionPlayer === "auto") {
    return getTabAutoPlayerBasics(tabId);
  } else {
    return optionPlayer;
  }
};

const getTabDisplayPlayerChance = (tabId: string) => {
  const displayOptions = getTabData(tabId, 'displayOptions', defaultDisplayOptions);
  const optionPlayer = displayOptions.playerChance;
  if (optionPlayer === "auto") {
    return getTabAutoPlayerChance(tabId);
  } else {
    return optionPlayer;
  }
};

/* 标签页结果处理 */

const onUpdateHoverContent = (tabId: string, content: HoverContent | null) => {
  setTabData(tabId, 'basicsHoverContent', content);
};

const onDealCard = (tabId: string, card: number) => {
  setTabData(tabId, 'dealtCard', card);
};

/* Debug functionality */
const showDebugModal = ref(false);
const debugCopyStatus = ref(0);
const debugConsoleStatus = ref(0);
const pathConsoleStatus = ref(0);

// Collect all relevant data in a single JSON object
const debugData = computed(() => {
  return {
    // Store data
    store: {
      isSolverFinished: store.isSolverFinished,
      isSolverRunning: store.isSolverRunning,
      hasSolverRun: store.hasSolverRun,
      isFinalizing: store.isFinalizing,
      isSolverError: store.isSolverError,
      navView: store.navView,
    },
    // Component state
    componentState: {
      isHandlerUpdated: isHandlerUpdated.value,
      isLocked: isLocked.value,
      cards: cards.value,
      dealtCard: dealtCard.value,
      displayMode: displayMode.value,
      chanceMode: chanceMode.value,
      displayOptions: displayOptions.value,
      copySuccess: copySuccess.value,
    },
    // Data flow records
    dataFlow: {
      toResultNav: {
        isHandlerUpdated: isHandlerUpdated.value,
        isLocked: isLocked.value,
        cards: cards.value,
        dealtCard: dealtCard.value,
      },
      fromResultNav: {
        onTriggerUpdateReceived: {
          selectedSpot: selectedSpot.value,
          selectedChance: selectedChance.value,
          currentBoard: currentBoard.value,
          results: results.value,
          chanceReports: chanceReports.value,
          totalBetAmount: totalBetAmount.value,
        }
      },
      toResultMiddle: {
        displayMode: displayMode.value,
        chanceMode: chanceMode.value,
        autoPlayerBasics: autoPlayerBasics.value,
        autoPlayerChance: autoPlayerChance.value,
        copySuccess: copySuccess.value,
      },
      toResultBasics: {
        cards: cards.value,
        selectedSpot: selectedSpot.value,
        selectedChance: selectedChance.value,
        currentBoard: currentBoard.value,
        totalBetAmount: totalBetAmount.value,
        results: results.value,
        displayOptions: displayOptions.value,
        displayPlayer: displayPlayerBasics.value,
        isCompareMode: displayMode.value === 'compare',
      },
      toResultTable: {
        cards: cards.value,
        selectedSpot: selectedSpot.value,
        results: results.value,
        displayPlayer: displayPlayerBasics.value,
        hoverContent: basicsHoverContent.value,
        tableBarWidth: displayOptions.value.barWidth,
        tableDisplayContent: displayOptions.value.actionContent,
      },
      toResultGraphs: {
        cards: cards.value,
        selectedSpot: selectedSpot.value,
        selectedChance: selectedChance.value,
        results: results.value,
        chanceReports: chanceReports.value,
        displayOptions: displayOptions.value,
        displayPlayer: displayPlayerBasics.value,
      },
      toResultCompare: {
        selectedSpot: selectedSpot.value,
        selectedChance: selectedChance.value,
        results: results.value,
      },
      toResultChance: {
        selectedSpot: selectedSpot.value,
        selectedChance: selectedChance.value,
        chanceReports: chanceReports.value,
        displayOptions: displayOptions.value,
        displayPlayer: displayPlayerChance.value,
      },
    },
    // Raw data objects
    rawData: {
      selectedSpot: selectedSpot.value,
      selectedChance: selectedChance.value,
      currentBoard: currentBoard.value,
      results: results.value,
      chanceReports: chanceReports.value,
      totalBetAmount: totalBetAmount.value,
      basicsHoverContent: basicsHoverContent.value,
    },
    // Debug metadata
    meta: {
      timestamp: new Date().toISOString(),
      version: "1.0.0",
    }
  };
});

const prettyDebugData = computed(() => {
  try {
    return JSON.stringify(debugData.value, null, 2);
  } catch (error) {
    return `Error generating JSON: ${error}`;
  }
});

const copyDebugToClipboard = async () => {
  try {
    await navigator.clipboard.writeText(prettyDebugData.value);
    debugCopyStatus.value = 1;
    setTimeout(() => {
      debugCopyStatus.value = 0;
    }, 2000);
  } catch (error) {
    debugCopyStatus.value = -1;
    setTimeout(() => {
      debugCopyStatus.value = 0;
    }, 2000);
  }
};

// 添加打印到Rust控制台的函数
const printToRustConsole = async () => {
  try {
    // 显示加载状态
    debugConsoleStatus.value = 2; // 2表示加载中
    
    // 构建包含元数据的调试信息
    const debugInfo = {
      timestamp: new Date().toISOString(),
      source: "ResultViewer Debug Data",
      data: debugData.value
    };
    
    const debugContent = JSON.stringify(debugInfo, null, 2);
    
    // 调用后端命令将数据发送到控制台
    await invoke('log_to_console', { 
      message: debugContent
    });
    
    // 保存到文件
    try {
      // 弹出保存对话框
      const filePath = await save({
        filters: [{
          name: 'JSON',
          extensions: ['json']
        }],
        defaultPath: `debug-data-${new Date().toISOString().replace(/:/g, '-')}.json`
      });
      
      if (filePath) {
        await writeTextFile(filePath, debugContent);
        console.log("Debug data saved to file:", filePath);
      }
    } catch (fileError) {
      console.error("Failed to save debug data to file:", fileError);
    }
    
    // 显示成功状态
    debugConsoleStatus.value = 1;
    setTimeout(() => {
      debugConsoleStatus.value = 0;
    }, 2000);
  } catch (error) {
    // 显示错误状态
    console.error("Failed to send data to console:", error);
    debugConsoleStatus.value = -1;
    setTimeout(() => {
      debugConsoleStatus.value = 0;
    }, 2000);
  }
};

// 添加打印决策路径到Rust控制台的函数
const printDecisionPathToConsole = async () => {
  try {
    // 显示加载状态
    pathConsoleStatus.value = 2; // 2表示加载中
    
    // 获取完整决策路径
    const pathData = await invokes.getCompleteDecisionPath();
    
    // 构建包含元数据的调试信息
    const pathInfo = {
      timestamp: new Date().toISOString(),
      source: "GTO Complete Decision Path",
      data: pathData
    };
    
    const pathContent = JSON.stringify(pathInfo, null, 2);
    
    // 调用后端命令将数据发送到控制台
    await invoke('log_to_console', { 
      message: pathContent
    });
    
    // 保存到文件
    try {
      // 弹出保存对话框
      const filePath = await save({
        filters: [{
          name: 'JSON',
          extensions: ['json']
        }],
        defaultPath: `decision-path-${new Date().toISOString().replace(/:/g, '-')}.json`
      });
      
      if (filePath) {
        await writeTextFile(filePath, pathContent);
        console.log("Decision path saved to file:", filePath);
      }
    } catch (fileError) {
      console.error("Failed to save decision path to file:", fileError);
    }
    
    // 显示成功状态
    pathConsoleStatus.value = 1;
    setTimeout(() => {
      pathConsoleStatus.value = 0;
    }, 2000);
  } catch (error) {
    // 显示错误状态
    console.error("Failed to get or send decision path data:", error);
    pathConsoleStatus.value = -1;
    setTimeout(() => {
      pathConsoleStatus.value = 0;
    }, 2000);
  }
};
</script>

<style scoped>
.debug-button-container {
  position: fixed;
  right: 20px;
  bottom: 20px;
  z-index: 10;
  display: flex;
  flex-direction: column;
  gap: 10px;
  align-items: flex-end;
}

.debug-button, .path-button {
  background-color: #ff5500;
  color: white;
  border: none;
  border-radius: 4px;
  padding: 8px 16px;
  font-weight: bold;
  cursor: pointer;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
  min-width: 100px;
}

.debug-button:hover {
  background-color: #ff7733;
}

.path-button {
  background-color: #8b5cf6;
}

.path-button:hover {
  background-color: #9d6dff;
}

.path-success-status {
  position: absolute;
  bottom: 70px;
  right: 20px;
  color: #2ecc71;
  background-color: rgba(255, 255, 255, 0.9);
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 12px;
  white-space: nowrap;
}

.path-error-status {
  position: absolute;
  bottom: 70px;
  right: 20px;
  color: #e74c3c;
  background-color: rgba(255, 255, 255, 0.9);
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 12px;
  white-space: nowrap;
}

.path-loading-status {
  position: absolute;
  bottom: 70px;
  right: 20px;
  color: #f39c12;
  background-color: rgba(255, 255, 255, 0.9);
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 12px;
  white-space: nowrap;
}

.debug-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 100;
}

.debug-modal {
  background-color: white;
  border-radius: 4px;
  width: 90%;
  max-width: 1200px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
}

.debug-modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 20px;
  border-bottom: 1px solid #eee;
}

.debug-modal-header h3 {
  margin: 0;
  font-size: 1.2rem;
}

.close-button {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: #666;
}

.debug-modal-body {
  padding: 20px;
  overflow-y: auto;
  flex-grow: 1;
}

.debug-actions {
  margin-bottom: 15px;
  display: flex;
  align-items: center;
  gap: 10px;
}

.copy-button, .console-button {
  background-color: #3498db;
  color: white;
  border: none;
  border-radius: 4px;
  padding: 6px 12px;
  cursor: pointer;
  margin-right: 8px;
}

.copy-button:hover, .console-button:hover {
  background-color: #2980b9;
}

.console-button {
  background-color: #27ae60;
}

.console-button:hover {
  background-color: #219653;
}

.copy-success {
  color: #2ecc71;
}

.copy-error {
  color: #e74c3c;
}

.copy-loading {
  color: #f39c12;
}

.debug-content {
  background-color: #f8f8f8;
  border: 1px solid #ddd;
  border-radius: 4px;
  padding: 15px;
  max-height: 70vh;
  overflow-y: auto;
  white-space: pre-wrap;
  font-family: monospace;
  font-size: 0.9rem;
}
</style>
