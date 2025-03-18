<template>
  <div class="flex my-1 items-center">
    Number of threads:
    <input
      v-model="numThreads"
      type="number"
      :class="
        'w-20 ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
        (numThreads < 1 || numThreads > 64 || numThreads % 1 !== 0
          ? 'input-error'
          : '')
      "
      min="1"
      max="64"
    />
    <button
      class="ml-3 button-base button-blue"
      :disabled="isTreeBuilding || store.isSolverRunning || store.isFinalizing"
      @click="buildTree"
    >
      Build New Tree
    </button>
    <button
      class="ml-3 button-base button-yellow"
      @click="toggleDebugInfo"
    >
      Debug
    </button>
  </div>

  <div class="my-1">Status: {{ treeStatus }}</div>
  
  <div v-if="showDebugInfo" class="my-3 p-3 bg-gray-100 rounded-lg text-xs overflow-auto max-h-80">
    <h3 class="font-bold mb-2">Debug Information:</h3>
    <div class="grid grid-cols-2 gap-2">
      <div>
        <h4 class="font-semibold">Store State:</h4>
        <pre>{{ JSON.stringify(storeDebugInfo, null, 2) }}</pre>
      </div>
      <div>
        <h4 class="font-semibold">Config State:</h4>
        <pre>{{ JSON.stringify(configDebugInfo, null, 2) }}</pre>
      </div>
    </div>
    <div class="mt-2">
      <h4 class="font-semibold">Board & Tree Config:</h4>
      <pre>{{ JSON.stringify(boardTreeDebugInfo, null, 2) }}</pre>
    </div>
    
    <div class="mt-4 border-t pt-2">
      <h4 class="font-semibold">Tree Building Debug:</h4>
      <div class="grid grid-cols-2 gap-2">
        <div>
          <p><span class="font-medium">Current Status:</span> {{ treeStatus }}</p>
          <p><span class="font-medium">isTreeBuilding:</span> {{ isTreeBuilding }}</p>
          <p><span class="font-medium">isTreeBuilt:</span> {{ isTreeBuilt }}</p>
          <p><span class="font-medium">Last Error:</span> {{ lastTreeBuildError || "None" }}</p>
        </div>
        <div>
          <p><span class="font-medium">Tree Build Started:</span> {{ treeBuildStartTime ? new Date(treeBuildStartTime).toLocaleTimeString() : "Not started" }}</p>
          <p><span class="font-medium">Elapsed Time:</span> {{ treeBuildStartTime ? ((Date.now() - treeBuildStartTime) / 1000).toFixed(2) + "s" : "N/A" }}</p>
          <p><span class="font-medium">gameInit Promise State:</span> {{ gameInitPromiseState }}</p>
        </div>
      </div>
      <div class="mt-2">
        <button class="button-base button-blue text-xs py-1" @click="trackTreeBuildingStatus">Check Tree Status</button>
        <button class="button-base button-green text-xs py-1 ml-2" @click="retryBuildTree">Retry Build Tree</button>
        <button class="button-base button-red text-xs py-1 ml-2" @click="forceResetTreeState">Force Reset Tree State</button>
      </div>
    </div>

    <div class="mt-2">
      <button class="button-base button-blue text-xs py-1" @click="refreshDebugInfo">刷新调试信息</button>
      <button class="button-base button-red text-xs py-1 ml-2" @click="logDebugInfo">控制台输出</button>
    </div>
  </div>

  <div v-if="isTreeBuilt" class="mt-3">
    <div>
      Precision mode:
      <Tippy
        class="inline-block cursor-help"
        max-width="500px"
        placement="bottom"
        trigger="mouseenter click"
        :delay="[200, 0]"
        :interactive="true"
      >
        <QuestionMarkCircleIcon class="w-5 h-5 text-gray-600" />
        <template #content>
          <div class="px-1 py-0.5 text-justify">
            Precision mode mainly affects memory usage, but it also has several
            other effects.
            <ul class="pl-6 list-disc">
              <li class="mt-1">
                Memory efficiency: 16-bit integer > 32-bit FP. The 16-bit
                integer mode uses only about half the memory of the 32-bit
                FP mode.
              </li>
              <li class="mt-1">
                Performance: Depends on the environment. The 32-bit FP mode
                requires fewer execution instructions than the 16-bit integer
                mode. However, memory reads and writes can become a bottleneck
                in highly multithreaded environments, and in such situations the
                16-bit integer mode may provide better performance.
              </li>
              <li class="mt-1">
                Significant digits: 32-bit FP (about 7 digits) > 16-bit integer
                (about 4 digits). The 16-bit integer mode is not suitable for
                satisfying an exploitability target of less than 0.1%.
              </li>
            </ul>
          </div>
        </template>
      </Tippy>
    </div>
    <div class="mt-1 ml-2">
      <label :class="{ 'cursor-pointer': !store.hasSolverRun }">
        <input
          v-model="isCompressionEnabled"
          class="mr-2 cursor-pointer disabled:cursor-default"
          type="radio"
          name="compression"
          :value="false"
          :disabled="store.hasSolverRun"
        />
        <span class="inline-block w-[6.75rem] ml-1">32-bit FP:</span>
        needs
        {{
          memoryUsageRaw >= 1023.5 * 1024 * 1024
            ? (memoryUsageRaw / (1024 * 1024 * 1024)).toFixed(2) + "GB"
            : (memoryUsageRaw / (1024 * 1024)).toFixed(0) + "MB"
        }}
        RAM
        {{ memoryUsageRaw > maxMemoryUsage ? "(out of memory)" : "" }}
      </label>
    </div>
    <div class="ml-2">
      <label :class="{ 'cursor-pointer': !store.hasSolverRun }">
        <input
          v-model="isCompressionEnabled"
          class="mr-2 cursor-pointer disabled:cursor-default"
          type="radio"
          name="compression"
          :value="true"
          :disabled="store.hasSolverRun"
        />
        <span class="inline-block w-[6.75rem] ml-1">16-bit integer:</span>
        needs
        {{
          memoryUsageRawCompressed >= 1023.5 * 1024 * 1024
            ? (memoryUsageRawCompressed / (1024 * 1024 * 1024)).toFixed(2) +
              "GB"
            : (memoryUsageRawCompressed / (1024 * 1024)).toFixed(0) + "MB"
        }}
        RAM
        {{ memoryUsageRawCompressed > maxMemoryUsage ? "(out of memory)" : "" }}
      </label>
    </div>
    <div
      v-if="store.isBunchingEnabled && store.bunchingFlop.length > 0"
      class="mt-1.5"
    >
      Additional RAM usage for bunching effect:
      {{
        memoryUsageBunching >= 1023.5 * 1024 * 1024
          ? (memoryUsageBunching / (1024 * 1024 * 1024)).toFixed(2) + "GB"
          : (memoryUsageBunching / (1024 * 1024)).toFixed(0) + "MB"
      }}
    </div>
    <div
      v-if="memoryUsage > maxMemoryUsage && osName !== 'macos'"
      class="mt-1.5"
    >
      Available RAM:
      {{ (availableMemory / (1024 * 1024 * 1024)).toFixed(2) }}GB /
      {{ (totalMemory / (1024 * 1024 * 1024)).toFixed() }}GB total
    </div>
    <div
      v-if="memoryUsage > maxMemoryUsage && osName === 'macos'"
      class="mt-1.5"
    >
      RAM limit: {{ (maxMemoryUsage / (1024 * 1024 * 1024)).toFixed(1) }}GB (=
      70% * {{ (totalMemory / (1024 * 1024 * 1024)).toFixed() }}GB total)
    </div>

    <div class="mt-4">
      Target exploitability:
      <Tippy
        class="inline-block cursor-help"
        max-width="500px"
        placement="bottom"
        trigger="mouseenter click"
        :delay="[200, 0]"
        :interactive="true"
      >
        <QuestionMarkCircleIcon class="w-5 h-5 text-gray-600" />
        <template #content>
          <div class="px-1 py-0.5 text-justify">
            <div>
              Specifies the acceptable distance to the Nash equilibrium. A lower
              value gives more accurate results, but also requires more
              computation time.
            </div>
            <div class="mt-3">
              <span class="underline">A more detailed description:</span>
              When a Nash equilibrium solution is obtained, the strategies of
              both players become MESs (Maximally Exploitative Strategies) to
              each other. Using this property, we define the distance to the
              Nash equilibrium of the obtained strategy as follows:
            </div>
            <div class="my-1 text-center">
              Distance = (Opponent's MES EV) - (Opponent's obtained EV).
            </div>
            <div>
              This distance is always non-negative and is zero if and only if
              the obtained strategy is a part of a particular Nash equilibrium.
              Exploitability is defined as the average distance of both players.
            </div>
          </div>
        </template>
      </Tippy>
      <input
        v-model="targetExploitability"
        type="number"
        :class="
          'w-20 ml-3 px-2 py-1 rounded-lg text-sm text-center ' +
          (targetExploitability <= 0 ? 'input-error' : '')
        "
        :disabled="store.hasSolverRun && !store.isSolverPaused"
        min="0"
        step="0.05"
      />
      %
    </div>

    <div class="mt-1">
      Maximum number of iterations:
      <input
        v-model="maxIterations"
        type="number"
        :class="
          'w-[5.5rem] ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
          (maxIterations < 0 ||
          maxIterations % 1 !== 0 ||
          maxIterations > 100000
            ? 'input-error'
            : '')
        "
        :disabled="store.hasSolverRun && !store.isSolverPaused"
        min="0"
        max="100000"
      />
    </div>

    <div
      v-if="!areFlopMatching && !store.hasSolverRun"
      class="mt-2 text-red-500 font-semibold"
    >
      Error: The input flop and the bunching data do not match.
    </div>

    <div
      v-if="
        store.isBunchingEnabled &&
        store.bunchingFlop.length === 0 &&
        !store.hasSolverRun
      "
      class="mt-2 text-orange-500 font-semibold"
    >
      Warning: Bunching effect is disabled because the data is not ready.
    </div>

    <div class="flex mt-6 gap-3">
      <button
        class="button-base button-blue"
        :disabled="
          store.hasSolverRun ||
          store.isBunchingRunning ||
          memoryUsageSelected > maxMemoryUsage ||
          !areFlopMatching ||
          numThreads < 1 ||
          numThreads > 64 ||
          numThreads % 1 !== 0 ||
          targetExploitability <= 0 ||
          maxIterations < 0 ||
          maxIterations % 1 !== 0 ||
          maxIterations > 100000
        "
        @click="runSolver"
      >
        Run Solver
      </button>
      <button
        class="button-base button-red"
        :disabled="!store.isSolverRunning"
        @click="() => (terminateFlag = true)"
      >
        Stop
      </button>
      <button
        v-if="!store.isSolverPaused"
        class="button-base button-green"
        :disabled="!store.isSolverRunning"
        @click="() => (pauseFlag = true)"
      >
        Pause
      </button>
      <button
        v-else
        class="button-base button-green"
        :disabled="
          store.isBunchingRunning ||
          numThreads < 1 ||
          numThreads > 64 ||
          numThreads % 1 !== 0 ||
          targetExploitability <= 0 ||
          maxIterations < 0 ||
          maxIterations % 1 !== 0 ||
          maxIterations > 100000
        "
        @click="resumeSolver"
      >
        Resume
      </button>
    </div>

    <div
      v-if="memoryUsage > maxMemoryUsage && !store.hasSolverRun"
      class="mt-6"
    >
      <button
        class="button-base button-red"
        @click="maxMemoryUsage = Number.POSITIVE_INFINITY"
      >
        Ignore RAM Limit (not recommended)
      </button>
    </div>

    <div v-if="store.hasSolverRun" class="mt-6">
      <div class="flex items-center">
        <span
          v-if="store.isSolverRunning || store.isFinalizing"
          class="spinner inline-block mr-3"
        ></span>
        {{
          store.isSolverRunning
            ? "Solver running..."
            : store.isFinalizing
            ? "Finalizing..."
            : store.isSolverPaused
            ? "Solver paused."
            : store.isSolverError
            ? solverErrorText
            : "Solver finished."
        }}
      </div>
      {{ iterationText }}
      <br />
      {{ exploitabilityText }}
      <br />
      {{ timeText }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import {
  useStore,
  useConfigStore,
  useTmpConfigStore,
  useSavedConfigStore,
  saveConfigTmp,
  saveConfig,
} from "../store";
import {
  MAX_AMOUNT,
  convertBetString,
  ROOT_LINE_STRING,
  INVALID_LINE_STRING,
  readableLineString,
  cardStringToNumber,
} from "../utils";
import * as invokes from "../invokes";

import { Tippy } from "vue-tippy";
import { QuestionMarkCircleIcon } from "@heroicons/vue/20/solid";

const checkConfig = (
  config: ReturnType<typeof useConfigStore>
): string | null => {
  if (config.board.length < 3) {
    return "Board must consist of at least three cards";
  }

  if (config.startingPot <= 0) {
    return "Starting pot must be positive";
  }

  if (config.startingPot > MAX_AMOUNT) {
    return "Starting pot is too large";
  }

  if (config.startingPot % 1 !== 0) {
    return "Starting pot must be an integer";
  }

  if (config.effectiveStack <= 0) {
    return "Effective stack must be positive";
  }

  if (config.effectiveStack > MAX_AMOUNT) {
    return "Effective stack is too large";
  }

  if (config.effectiveStack % 1 !== 0) {
    return "Effective stack is be an integer";
  }

  const betConfig = [
    { s: config.oopFlopBetSanitized, kind: "OOP flop bet" },
    { s: config.oopFlopRaiseSanitized, kind: "OOP flop raise" },
    { s: config.oopTurnBetSanitized, kind: "OOP turn bet" },
    { s: config.oopTurnRaiseSanitized, kind: "OOP turn raise" },
    { s: config.oopRiverBetSanitized, kind: "OOP river bet" },
    { s: config.oopRiverRaiseSanitized, kind: "OOP river raise" },
    { s: config.ipFlopBetSanitized, kind: "IP flop bet" },
    { s: config.ipFlopRaiseSanitized, kind: "IP flop raise" },
    { s: config.ipTurnBetSanitized, kind: "IP turn bet" },
    { s: config.ipTurnRaiseSanitized, kind: "IP turn raise" },
    { s: config.ipRiverBetSanitized, kind: "IP river bet" },
    { s: config.ipRiverRaiseSanitized, kind: "IP river raise" },
  ];

  for (const { s, kind } of betConfig) {
    if (!s.valid) {
      return `${kind}: ${s.s}`;
    }
  }

  if (config.donkOption) {
    if (!config.oopTurnDonkSanitized.valid) {
      return `OOP turn donk: ${config.oopTurnDonkSanitized.s}`;
    }
    if (!config.oopRiverDonkSanitized.valid) {
      return `OOP river donk: ${config.oopRiverDonkSanitized.s}`;
    }
  }

  if (config.addAllInThreshold < 0) {
    return "Invalid add all-in threshold";
  }

  if (config.forceAllInThreshold < 0) {
    return "Invalid force all-in threshold";
  }

  if (config.mergingThreshold < 0) {
    return "Invalid merging threshold";
  }

  if (
    config.expectedBoardLength > 0 &&
    config.board.length !== config.expectedBoardLength
  ) {
    return `Invalid board (expected ${config.expectedBoardLength} cards)`;
  }

  const addedLinesArray =
    config.addedLines === ""
      ? []
      : config.addedLines.split(",").map(readableLineString);

  const removedLinesArray =
    config.removedLines === ""
      ? []
      : config.removedLines.split(",").map(readableLineString);

  if (
    addedLinesArray.includes(ROOT_LINE_STRING) ||
    addedLinesArray.includes(INVALID_LINE_STRING) ||
    removedLinesArray.includes(ROOT_LINE_STRING) ||
    removedLinesArray.includes(INVALID_LINE_STRING)
  ) {
    return "Invalid line found (loaded broken configurations?)";
  }

  if (
    ![0, 3, 4, 5].includes(config.expectedBoardLength) ||
    (config.expectedBoardLength === 0 &&
      (addedLinesArray.length > 0 || removedLinesArray.length > 0)) ||
    (config.expectedBoardLength > 0 &&
      addedLinesArray.length === 0 &&
      removedLinesArray.length === 0)
  ) {
    return "Invalid configurations (loaded broken configurations?)";
  }

  return null;
};

const store = useStore();
const config = useConfigStore();
const tmpConfig = useTmpConfigStore();
const savedConfig = useSavedConfigStore();

const numThreads = ref(navigator.hardwareConcurrency || 1);
const targetExploitability = ref(0.3);
const maxIterations = ref(1000);

const isTreeBuilding = ref(false);
const isTreeBuilt = ref(false);
const treeStatus = ref("Module not loaded");
const memoryUsageRaw = ref(0);
const memoryUsageRawCompressed = ref(0);
const memoryUsageBunching = ref(0);
const osName = ref<Awaited<ReturnType<typeof invokes.osName>> | null>(null);
const maxMemoryUsage = ref(0);
const availableMemory = ref(0);
const totalMemory = ref(0);
const isCompressionEnabled = ref(false);
const solverErrorText = ref("");
const terminateFlag = ref(false);
const pauseFlag = ref(false);
const currentIteration = ref(-1);
const exploitability = ref(Number.POSITIVE_INFINITY);
const elapsedTimeMs = ref(-1);

let startTime = 0;
let exploitabilityUpdated = false;

const memoryUsage = computed(() => {
  if (store.isBunchingEnabled && store.bunchingFlop.length > 0) {
    return memoryUsageRaw.value + memoryUsageBunching.value;
  } else {
    return memoryUsageRaw.value;
  }
});

const memoryUsageCompressed = computed(() => {
  if (store.isBunchingEnabled && store.bunchingFlop.length > 0) {
    return memoryUsageRawCompressed.value + memoryUsageBunching.value;
  } else {
    return memoryUsageRawCompressed.value;
  }
});

const memoryUsageSelected = computed(() => {
  if (isCompressionEnabled.value) {
    return memoryUsageCompressed.value;
  } else {
    return memoryUsage.value;
  }
});

const areFlopMatching = computed(() => {
  const flop = savedConfig.board.slice(0, 3);
  const bunchingFlop = store.bunchingFlop;
  return (
    !store.isBunchingEnabled ||
    bunchingFlop.length === 0 ||
    (flop[0] === bunchingFlop[0] &&
      flop[1] === bunchingFlop[1] &&
      flop[2] === bunchingFlop[2])
  );
});

const iterationText = computed(() => {
  if (currentIteration.value === -1) {
    return "Allocating memory...";
  } else if (currentIteration.value === -2) {
    return "Collecting bunching data...";
  } else {
    return `Iteration: ${currentIteration.value}`;
  }
});

const exploitabilityText = computed(() => {
  if (!Number.isFinite(exploitability.value)) {
    return "";
  } else {
    const valueText = exploitability.value.toFixed(2);
    const percent = (exploitability.value * 100) / config.startingPot;
    const percentText = `${percent.toFixed(2)}%`;
    return `Exploitability: ${valueText} (${percentText})`;
  }
});

const timeText = computed(() => {
  if (elapsedTimeMs.value === -1 || !store.isSolverFinished) {
    return "";
  } else {
    return `Time: ${(elapsedTimeMs.value / 1000).toFixed(2)}s`;
  }
});

const showDebugInfo = ref(false);
const storeDebugInfo = ref({});
const configDebugInfo = ref({});
const boardTreeDebugInfo = ref({});
const treeBuildStartTime = ref(0);
const lastTreeBuildError = ref("");
const gameInitPromiseState = ref("idle");

const toggleDebugInfo = () => {
  showDebugInfo.value = !showDebugInfo.value;
  if (showDebugInfo.value) {
    refreshDebugInfo();
  }
};

const refreshDebugInfo = () => {
  storeDebugInfo.value = {
    isSolverRunning: store.isSolverRunning,
    isSolverPaused: store.isSolverPaused,
    isSolverFinished: store.isSolverFinished,
    isSolverError: store.isSolverError,
    isFinalizing: store.isFinalizing,
    hasSolverRun: store.hasSolverRun,
    isBunchingEnabled: store.isBunchingEnabled,
    isBunchingRunning: store.isBunchingRunning,
    bunchingFlop: store.bunchingFlop,
  };
  
  configDebugInfo.value = {
    board: config.board,
    startingPot: config.startingPot,
    effectiveStack: config.effectiveStack,
    donkOption: config.donkOption,
    rakePercent: config.rakePercent,
    rakeCap: config.rakeCap,
    expectedBoardLength: config.expectedBoardLength,
  };
  
  boardTreeDebugInfo.value = {
    savedConfigBoard: savedConfig.board,
    tmpConfigBoard: tmpConfig.board,
    numericBoard: tmpConfig.board.map(card => getCardNumber(card)),
    areFlopMatching: areFlopMatching.value,
    isTreeBuilt: isTreeBuilt.value,
    isTreeBuilding: isTreeBuilding.value,
    treeStatus: treeStatus.value,
    memoryUsage: memoryUsage.value,
    memoryUsageCompressed: memoryUsageCompressed.value,
    addedLines: config.addedLines,
    removedLines: config.removedLines,
    boardRaw: tmpConfig.board,
    bunchingFlop: store.bunchingFlop,
    areFlopMatchingResult: areFlopMatching.value,
  };
};

const trackTreeBuildingStatus = async () => {
  try {
    const status = await invokes.gameGetStatus?.() || "Status method not available";
    console.log("Tree status from backend:", status);
    
    refreshDebugInfo();
    
    if (typeof invokes.gameGetStatus === 'function') {
      treeStatus.value = `From backend: ${status}`;
    } else {
      console.log("gameGetStatus not implemented in invokes");
    }
  } catch (err) {
    console.error("Error checking tree status:", err);
    lastTreeBuildError.value = err.toString();
  }
};

const retryBuildTree = async () => {
  if (isTreeBuilding.value) {
    if (!confirm("Tree building is in progress. Force restart?")) {
      return;
    }
  }
  console.log("Retrying tree build...");
  isTreeBuilding.value = false;
  isTreeBuilt.value = false;
  lastTreeBuildError.value = "";
  treeStatus.value = "Preparing to build tree...";
  
  await new Promise(resolve => setTimeout(resolve, 100));
  
  buildTree();
};

const forceResetTreeState = () => {
  if (!confirm("This will reset the tree state without rebuilding. Continue?")) {
    return;
  }
  
  isTreeBuilding.value = false;
  isTreeBuilt.value = false;
  treeStatus.value = "Tree state forcibly reset";
  gameInitPromiseState.value = "idle";
  lastTreeBuildError.value = "";
  treeBuildStartTime.value = 0;
  console.log("Tree state forcibly reset");
  refreshDebugInfo();
};

const logDebugInfo = () => {
  console.log("=== DEBUG INFO ===");
  console.log("Store State:", store);
  console.log("Config State:", config);
  console.log("Tmp Config State:", tmpConfig);
  console.log("Saved Config State:", savedConfig);
  console.log("Tree Status:", {
    isTreeBuilt: isTreeBuilt.value,
    isTreeBuilding: isTreeBuilding.value,
    treeStatus: treeStatus.value,
    treeBuildStartTime: treeBuildStartTime.value ? new Date(treeBuildStartTime.value).toLocaleString() : "Not started",
    lastError: lastTreeBuildError.value,
    gameInitPromiseState: gameInitPromiseState.value
  });
  console.log("Memory Usage:", {
    raw: memoryUsageRaw.value,
    compressed: memoryUsageRawCompressed.value,
    bunching: memoryUsageBunching.value,
    total: memoryUsage.value,
    totalCompressed: memoryUsageCompressed.value,
    selected: memoryUsageSelected.value,
    max: maxMemoryUsage.value,
  });
  console.log("================");
  
  try {
    const { dialog } = window.__TAURI__;
    if (dialog) {
      dialog.message("调试信息已输出到控制台");
    }
  } catch (e) {
    console.log("Tauri API not available:", e);
  }
};

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

const buildTree = async () => {
  isTreeBuilt.value = false;
  treeBuildStartTime.value = Date.now();
  gameInitPromiseState.value = "pending";
  lastTreeBuildError.value = "";

  const configError = checkConfig(config);
  if (configError !== null) {
    treeStatus.value = `Error: ${configError}`;
    lastTreeBuildError.value = configError;
    gameInitPromiseState.value = "rejected";
    return;
  }

  saveConfigTmp();
  isTreeBuilding.value = true;
  treeStatus.value = "Building tree...";

  try {
    const numericBoard = tmpConfig.board.map(card => getCardNumber(card));
    console.log("原始卡牌:", tmpConfig.board);
    console.log("转换后的数字卡牌:", numericBoard);

    const errorString = await invokes.gameInit(
      numericBoard,
      tmpConfig.startingPot,
      tmpConfig.effectiveStack,
      tmpConfig.rakePercent / 100,
      tmpConfig.rakeCap,
      tmpConfig.donkOption,
      convertBetString(tmpConfig.oopFlopBet),
      convertBetString(tmpConfig.oopFlopRaise),
      convertBetString(tmpConfig.oopTurnBet),
      convertBetString(tmpConfig.oopTurnRaise),
      tmpConfig.donkOption ? convertBetString(tmpConfig.oopTurnDonk) : "",
      convertBetString(tmpConfig.oopRiverBet),
      convertBetString(tmpConfig.oopRiverRaise),
      tmpConfig.donkOption ? convertBetString(tmpConfig.oopRiverDonk) : "",
      convertBetString(tmpConfig.ipFlopBet),
      convertBetString(tmpConfig.ipFlopRaise),
      convertBetString(tmpConfig.ipTurnBet),
      convertBetString(tmpConfig.ipTurnRaise),
      convertBetString(tmpConfig.ipRiverBet),
      convertBetString(tmpConfig.ipRiverRaise),
      tmpConfig.addAllInThreshold / 100,
      tmpConfig.forceAllInThreshold / 100,
      tmpConfig.mergingThreshold / 100,
      tmpConfig.addedLines,
      tmpConfig.removedLines
    );

    if (errorString) {
      isTreeBuilding.value = false;
      treeStatus.value = "Error: " + errorString;
      lastTreeBuildError.value = errorString;
      gameInitPromiseState.value = "rejected";
      return;
    }

    gameInitPromiseState.value = "fulfilled";
    saveConfig();

    console.log("Tree built successfully, collecting memory info...");
    try {
      [memoryUsageRaw.value, memoryUsageRawCompressed.value] =
        await invokes.gameMemoryUsage();
      memoryUsageBunching.value = await invokes.gameMemoryUsageBunching();

      osName.value = await invokes.osName();
      [availableMemory.value, totalMemory.value] = await invokes.memory();

      if (osName.value === "macos") {
        maxMemoryUsage.value = totalMemory.value * 0.7;
      } else {
        maxMemoryUsage.value = availableMemory.value;
      }

      if (
        memoryUsage.value > maxMemoryUsage.value &&
        memoryUsageCompressed.value <= maxMemoryUsage.value
      ) {
        isCompressionEnabled.value = true;
      }

      console.log("Memory info collected successfully");
    } catch (err) {
      console.error("Error collecting memory info:", err);
      lastTreeBuildError.value = `Tree built but memory info failed: ${err}`;
    }

    isTreeBuilding.value = false;
    isTreeBuilt.value = true;
    treeStatus.value = "Successfully built tree";

    store.isSolverRunning = false;
    store.isSolverPaused = false;
    store.isSolverFinished = false;
    store.isSolverError = false;
  } catch (err) {
    console.error("Error in buildTree:", err);
    isTreeBuilding.value = false;
    treeStatus.value = `Error building tree: ${err}`;
    lastTreeBuildError.value = err.toString();
    gameInitPromiseState.value = "rejected";
  }
};

const runSolver = async () => {
  terminateFlag.value = false;
  pauseFlag.value = false;
  currentIteration.value = -1;
  exploitability.value = Number.POSITIVE_INFINITY;
  elapsedTimeMs.value = -1;

  store.isSolverRunning = true;

  startTime = performance.now();

  await invokes.setNumThreads(numThreads.value);
  await invokes.gameAllocateMemory(isCompressionEnabled.value);

  if (store.isBunchingEnabled && store.bunchingFlop.length > 0) {
    if (!areFlopMatching.value) {
      solverErrorText.value = "Error: Flop cards do not match bunching data";
      store.isSolverRunning = false;
      store.isSolverError = true;
      return;
    }
    
    currentIteration.value = -2;
    
    const errorString = await invokes.gameSetBunching();
    
    if (errorString) {
      solverErrorText.value = "Error: " + errorString;
      store.isSolverRunning = false;
      store.isSolverError = true;
      return;
    }
  }

  currentIteration.value = 0;
  exploitability.value = Math.max(await invokes.gameExploitability(), 0);
  exploitabilityUpdated = true;

  await resumeSolver();
};

const resumeSolver = async () => {
  store.isSolverRunning = true;
  store.isSolverPaused = false;

  if (startTime === 0) {
    startTime = performance.now();
    await invokes.setNumThreads(numThreads.value);
  }

  const target = (config.startingPot * targetExploitability.value) / 100;

  while (
    !terminateFlag.value &&
    currentIteration.value < maxIterations.value &&
    exploitability.value > target
  ) {
    if (pauseFlag.value) {
      const end = performance.now();
      elapsedTimeMs.value += end - startTime;
      startTime = 0;
      pauseFlag.value = false;
      store.isSolverRunning = false;
      store.isSolverPaused = true;
      return;
    }

    await invokes.gameSolveStep(currentIteration.value);
    ++currentIteration.value;
    exploitabilityUpdated = false;

    if (currentIteration.value % 10 === 0) {
      exploitability.value = Math.max(await invokes.gameExploitability(), 0);
      exploitabilityUpdated = true;
    }
  }

  if (!exploitabilityUpdated) {
    exploitability.value = Math.max(await invokes.gameExploitability(), 0);
  }

  store.isSolverRunning = false;
  store.isFinalizing = true;

  await invokes.gameFinalize();

  store.isFinalizing = false;
  store.isSolverFinished = true;

  const end = performance.now();
  elapsedTimeMs.value += end - startTime;
};
</script>

<style scoped>
.button-yellow {
  @apply bg-yellow-500 hover:bg-yellow-600 text-white;
}

/* 如果需要，可以在这里添加更多样式 */
</style>
