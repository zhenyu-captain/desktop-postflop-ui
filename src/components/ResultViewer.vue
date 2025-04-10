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
    <ResultNav
      :is-handler-updated="isHandlerUpdated"
      :is-locked="isLocked"
      :cards="cards"
      :dealt-card="dealtCard"
      @update:is-handler-updated="(value) => (isHandlerUpdated = value)"
      @update:is-locked="(value) => (isLocked = value)"
      @trigger-update="onUpdateSpot"
    />

    <ResultMiddle
      :display-mode="displayMode"
      :chance-mode="chanceMode"
      :auto-player-basics="autoPlayerBasics"
      :auto-player-chance="autoPlayerChance"
      :copy-success="copySuccess"
      @update:display-mode="updateDisplayMode"
      @update:display-options="updateDisplayOptions"
      @copy-to-clipboard="copyRangeTextToClipboard"
      @reset-copy-success="resetCopySuccess"
    />

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

    <div
      v-if="store.navView === 'results' && selectedSpot && results"
      class="flex flex-grow min-h-0"
    >
      <template v-if="displayMode === 'basics'">
        <ResultBasics
          style="flex: 4"
          :cards="cards"
          :selected-spot="selectedSpot"
          :selected-chance="selectedChance"
          :current-board="currentBoard"
          :total-bet-amount="totalBetAmount"
          :results="results"
          :display-options="displayOptions"
          :display-player="displayPlayerBasics"
          :is-compare-mode="false"
          @update-hover-content="onUpdateHoverContent"
        />

        <ResultTable
          style="flex: 3"
          table-mode="basics"
          :cards="cards"
          :selected-spot="selectedSpot"
          :results="results"
          :display-player="displayPlayerBasics"
          :hover-content="basicsHoverContent"
          :table-bar-width="displayOptions.barWidth"
          :table-display-content="displayOptions.actionContent"
        />
      </template>

      <template v-else-if="displayMode === 'graphs'">
        <ResultGraphs
          :cards="cards"
          :selected-spot="selectedSpot"
          :selected-chance="selectedChance"
          :results="results"
          :chance-reports="chanceReports"
          :display-options="displayOptions"
          :display-player="displayPlayerBasics"
        />
      </template>

      <template v-else-if="displayMode === 'compare'">
        <ResultBasics
          style="flex: 5"
          :cards="cards"
          :selected-spot="selectedSpot"
          :selected-chance="selectedChance"
          :current-board="currentBoard"
          :total-bet-amount="totalBetAmount"
          :results="results"
          :display-options="displayOptions"
          display-player="oop"
          :is-compare-mode="true"
        />

        <ResultCompare
          style="flex: 2"
          :selected-spot="selectedSpot"
          :selected-chance="selectedChance"
          :results="results"
        />

        <ResultBasics
          style="flex: 5"
          :cards="cards"
          :selected-spot="selectedSpot"
          :selected-chance="selectedChance"
          :current-board="currentBoard"
          :total-bet-amount="totalBetAmount"
          :results="results"
          :display-options="displayOptions"
          display-player="ip"
          :is-compare-mode="true"
        />
      </template>

      <template v-else-if="displayMode === 'chance' && selectedChance">
        <ResultChance
          :selected-spot="selectedSpot"
          :selected-chance="selectedChance"
          :chance-reports="chanceReports"
          :display-options="displayOptions"
          :display-player="displayPlayerChance"
          @deal-card="onDealCard"
        />
      </template>
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
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useStore } from "../store";
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

import ResultNav from "./ResultNav.vue";
import ResultMiddle from "./ResultMiddle.vue";
import ResultBasics from "./ResultBasics.vue";
import ResultTable from "./ResultTable.vue";
import ResultCompare from "./ResultCompare.vue";
import ResultGraphs from "./ResultGraphs.vue";
import ResultChance from "./ResultChance.vue";

const store = useStore();

/* Navigation */

const isHandlerUpdated = ref(false);
const isLocked = ref(false);

const cards = ref<number[][]>([[], []]);
const dealtCard = ref(-1);

const selectedSpot = ref<Spot | null>(null);
const selectedChance = ref<SpotChance | null>(null);
const currentBoard = ref<number[]>([]);
const results = ref<Results | null>(null);
const chanceReports = ref<ChanceReports | null>(null);
const totalBetAmount = ref([0, 0]);

const isSolverFinished = ref(false);
store.$subscribe(async (_, store) => {
  if (isSolverFinished.value !== store.isSolverFinished) {
    if ((isSolverFinished.value = store.isSolverFinished)) {
      await init();
    } else {
      clear();
    }
  }
});

const init = async () => {
  cards.value = await invokes.gamePrivateCards();
  isHandlerUpdated.value = true;
};

const clear = () => {
  cards.value = [[], []];
  selectedSpot.value = null;
  selectedChance.value = null;
  results.value = null;
  chanceReports.value = null;
};

const onUpdateSpot = (
  newSelectedSpot: Spot | null,
  newSelectedChance: SpotChance | null,
  newCurrentBoard: number[],
  newResults: Results,
  newChanceReports: ChanceReports | null,
  newTotalBetAmount: number[]
) => {
  dealtCard.value = -1;
  selectedSpot.value = newSelectedSpot;
  selectedChance.value = newSelectedChance;
  currentBoard.value = newCurrentBoard;
  results.value = newResults;
  chanceReports.value = newChanceReports;
  totalBetAmount.value = newTotalBetAmount;
  isLocked.value = false;

  chanceMode.value = newSelectedChance?.player ?? "";
};

/* Middle Bar */

const displayMode = ref<DisplayMode>("basics");
const chanceMode = ref("");

const displayOptions = ref<DisplayOptions>({
  playerBasics: "auto",
  playerChance: "auto",
  barHeight: "normalized",
  suit: "grouped",
  strategy: "show",
  contentBasics: "default",
  contentGraphs: "eq",
  chartChance: "strategy-combos",
});

const copySuccess = ref(0);

const updateDisplayMode = (mode: DisplayMode) => {
  displayMode.value = mode;
};

const updateDisplayOptions = (options: DisplayOptions) => {
  displayOptions.value = options;
};

const copyRangeTextToClipboard = async () => {
  const text = "Hello World";
  navigator.clipboard
    .writeText(text)
    .then(() => (copySuccess.value = 1))
    .catch(() => (copySuccess.value = -1));
};

const resetCopySuccess = () => {
  copySuccess.value = 0;
};

/* Computed */

const autoPlayerBasics = computed(() => {
  const spot = selectedSpot.value;
  const chance = selectedChance.value;
  if (!spot) return "oop";

  if (chance) {
    return chance.prevPlayer;
  } else if (spot.type === "terminal") {
    return spot.prevPlayer;
  } else {
    return (spot as SpotPlayer).player;
  }
});

const autoPlayerChance = computed(() => {
  const spot = selectedSpot.value;
  if (!spot) return "oop";
  if (spot.type === "terminal") {
    return spot.prevPlayer;
  } else {
    return (spot as SpotPlayer).player;
  }
});

const displayPlayerBasics = computed(() => {
  const optionPlayer = displayOptions.value.playerBasics;
  if (optionPlayer === "auto") {
    return autoPlayerBasics.value;
  } else {
    return optionPlayer;
  }
});

const displayPlayerChance = computed(() => {
  const optionPlayer = displayOptions.value.playerChance;
  if (optionPlayer === "auto") {
    return autoPlayerChance.value;
  } else {
    return optionPlayer;
  }
});

/* Results */

const basicsHoverContent = ref<HoverContent | null>(null);

const onUpdateHoverContent = (content: HoverContent | null) => {
  basicsHoverContent.value = content;
};

const onDealCard = (card: number) => {
  dealtCard.value = card;
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
