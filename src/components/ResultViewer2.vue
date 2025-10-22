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
        <span>Upgraded Results Viewer – Multi-Tab Enhanced Version</span>
      </div>
      <div class="flex items-center space-x-4">
        <button 
          @click="toggleEnhancedMode" 
          :class="[
            'px-3 py-1 rounded text-xs transition-colors',
            enhancedMode ? 'bg-yellow-400 text-black' : 'bg-white bg-opacity-20 text-white'
          ]"
        >
          {{ enhancedMode ? 'Enhanced Mode: On' : 'Enhanced Mode: Off' }}
        </button>
        <button 
          @click="showVersionInfo = true"
          class="bg-white bg-opacity-20 text-white px-3 py-1 rounded text-xs hover:bg-opacity-30 transition-colors"
        >
          New Features
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
            :ref="(el) => { if (el) setNavRef(tab.id, el); }"
            :key="`nav-${tab.id}`"
            :tab-id="tab.id"
            :is-handler-updated="getTabData(tab.id, 'isHandlerUpdated', false)"
            :is-locked="getTabData(tab.id, 'isLocked', false)"
            :cards="getTabData(tab.id, 'cards', [[], []])"
            :dealt-card="getTabData(tab.id, 'dealtCard', -1)"
            @update:is-handler-updated="(value) => setTabData(tab.id, 'isHandlerUpdated', value)"
            @update:is-locked="(value) => setTabData(tab.id, 'isLocked', value)"
            @state-changed="onNavStateChanged"
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
    <div class="debug-button-container" style="display: none;">
      <button 
        class="debug-button"
        @click="showDebugModal = true"
        title="Debug Data"
      >
        Debug
      </button>
      
      <button 
        class="tab-debug-button"
        @click="showTabDebugModal = true"
        title="Multi-Tab State Debug"
      >
        Tab Debug
      </button>
      
      <button 
        class="path-button"
        @click="printDecisionPathToConsole"
        title="Print GTO Decision Path and Save to File"
      >
        Decision Path
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
          <button @click="printToRustConsole" class="console-button">Print & Save to File</button>
          <span v-if="debugCopyStatus === 1" class="copy-success">Copied!</span>
          <span v-if="debugCopyStatus === -1" class="copy-error">Copy failed</span>
          <span v-if="debugConsoleStatus === 1" class="copy-success">Sent to console and saved!</span>
          <span v-if="debugConsoleStatus === 2" class="copy-loading">Processing...</span>
          <span v-if="debugConsoleStatus === -1" class="copy-error">Operation failed</span>
        </div>
        <pre class="debug-content">{{ prettyDebugData }}</pre>
      </div>
    </div>
  </div>

  <!-- Version Info Modal -->
  <div v-if="showVersionInfo" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg shadow-xl w-4/5 max-w-2xl max-h-[80vh] overflow-hidden flex flex-col">
      <div class="p-4 border-b border-gray-200 flex justify-between items-center bg-gradient-to-r from-green-500 to-blue-600">
        <h3 class="text-lg font-semibold text-white">🚀 View Results 2.0 - New Features Introduction</h3>
        <button @click="showVersionInfo = false" class="text-white hover:text-gray-200">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
      <div class="p-6 overflow-y-auto flex-1">
        <div class="space-y-4">
          <div class="bg-green-50 border-l-4 border-green-400 p-4">
            <h4 class="font-semibold text-green-800 mb-2">🎯 Purpose of the Multi-Tab System</h4>
            <p class="text-green-700">The purpose of the multi-tab system is to allow you, whether seated as SB, BB, MP, CO, BTN, or any position in a 9-handed game, to open a dedicated tab for that role and analyze matchups against other positions. Each role has its own independent tab with separate navigation and decision states, enabling parallel comparison of strategies across different seats.</p>
          </div>
          
          <div class="bg-yellow-50 border-l-4 border-yellow-400 p-4">
            <h4 class="font-semibold text-yellow-800 mb-2">⚡ Enhanced User Experience</h4>
            <p class="text-yellow-700">You can rename a tab by double-clicking it. For example, you can rename Decision Path 1 to SB, Decision Path 2 to BB, Decision Path 3 to CO, and Decision Path 4 to BTN. This provides a more intuitive and efficient user experience.</p>
          </div>

          <div class="bg-purple-50 border-l-4 border-purple-400 p-4">
            <h4 class="font-semibold text-purple-800 mb-2">🎛️ Independent State Management</h4>
            <p class="text-purple-700">Each tab has completely independent state management, including selected actions, card deals, display modes, etc. When switching tabs, states are automatically saved and restored without interfering with each other.</p>
          </div>
        </div>
      </div>
      <div class="p-4 border-t border-gray-200 flex justify-end">
        <button @click="showVersionInfo = false" class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700">
          Get Started
        </button>
      </div>
    </div>
  </div>

  <!-- Tab Debug Modal -->
  <div v-if="showTabDebugModal" class="debug-modal-overlay">
    <div class="tab-debug-modal">
      <div class="debug-modal-header">
        <h3>🔧 Multi-Tab State Debugger</h3>
        <button @click="showTabDebugModal = false" class="close-button">×</button>
      </div>
      <div class="debug-modal-body">
        <div class="debug-actions">
          <button @click="refreshDebugInfo" class="refresh-button">🔄 Refresh Status</button>
          <button @click="testSaveAllTabs" class="test-button">💾 Test Save All Tabs</button>
          <button @click="testRestoreCurrentTab" class="test-button">📥 Test Restore Current Tab</button>
          <button @click="testSaveCurrentTabNav" class="test-button" title="Manually save the navigation state of the current active tab (fallback function, normally saves automatically)">💾 Manual Save</button>
          <button @click="checkRefsStatus" class="test-button">🔍 Check Reference Status</button>
          <button @click="exportTabDebugData" class="export-button">📤 Export Debug Data</button>
          <button @click="clearAllTabStates" class="clear-button">🗑️ Clear All States</button>
        </div>
        
        <div class="debug-sections">
          <!-- 基础信息 -->
          <div class="debug-section">
            <h4>📊 Basic Status Information</h4>
            <div class="debug-info">
              <div><strong>Current Active Tab:</strong> {{ currentActiveTabId || 'None' }}</div>
              <div><strong>Total Tabs:</strong> {{ tabManager?.tabs?.length || 0 }}</div>
              <div><strong>Data Storage Count:</strong> {{ tabsData.size }}</div>
              <div><strong>Navigation Reference Count:</strong> {{ navRefs.size }}</div>
              <div><strong>Switch Lock Status:</strong> {{ switchingLock ? '🔒 Locked' : '🔓 Unlocked' }}</div>
            </div>
          </div>

          <!-- 标签页详细信息 -->
          <div class="debug-section">
            <h4>📋 Tab Detailed Status</h4>
            <div class="tabs-debug-list">
              <div 
                v-for="tab in tabManager?.tabs || []" 
                :key="tab.id"
                :class="['tab-debug-item', { 'active': tab.id === currentActiveTabId }]"
              >
                <div class="tab-debug-header">
                  <span class="tab-name">{{ tab.name }}</span>
                  <span class="tab-id">ID: {{ tab.id }}</span>
                  <span v-if="tab.id === currentActiveTabId" class="active-indicator">🟢 Active</span>
                </div>
                
                <div class="tab-debug-details">
                  <div class="debug-row">
                    <span>Data Storage:</span>
                    <span :class="tabsData.has(tab.id) ? 'status-good' : 'status-bad'">
                      {{ tabsData.has(tab.id) ? '✅ Exists' : '❌ Missing' }}
                    </span>
                  </div>
                  
                  <div class="debug-row">
                    <span>Navigation Reference:</span>
                    <span :class="navRefs.has(tab.id) ? 'status-good' : 'status-bad'">
                      {{ navRefs.has(tab.id) ? '✅ Exists' : '❌ Missing' }}
                    </span>
                  </div>
                  
                  <div v-if="tabsData.has(tab.id)" class="tab-state-details">
                    <div class="debug-row">
                      <span>Navigation State:</span>
                      <span :class="getTabNavStateStatus(tab.id).class">
                        {{ getTabNavStateStatus(tab.id).text }}
                      </span>
                    </div>

                    <div class="debug-row">
                      <span>Spots Count:</span>
                      <span>{{ getTabData(tab.id, 'navState')?.spots?.length || 0 }}</span>
                    </div>

                    <div class="debug-row">
                      <span>Selected Spot:</span>
                      <span>{{ getTabData(tab.id, 'navState')?.selectedSpotIndex ?? 'None' }}</span>
                    </div>

                    <div class="debug-row">
                      <span>Selected Chance:</span>
                      <span>{{ getTabData(tab.id, 'navState')?.selectedChanceIndex ?? 'None' }}</span>
                    </div>

                    <div class="debug-row">
                      <span>Last Saved:</span>
                      <span>{{ formatTime(getTabData(tab.id, 'lastSavedAt')) }}</span>
                    </div>

                    <div class="debug-row">
                      <span>Display Mode:</span>
                      <span>{{ getTabData(tab.id, 'displayMode') || 'Not Set' }}</span>
                    </div>

                    <div class="debug-row">
                      <span>Results Data:</span>
                      <span :class="getTabData(tab.id, 'results') ? 'status-good' : 'status-warning'">
                        {{ getTabData(tab.id, 'results') ? '✅ Has Data' : '⚠️ No Data' }}
                      </span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 引用状态检查 -->
          <div class="debug-section">
            <h4>🔗 Component Reference Status</h4>
            <div class="refs-debug-info">
              <div v-for="[tabId, ref] of navRefs" :key="tabId" class="ref-debug-item">
                <div><strong>{{ tabId }}:</strong></div>
                <div class="debug-row">
                  <span>Reference Type:</span>
                  <span>{{ typeof ref }}</span>
                </div>
                <div class="debug-row">
                  <span>Export Method:</span>
                  <span :class="typeof ref?.exportCurrentState === 'function' ? 'status-good' : 'status-bad'">
                    {{ typeof ref?.exportCurrentState === 'function' ? '✅ Available' : '❌ Unavailable' }}
                  </span>
                </div>
                <div class="debug-row">
                  <span>Import Method:</span>
                  <span :class="typeof ref?.importState === 'function' ? 'status-good' : 'status-bad'">
                    {{ typeof ref?.importState === 'function' ? '✅ Available' : '❌ Unavailable' }}
                  </span>
                </div>
              </div>
            </div>
          </div>

          <!-- 操作日志 -->
          <div class="debug-section">
            <h4>📝 Operation Logs (Last 10)</h4>
            <div class="debug-logs">
              <div 
                v-for="(log, index) in debugLogs.slice(-10)" 
                :key="index"
                :class="['debug-log-item', `log-${log.type}`]"
              >
                <span class="log-time">{{ formatTime(log.timestamp) }}</span>
                <span class="log-type">{{ log.type.toUpperCase() }}</span>
                <span class="log-message">{{ log.message }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch, nextTick, onUnmounted, onMounted } from "vue";
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
const navRefs = ref<Map<string, any>>(new Map());

// 在组件挂载时，同步一次当前活跃标签ID，避免依赖ref回调触发活跃ID更新
onMounted(() => {
  const activeId = tabManager.value?.activeTab?.id;
  if (activeId) {
    currentActiveTabId.value = activeId;
    addDebugLog('info', `Initial active tab: ${activeId}`);
  }
});

// 默认显示选项
const defaultDisplayOptions: DisplayOptions = {
  playerBasics: "auto",
  playerChance: "auto",
  barHeight: "normalized",
  suit: "grouped",
  strategy: "none",
  contentBasics: "ev",
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
      // 基础状态
      isHandlerUpdated: true,
      isLocked: false,
      cards: cards,
      dealtCard: -1,
      
      // 导航状态
      selectedSpot: null,
      selectedChance: null,
      currentBoard: [],
      results: null,
      chanceReports: null,
      totalBetAmount: [0, 0],
      
      // 显示状态
      displayMode: 'basics' as DisplayMode,
      chanceMode: '',
      displayOptions: { ...defaultDisplayOptions },
      copySuccess: 0,
      basicsHoverContent: null,
      
      // 导航历史和状态持久化
      navigationHistory: null,
      navState: null,  // 导航状态（由 ResultNav2 导出的完整状态）
      lastSavedAt: new Date(),
      
      // 标签页元数据
      createdAt: new Date(),
      name: `Decision Path ${tabsData.value.size + 1}`,
    };
    
    tabsData.value.set(tabId, tabData);
    console.log(`Initialized tab ${tabId} with fresh data`);
  } catch (error) {
    console.error(`Error initializing tab ${tabId}:`, error);
  }
};

// 清除所有标签页数据
const clearAllTabs = () => {
  tabsData.value.clear();
};

// 当前活跃的标签页ID
const currentActiveTabId = ref<string>('');

// 保存当前活跃标签页的状态
const saveCurrentTabState = async () => {
  const currentTab = tabManager.value?.activeTab;
  if (!currentTab) return;
  
  try {
    const tabData = tabsData.value.get(currentTab.id);
    if (tabData) {
      // 标记状态已保存的时间
      tabData.lastSavedAt = new Date();
      
      console.log(`Saved state for tab ${currentTab.id}:`, {
        selectedSpot: tabData.selectedSpot?.type,
        selectedChance: tabData.selectedChance?.player,
        displayMode: tabData.displayMode,
        results: !!tabData.results
      });
    }
  } catch (error) {
    console.error(`Error saving tab state for ${currentTab.id}:`, error);
  }
};

// 保存当前活跃标签页的导航状态
const saveCurrentTabNavState = async () => {
  if (!currentActiveTabId.value) {
    console.log('No active tab to save state for');
    return;
  }

  try {
    const tabId = currentActiveTabId.value;
    const navRef = getNavRef(tabId);
    const tabData = tabsData.value.get(tabId);

    console.log(`Attempting to save navigation state for tab ${tabId}:`, {
      hasNavRef: !!navRef,
      hasExportMethod: !!(navRef?.exportCurrentState),
      hasTabData: !!tabData
    });

    if (navRef && typeof navRef.exportCurrentState === 'function' && tabData) {
      const navState = navRef.exportCurrentState();

      console.log(`Navigation state exported for tab ${tabId}:`, {
        hasNavState: !!navState,
        hasSpots: !!(navState?.spots),
        spotsCount: navState?.spots?.length || 0,
        spotsArray: navState?.spots,
        selectedSpotIndex: navState?.selectedSpotIndex,
        selectedChanceIndex: navState?.selectedChanceIndex,
        exportedAt: navState?.exportedAt,
        hasRates: !!(navState?.rates)
      });

      if (navState && Array.isArray(navState.spots)) {
        tabData.navState = navState;
        tabData.lastSavedAt = new Date();

        console.log(`Successfully saved navigation state for tab ${tabId}:`, {
          spotsCount: navState.spots.length,
          selectedSpotIndex: navState.selectedSpotIndex,
          selectedChanceIndex: navState.selectedChanceIndex,
          exportedAt: navState.exportedAt,
          hasRates: !!navState.rates
        });
      } else {
        console.warn(`Invalid navigation state exported for tab ${tabId}:`, {
          navState,
          hasSpots: !!(navState?.spots),
          spotsIsArray: Array.isArray(navState?.spots),
          spotsCount: navState?.spots?.length
        });
      }
    } else {
      console.warn(`Cannot save navigation state for tab ${tabId}:`, {
        hasNavRef: !!navRef,
        hasExportMethod: !!(navRef?.exportCurrentState),
        hasTabData: !!tabData
      });
    }
  } catch (error) {
    console.error(`Error saving navigation state for tab ${currentActiveTabId.value}:`, error);
  }
};

// 恢复标签页的导航状态
const restoreTabNavState = async (tabId: string) => {
  try {
    const tabData = tabsData.value.get(tabId);
    if (!tabData) {
      console.warn(`No tab data found for ${tabId}, cannot restore state`);
      return;
    }
    
    console.log(`Restoring navigation state for tab ${tabId}:`, {
      hasNavState: !!tabData.navState,
      selectedSpot: tabData.selectedSpot?.type,
      selectedChance: tabData.selectedChance?.player,
      displayMode: tabData.displayMode,
      results: !!tabData.results,
      lastSavedAt: tabData.lastSavedAt
    });
    
    // 多次等待确保DOM完全更新
    await nextTick();
    await nextTick();
    
    // 使用 setTimeout 确保在下一个事件循环中执行
    await new Promise(resolve => setTimeout(resolve, 10));
    
    const navRef = getNavRef(tabId);
    console.log(`Navigation ref for tab ${tabId}:`, {
      hasRef: !!navRef,
      hasImportState: !!(navRef?.importState),
      hasNavState: !!tabData.navState
    });
    
    if (navRef && tabData.navState && typeof navRef.importState === 'function') {
      // 在恢复导航状态之前，先应用游戏历史（如果有的话）
      if (tabData.navState.configSnapshot) {
        try {
          // 确保游戏配置是正确的
          const useConfigStore = useSavedConfigStore();
          const currentConfig = {
            board: useConfigStore.board,
            startingPot: useConfigStore.startingPot,
            effectiveStack: useConfigStore.effectiveStack
          };
          
          // 检查配置是否匹配
          const savedConfig = tabData.navState.configSnapshot;
          const configMatches = 
            JSON.stringify(currentConfig.board) === JSON.stringify(savedConfig.board) &&
            currentConfig.startingPot === savedConfig.startingPot &&
            currentConfig.effectiveStack === savedConfig.effectiveStack;
            
          if (!configMatches) {
            console.warn(`Config mismatch for tab ${tabId}, will reinitialize`);
            tabData.isHandlerUpdated = true;
            return;
          }
        } catch (configError) {
          console.error(`Config check failed for tab ${tabId}:`, configError);
        }
      }
      
      // 恢复导航状态
      navRef.importState(tabData.navState);
      console.log(`Successfully restored navigation state for tab ${tabId}`);
      
      // 恢复后触发保存，刷新 lastSavedAt 时间戳
      await nextTick();
      scheduleAutoSave(tabId);
    } else {
      console.log(`Cannot restore navigation state for tab ${tabId}, triggering reinitialize`);
      // 如果没有保存的导航状态或引用无效，触发重新初始化
      tabData.isHandlerUpdated = true;
    }
    
  } catch (error) {
    console.error(`Error restoring navigation state for tab ${tabId}:`, error);
    // 如果恢复失败，重新初始化
    const tabData = tabsData.value.get(tabId);
    if (tabData) {
      tabData.isHandlerUpdated = true;
    }
  }
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

// 设置导航组件引用
const setNavRef = (tabId: string, ref: any) => {
  if (ref) {
    // 只在ref真正变化时才设置，避免无限循环
    const existingRef = navRefs.value.get(tabId);
    // 如果引用对象未变化，则直接返回，避免重复设置和日志
    if (existingRef === ref) {
      return;
    }

    if (!existingRef) {
      // 只在首次设置时记录和更新（不在此更新活跃ID，避免循环）
      navRefs.value.set(tabId, ref);
      console.log(`Set navigation ref for tab ${tabId} (first time)`);
      addDebugLog('info', `Set navigation reference: ${tabId}`);
    } else {
      // 已存在，静默更新ref，不触发任何响应式操作
      navRefs.value.set(tabId, ref);
    }
    
    // 在组件真正"可导出"的节点，尝试自动保存活跃标签页的导航状态
    nextTick(() => {
      setTimeout(() => {
        ensureActiveTabNavStateSaved(tabId);
      }, 50); // 短暂延迟确保组件完全初始化
    });
  } else {
    // 清除引用
    navRefs.value.delete(tabId);
    console.log(`Cleared navigation ref for tab ${tabId}`);
    addDebugLog('info', `Cleared navigation reference: ${tabId}`);
  }
};

// 获取导航组件引用
const getNavRef = (tabId: string) => {
  return navRefs.value.get(tabId);
};

// 防止并发切换的锁
const switchingLock = ref(false);

// 自动保存锁，防止重复触发
const autoSaveLock = ref(new Set<string>());

// 去抖保存机制
const autoSaveTimers = ref(new Map<string, NodeJS.Timeout>());

// 页面关闭前兜底保存
const saveAllTabsBeforeUnload = async () => {
  console.log('🔄 Saving all tabs before page unload...');
  const savePromises: Promise<boolean>[] = [];
  
  for (const [tabId] of tabsData.value) {
    const navRef = navRefs.value.get(tabId);
    if (navRef && typeof navRef.exportCurrentState === 'function') {
      savePromises.push(saveNavStateByTabId(tabId));
    }
  }
  
  try {
    const results = await Promise.all(savePromises);
    const savedCount = results.filter(success => success).length;
    console.log(`✅ Saved ${savedCount}/${results.length} tabs before unload`);
  } catch (error) {
    console.error('❌ Error saving tabs before unload:', error);
  }
};

// 去抖自动保存函数
const scheduleAutoSave = (tabId?: string) => {
  const targetTabId = tabId || currentActiveTabId.value;
  if (!targetTabId) return;

  // 清除之前的定时器
  const existingTimer = autoSaveTimers.value.get(targetTabId);
  if (existingTimer) {
    clearTimeout(existingTimer);
  }

  // 设置新的定时器（300ms 去抖）
  const timer = setTimeout(async () => {
    console.log(`🔄 Auto-saving navigation state for tab: ${targetTabId}`);
    addDebugLog('info', `Auto-saving navigation state: ${targetTabId}`);
    
    try {
      const success = await saveNavStateByTabId(targetTabId);
      if (success) {
        console.log(`✅ Auto-saved navigation state for tab: ${targetTabId}`);
        addDebugLog('success', `Auto-save successful: ${targetTabId}`);
      } else {
        console.warn(`⚠️ Auto-save failed for tab: ${targetTabId}`);
        addDebugLog('warning', `Auto-save failed: ${targetTabId}`);
      }
    } catch (error) {
      console.error(`❌ Auto-save error for tab ${targetTabId}:`, error);
      addDebugLog('error', `Auto-save error: ${targetTabId} - ${error}`);
    } finally {
      // 清理定时器
      autoSaveTimers.value.delete(targetTabId);
    }
  }, 300);

  autoSaveTimers.value.set(targetTabId, timer);
  console.log(`⏰ Scheduled auto-save for tab ${targetTabId} in 300ms`);
};

// 处理导航状态变化事件
const onNavStateChanged = (navState: any) => {
  const activeTabId = currentActiveTabId.value;
  if (!activeTabId) return;

  console.log(`🔔 Navigation state changed for tab ${activeTabId}:`, {
    spotsCount: navState?.spots?.length || 0,
    selectedSpotIndex: navState?.selectedSpotIndex,
    selectedChanceIndex: navState?.selectedChanceIndex,
  });

  // 触发去抖保存
  scheduleAutoSave(activeTabId);
};

/* 生命周期管理 */

// 页面加载时设置 beforeunload 监听器
if (typeof window !== 'undefined') {
  const handleBeforeUnload = (event: BeforeUnloadEvent) => {
    // 同步保存所有标签页（浏览器限制异步操作）
    try {
      for (const [tabId] of tabsData.value) {
        const navRef = navRefs.value.get(tabId);
        const tabData = tabsData.value.get(tabId);
        if (navRef && typeof navRef.exportCurrentState === 'function' && tabData) {
          const navState = navRef.exportCurrentState();
          if (navState && Array.isArray(navState.spots)) {
            tabData.navState = navState;
            tabData.lastSavedAt = new Date();
          }
        }
      }
      console.log('✅ Synchronously saved all tabs before unload');
    } catch (error) {
      console.error('❌ Error in beforeunload save:', error);
    }
  };

  window.addEventListener('beforeunload', handleBeforeUnload);

  // 清理函数（在组件卸载时移除监听器）
  onUnmounted(() => {
    window.removeEventListener('beforeunload', handleBeforeUnload);
  });
}

// 幂等的工具函数：确保活跃标签页的导航状态已保存
const ensureActiveTabNavStateSaved = async (tabId: string) => {
  // 防重复触发
  if (autoSaveLock.value.has(tabId)) {
    return;
  }

  // 仅当该 tabId 是当前活跃标签页时处理
  if (tabId !== currentActiveTabId.value) {
    return;
  }

  const tabData = tabsData.value.get(tabId);
  const navRef = navRefs.value.get(tabId);

  // 仅当存在 navRef.exportCurrentState 且该 tab 还没有 navState 时，自动保存
  if (navRef && 
      typeof navRef.exportCurrentState === 'function' && 
      tabData && 
      !tabData.navState) {
    
    console.log(`🔄 Auto-ensuring nav state saved for active tab: ${tabId}`);
    autoSaveLock.value.add(tabId);
    
    try {
      const navState = navRef.exportCurrentState();
      if (navState && Array.isArray(navState.spots)) {
        tabData.navState = navState;
        tabData.lastSavedAt = new Date();
        console.log(`✅ Auto-saved navigation state for tab ${tabId}:`, {
          spotsCount: navState.spots.length,
          selectedSpotIndex: navState.selectedSpotIndex,
          selectedChanceIndex: navState.selectedChanceIndex,
        });
        addDebugLog('success', `Auto-saved navigation state: ${tabId}`);
      }
    } catch (error) {
      console.error(`❌ Auto-save failed for tab ${tabId}:`, error);
    } finally {
      // 短暂延迟后移除锁，允许后续保存
      setTimeout(() => {
        autoSaveLock.value.delete(tabId);
      }, 1000);
    }
  }
};

// 标签页事件处理
const onTabSwitched = async (tabId: string, tab: TabData) => {
  // 先更新活跃 ID，避免因锁而错位
  const previousTabId = currentActiveTabId.value;
  currentActiveTabId.value = tabId;
  console.log(`Tab switched: ${previousTabId} → ${tabId}`);
  addDebugLog('info', `Tab switched: "${previousTabId}" → "${tabId}"`);
  
  // 防止并发切换
  if (switchingLock.value) {
    const msg = `标签页切换正在进行中，但已更新活跃ID到 ${tabId}`;
    console.warn(msg);
    addDebugLog('warning', msg);
    return;
  }
  
  switchingLock.value = true;
  
  try {
    // 步骤1: 保存前一个标签页的状态（如果存在且不同）
    if (previousTabId && previousTabId !== tabId) {
      addDebugLog('info', `Saving previous tab state: ${previousTabId}`);
      console.log(`Saving state for previous tab: ${previousTabId}`);
      await saveCurrentTabNavState();
      
      // 额外等待确保保存完成
      await nextTick();
    }
    
    // 步骤2: 初始化或恢复目标标签页
    if (!tabsData.value.has(tabId)) {
      addDebugLog('info', `Initializing new tab: ${tabId}`);
      console.log(`Initializing new tab: ${tabId}`);
      await initTabData(tabId);
    } else {
      addDebugLog('info', `Restoring existing tab: ${tabId}`);
      console.log(`Restoring existing tab: ${tabId}`);
      await restoreTabNavState(tabId);
    }
    
    addDebugLog('success', `Tab switch successful: ${tabId}`);
    console.log(`Successfully switched to tab: ${tabId}`);
    
    // 在切换完成后，确保当前活跃标签页的导航状态已保存
    await nextTick();
    setTimeout(() => {
      ensureActiveTabNavStateSaved(tabId);
    }, 100); // 稍长的延迟确保恢复操作完成
    
  } catch (error) {
    const errorMsg = `标签页切换失败 ${tabId}: ${error}`;
    console.error(`Error during tab switch to ${tabId}:`, error);
    addDebugLog('error', errorMsg);
    
    // 错误恢复：如果切换失败，至少确保目标标签页有基本数据
    if (!tabsData.value.has(tabId)) {
      try {
        await initTabData(tabId);
        addDebugLog('info', `Error recovery: Initialized tab ${tabId}`);
      } catch (initError) {
        const initErrorMsg = `错误恢复失败 ${tabId}: ${initError}`;
        console.error(`Failed to initialize tab ${tabId} during error recovery:`, initError);
        addDebugLog('error', initErrorMsg);
      }
    }
  } finally {
    switchingLock.value = false;
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

const onTabRenamed = (tabId: string, newName: string) => {
  console.log(`Tab renamed: ${tabId} -> "${newName}"`);
  addDebugLog('info', `Tab renamed: ${tabId} -> "${newName}"`);

  // 更新 tabsData 中的名称
  const tabData = tabsData.value.get(tabId);
  if (tabData) {
    tabData.name = newName;
  }

  // 刷新调试信息
  refreshDebugInfo();
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
  if (!tabData) {
    console.warn(`Tab data not found for tab ${tabId}`);
    return;
  }

  // 更新标签页数据
  tabData.dealtCard = -1;
  tabData.selectedSpot = newSelectedSpot;
  tabData.selectedChance = newSelectedChance;
  tabData.currentBoard = newCurrentBoard;
  tabData.results = newResults;
  tabData.chanceReports = newChanceReports;
  tabData.totalBetAmount = newTotalBetAmount;
  tabData.isLocked = false;
  tabData.chanceMode = newSelectedChance?.player ?? "";
  
  // 保存导航历史（从 spots 数组中提取选择的索引）
  if (newSelectedSpot) {
    try {
      // 这里我们需要从 ResultNav2 组件获取当前的 spots 状态
      // 暂时先记录基本信息
      tabData.navigationHistory = {
        selectedSpotIndex: newSelectedSpot.index,
        selectedChanceIndex: newSelectedChance?.index || -1,
        lastUpdated: new Date()
      };
    } catch (error) {
      console.error(`Error saving navigation history for tab ${tabId}:`, error);
    }
  }
  
  console.log(`Updated spot for tab ${tabId}:`, {
    selectedSpot: newSelectedSpot?.type,
    selectedChance: newSelectedChance?.player,
    currentBoard: newCurrentBoard.length,
    hasResults: !!newResults,
    navigationHistory: tabData.navigationHistory
  });

  // 触发去抖自动保存（兜底机制）
  scheduleAutoSave(tabId);
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
const showTabDebugModal = ref(false);
const debugCopyStatus = ref(0);
const debugConsoleStatus = ref(0);
const pathConsoleStatus = ref(0);

// 调试日志系统
const debugLogs = ref<Array<{
  timestamp: Date;
  type: 'info' | 'warning' | 'error' | 'success';
  message: string;
}>>([]);

// 添加调试日志
const addDebugLog = (type: 'info' | 'warning' | 'error' | 'success', message: string) => {
  debugLogs.value.push({
    timestamp: new Date(),
    type,
    message
  });
  
  // 保持最多100条日志
  if (debugLogs.value.length > 100) {
    debugLogs.value.shift();
  }
};

// 格式化时间
const formatTime = (date: Date | string | undefined) => {
  if (!date) return '未设置';
  const d = typeof date === 'string' ? new Date(date) : date;
  return d.toLocaleTimeString('zh-CN', { 
    hour12: false, 
    hour: '2-digit', 
    minute: '2-digit', 
    second: '2-digit' 
  });
};

// 获取标签页导航状态的显示信息
const getTabNavStateStatus = (tabId: string) => {
  const tabData = tabsData.value.get(tabId);
  const navRef = navRefs.value.get(tabId);
  const navState = tabData?.navState;
  
  // 如果有导航状态且spots是数组（无论是否为空），说明已保存
  if (navState && Array.isArray(navState.spots)) {
    const spotsCount = navState.spots.length;
    return {
      class: 'status-good',
      text: spotsCount > 0 ? `✅ 已保存 (${spotsCount} spots)` : '✅ 已保存 (初始状态)'
    };
  }
  
  // 如果有导航引用但没有保存状态，说明可以保存但尚未保存
  if (navRef && typeof navRef.exportCurrentState === 'function') {
    return {
      class: 'status-warning',
      text: '⚠️ Savable but not saved'
    };
  }
  
  // 如果没有导航引用，说明组件还未准备好
  if (!navRef) {
    return {
      class: 'status-bad',
      text: '❌ Navigation component not ready'
    };
  }
  
  // 默认情况
  return {
    class: 'status-warning',
    text: '⚠️ Status unknown'
  };
};

// 调试功能方法
const refreshDebugInfo = () => {
  addDebugLog('info', 'Manually refreshing debug info');
  // 强制重新渲染调试界面
  showTabDebugModal.value = false;
  nextTick(() => {
    showTabDebugModal.value = true;
  });
};

const testSaveAllTabs = async () => {
  addDebugLog('info', 'Starting test to save all tab states');
  
  try {
    // 保存当前活跃标签页
    if (currentActiveTabId.value) {
      await saveCurrentTabNavState();
      addDebugLog('success', `Saved current tab: ${currentActiveTabId.value}`);
    }
    
    // 尝试获取所有标签页的引用状态
    let successCount = 0;
    let failCount = 0;
    
    for (const tab of tabManager.value?.tabs || []) {
      const navRef = getNavRef(tab.id);
      if (navRef && typeof navRef.exportCurrentState === 'function') {
        try {
          const state = navRef.exportCurrentState();
          if (state && state.spots) {
            successCount++;
            addDebugLog('success', `Tab ${tab.id} state export successful (${state.spots.length} spots)`);
          } else {
            failCount++;
            addDebugLog('warning', `Tab ${tab.id} state export is empty`);
          }
        } catch (error) {
          failCount++;
          addDebugLog('error', `Tab ${tab.id} state export failed: ${error}`);
        }
      } else {
        failCount++;
        addDebugLog('error', `Tab ${tab.id} missing navigation reference or export method`);
      }
    }
    
    addDebugLog('info', `Test completed: ${successCount} successful, ${failCount} failed`);
  } catch (error) {
    addDebugLog('error', `Test save failed: ${error}`);
  }
};

const testRestoreCurrentTab = async () => {
  if (!currentActiveTabId.value) {
    addDebugLog('warning', 'No current active tab available for restore test');
    return;
  }
  
  addDebugLog('info', `Starting tab restore test: ${currentActiveTabId.value}`);
  
  try {
    await restoreTabNavState(currentActiveTabId.value);
    addDebugLog('success', `Tab ${currentActiveTabId.value} restore test completed`);
  } catch (error) {
    addDebugLog('error', `Tab ${currentActiveTabId.value} restore test failed: ${error}`);
  }
};

const checkRefsStatus = () => {
  const currentTabId = currentActiveTabId.value;
  addDebugLog('info', `Starting reference status check: Active tab=${currentTabId}`);

  console.log('🔍 完整的引用状态检查:');
  console.log('='.repeat(50));

  // 检查所有标签页
  const allTabs = tabManager.value?.tabs || [];
  console.log(`📊 总标签页数量: ${allTabs.length}`);
  console.log(`📊 navRefs数量: ${(navRefs.value as Map<string, any>).size}`);
  console.log(`📊 tabsData数量: ${(tabsData.value as Map<string, any>).size}`);
  console.log(`📊 当前活跃标签页: ${currentTabId || '无'}`);
  console.log('');

  for (const tab of allTabs) {
    const tabId = tab.id;
    const navRef = navRefs.value.get(tabId);
    const tabData = tabsData.value.get(tabId);

    console.log(`📋 标签页 ${tabId} (${tab.name}):`);
    console.log(`  - 数据存在: ${!!tabData ? '✅' : '❌'}`);
    console.log(`  - 导航引用: ${!!navRef ? '✅' : '❌'}`);

    if (navRef) {
      console.log(`  - 导出方法: ${!!navRef.exportCurrentState ? '✅' : '❌'}`);
      console.log(`  - 导入方法: ${!!navRef.importState ? '✅' : '❌'}`);
      console.log(`  - 引用类型: ${typeof navRef}`);
      console.log(`  - 方法列表:`, Object.getOwnPropertyNames(navRef));
    }

    if (tabData) {
      console.log(`  - navState: ${!!tabData.navState ? '✅' : '❌'}`);
      if (tabData.navState) {
        console.log(`  - spots数组: ${Array.isArray(tabData.navState.spots) ? '✅' : '❌'}`);
        console.log(`  - spots数量: ${tabData.navState.spots?.length || 0}`);
        console.log(`  - 选中Spot: ${tabData.navState.selectedSpotIndex ?? '无'}`);
        console.log(`  - 选中Chance: ${tabData.navState.selectedChanceIndex ?? '无'}`);
      }
    }

    console.log(`  - 状态判断:`, getTabNavStateStatus(tabId));
    console.log('');
  }

  console.log('='.repeat(50));
  addDebugLog('success', 'Reference status check completed, please check console output');
};



// 按指定 ID 保存导航状态
const saveNavStateByTabId = async (targetTabId: string) => {
  const navRef = getNavRef(targetTabId);
  const tabData = tabsData.value.get(targetTabId);

  console.log(`Attempting to save navigation state for tab ${targetTabId}:`, {
    hasNavRef: !!navRef,
    hasExportMethod: !!(navRef?.exportCurrentState),
    hasTabData: !!tabData
  });

  if (navRef && typeof navRef.exportCurrentState === 'function' && tabData) {
    const navState = navRef.exportCurrentState();

    console.log(`Navigation state exported for tab ${targetTabId}:`, {
      hasNavState: !!navState,
      hasSpots: !!(navState?.spots),
      spotsCount: navState?.spots?.length || 0,
      selectedSpotIndex: navState?.selectedSpotIndex,
      selectedChanceIndex: navState?.selectedChanceIndex,
      exportedAt: navState?.exportedAt,
      hasRates: !!(navState?.rates)
    });

    if (navState && Array.isArray(navState.spots)) {
      tabData.navState = navState;
      tabData.lastSavedAt = new Date();

      console.log(`Successfully saved navigation state for tab ${targetTabId}:`, {
        spotsCount: navState.spots.length,
        selectedSpotIndex: navState.selectedSpotIndex,
        selectedChanceIndex: navState.selectedChanceIndex,
        exportedAt: navState.exportedAt,
        hasRates: !!navState.rates
      });
      return true;
    } else {
      console.warn(`Invalid navigation state exported for tab ${targetTabId}:`, {
        navState,
        hasSpots: !!(navState?.spots),
        spotsIsArray: Array.isArray(navState?.spots),
        spotsCount: navState?.spots?.length
      });
      return false;
    }
  } else {
    console.warn(`Cannot save navigation state for tab ${targetTabId}:`, {
      hasNavRef: !!navRef,
      hasExportMethod: !!(navRef?.exportCurrentState),
      hasTabData: !!tabData
    });
    return false;
  }
};

const testSaveCurrentTabNav = async () => {
  // 优先从 TabManager 获取当前活跃标签页，避免依赖 currentActiveTabId
  const activeTabFromManager = tabManager.value?.activeTab?.id;
  const targetTabId = activeTabFromManager || currentActiveTabId.value;

  if (!targetTabId) {
    addDebugLog('warning', 'No current active tab available for save test');
    return;
  }

  console.log(`手动保存调试信息:`, {
    activeTabFromManager,
    currentActiveTabId: currentActiveTabId.value,
    targetTabId,
    navRefsKeys: Array.from((navRefs.value as Map<string, any>).keys()),
    tabManagerActiveTab: tabManager.value?.activeTab?.id
  });

  addDebugLog('info', `Starting test to save current tab navigation state: ${targetTabId}`);

  // 添加延迟确保组件完全加载
  await nextTick();
  await new Promise(resolve => setTimeout(resolve, 50));

  // 检查导航引用是否存在
  const navRef = getNavRef(targetTabId);
  if (!navRef) {
    addDebugLog('error', `Navigation reference does not exist, cannot save state`);
    console.error(`Navigation ref not found for tab ${targetTabId}`);
    return;
  }

  // 添加额外的状态检查
  if (typeof navRef.exportCurrentState !== 'function') {
    addDebugLog('error', `Navigation component export method unavailable`);
    console.error(`Export method not available for tab ${targetTabId}`);
    return;
  }

  const tabData = tabsData.value.get(targetTabId);
  if (!tabData) {
    addDebugLog('error', `Tab data does not exist, cannot save state`);
    console.error(`Tab data not found for tab ${targetTabId}`);
    return;
  }

  try {
    const success = await saveNavStateByTabId(targetTabId);
    if (success) {
      addDebugLog('success', `Manual save navigation state successful: ${targetTabId}`);
    } else {
      addDebugLog('error', `Manual save navigation state failed: ${targetTabId}`);
    }

    // 强制刷新状态显示
    refreshDebugInfo();
  } catch (error) {
    addDebugLog('error', `Tab ${targetTabId} navigation state save test failed: ${error}`);
  }
};

const exportTabDebugData = async () => {
  try {
    const tabsInfo = [];
    // 收集每个标签页的详细信息
    for (const tab of tabManager.value?.tabs || []) {
      const tabData = tabsData.value.get(tab.id);
      const navRef = navRefs.value.get(tab.id);

      tabsInfo.push({
        id: tab.id,
        name: tab.name,
        hasData: !!tabData,
        hasRef: !!navRef,
        hasNavState: !!(tabData?.navState),
        spotsCount: tabData?.navState?.spots?.length || 0,
        selectedSpotIndex: tabData?.navState?.selectedSpotIndex,
        selectedChanceIndex: tabData?.navState?.selectedChanceIndex,
        lastSavedAt: tabData?.lastSavedAt,
        displayMode: tabData?.displayMode,
        hasResults: !!tabData?.results,
      });
    }

    const debugData = {
      timestamp: new Date().toISOString(),
      currentActiveTabId: currentActiveTabId.value,
      switchingLock: switchingLock.value,
      tabsCount: tabManager.value?.tabs?.length || 0,
      dataCount: (tabsData.value as Map<string, any>).size,
      refsCount: (navRefs.value as Map<string, any>).size,
      tabs: tabsInfo,
      logs: debugLogs.value,
    };

    const content = JSON.stringify(debugData, null, 2);
    await navigator.clipboard.writeText(content);
    addDebugLog('success', 'Debug data copied to clipboard');
  } catch (error) {
    addDebugLog('error', `Export debug data failed: ${error}`);
  }
};

const clearAllTabStates = () => {
  if (confirm('确定要清除所有标签页状态吗？这将重置所有标签页到初始状态。')) {
    tabsData.value.clear();
    navRefs.value.clear();
    currentActiveTabId.value = '';
    addDebugLog('warning', 'All tab states have been cleared');
  }
};

// Collect all relevant data in a single JSON object
const debugData = computed(() => {
  const currentTabId = currentActiveTabId.value;
  const currentTabData = currentTabId ? tabsData.value.get(currentTabId) : null;

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
      enhancedMode: enhancedMode.value,
      showVersionInfo: showVersionInfo.value,
      currentActiveTabId: currentActiveTabId.value,
      tabsDataCount: tabsData.value.size,
      navRefsCount: navRefs.value.size,
      switchingLock: switchingLock.value,
    },
    // Current tab data
    currentTabData: currentTabData ? {
      id: currentTabData.name || currentTabId,
      displayMode: currentTabData.displayMode,
      hasResults: !!currentTabData.results,
      hasNavState: !!currentTabData.navState,
      spotsCount: currentTabData.navState?.spots?.length || 0,
      selectedSpotIndex: currentTabData.navState?.selectedSpotIndex,
      selectedChanceIndex: currentTabData.navState?.selectedChanceIndex,
      lastSavedAt: currentTabData.lastSavedAt,
    } : null,
    // All tabs data
    allTabsData: Array.from((tabsData.value as Map<string, any>).entries()).map(([tabId, data]) => ({
      id: tabId,
      name: data.name,
      hasNavState: !!data.navState,
      spotsCount: data.navState?.spots?.length || 0,
      hasResults: !!data.results,
      displayMode: data.displayMode,
    })),
    // Debug metadata
    meta: {
      timestamp: new Date().toISOString(),
      version: "2.0.0",
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

.debug-button, .tab-debug-button, .path-button {
  background-color: #ff5500;
  color: white;
  border: none;
  border-radius: 4px;
  padding: 8px 16px;
  font-weight: bold;
  cursor: pointer;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
  min-width: 120px;
  width: 120px;
  text-align: center;
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

/* Tab Debugger Styles */
.tab-debug-button {
  background-color: #10b981;
}

.tab-debug-button:hover {
  background-color: #059669;
}

.tab-debug-modal {
  background-color: white;
  border-radius: 8px;
  width: 95%;
  max-width: 1400px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
}

.debug-actions {
  margin-bottom: 20px;
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.debug-actions button {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
  transition: all 0.2s;
}

.refresh-button { background-color: #3b82f6; color: white; }
.refresh-button:hover { background-color: #2563eb; }

.test-button { background-color: #10b981; color: white; }
.test-button:hover { background-color: #059669; }

.export-button { background-color: #f59e0b; color: white; }
.export-button:hover { background-color: #d97706; }

.clear-button { background-color: #ef4444; color: white; }
.clear-button:hover { background-color: #dc2626; }

.debug-sections {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
  max-height: 70vh;
  overflow-y: auto;
}

.debug-section {
  background: #f8fafc;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  padding: 16px;
}

.debug-section h4 {
  margin: 0 0 12px 0;
  color: #1e293b;
  font-size: 1.1rem;
  font-weight: 600;
}

.debug-info > div {
  margin: 8px 0;
  padding: 4px 0;
  border-bottom: 1px solid #e2e8f0;
}

.tabs-debug-list {
  max-height: 400px;
  overflow-y: auto;
}

.tab-debug-item {
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  padding: 12px;
  margin-bottom: 12px;
}

.tab-debug-item.active {
  border-color: #3b82f6;
  background: #eff6ff;
}

.tab-debug-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 8px;
}

.tab-name {
  font-weight: 600;
  color: #1e293b;
}

.tab-id {
  font-size: 0.8rem;
  color: #64748b;
  background: #f1f5f9;
  padding: 2px 6px;
  border-radius: 3px;
}

.active-indicator {
  font-size: 0.8rem;
  color: #059669;
  font-weight: 500;
}

.debug-row {
  display: flex;
  justify-content: space-between;
  padding: 4px 0;
  font-size: 0.9rem;
}

.debug-row span:first-child {
  color: #64748b;
}

.status-good { color: #059669; font-weight: 500; }
.status-warning { color: #d97706; font-weight: 500; }
.status-bad { color: #dc2626; font-weight: 500; }

.tab-state-details {
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid #e2e8f0;
}

.refs-debug-info {
  max-height: 300px;
  overflow-y: auto;
}

.ref-debug-item {
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 4px;
  padding: 8px;
  margin-bottom: 8px;
}

.debug-logs {
  max-height: 300px;
  overflow-y: auto;
  font-family: monospace;
  font-size: 0.8rem;
}

.debug-log-item {
  display: flex;
  gap: 8px;
  padding: 4px 8px;
  margin: 2px 0;
  border-radius: 3px;
  align-items: center;
}

.log-info { background: #eff6ff; }
.log-success { background: #f0fdf4; }
.log-warning { background: #fefce8; }
.log-error { background: #fef2f2; }

.log-time {
  color: #64748b;
  min-width: 60px;
}

.log-type {
  font-weight: 600;
  min-width: 60px;
}

.log-info .log-type { color: #2563eb; }
.log-success .log-type { color: #059669; }
.log-warning .log-type { color: #d97706; }
.log-error .log-type { color: #dc2626; }

.log-message {
  flex: 1;
}
</style>
