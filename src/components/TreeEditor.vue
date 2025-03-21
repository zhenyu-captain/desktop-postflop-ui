<template>
  <!-- Error -->
  <div 
    v-if="isTreeError" 
    class="p-4 rounded-lg bg-red-100 text-red-700 border border-red-300 flex items-center mb-4"
  >
    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
    </svg>
    <span class="font-medium">Error: Failed to build tree (loaded broken tree?)</span>
  </div>

  <!-- Navigation -->
  <div
    v-if="!isTreeError"
    ref="navDiv"
    class="flex h-[10.5rem] gap-2 p-2 overflow-x-auto whitespace-nowrap rounded-lg bg-gray-50 dark:bg-gray-800 shadow-sm mb-4"
  >
    <div
      v-for="spot in spots"
      :key="spot.index"
      :class="
        'flex flex-col h-full px-2 py-1 justify-start ' +
        'rounded-lg shadow-md border-[2px] transition-all duration-200 ' +
        (spot.type === 'chance'
          ? 'hover:border-red-500 bg-red-50 dark:bg-red-900/20 '
          : 'hover:border-blue-500 bg-white dark:bg-gray-700 ') +
        (spot.index === selectedSpotIndex
          ? spot.type === 'chance' ? 'border-red-500 ' : 'border-blue-500 cursor-default'
          : 'border-gray-300 dark:border-gray-600 cursor-pointer')
      "
      @click="selectSpot(spot.index, false, false, true)"
    >
      <!-- Root or Chance -->
      <template v-if="spot.type === 'root' || spot.type === 'chance'">
        <div
          class="px-1.5 pt-1 pb-0.5 font-semibold text-gray-800 dark:text-gray-200"
        >
          {{ spot.player.toUpperCase() }}
        </div>
        <div
          class="flex flex-col flex-grow px-3 items-center justify-evenly"
        >
          <div class="text-gray-700 dark:text-gray-300">
            <div class="font-medium">Pot {{ spot.pot }}</div>
            <div class="font-medium">Stack {{ spot.stack }}</div>
          </div>
        </div>
      </template>

      <!-- Player -->
      <template v-if="spot.type === 'player'">
        <div
          :class="
            'px-1.5 py-1 font-semibold text-gray-800 dark:text-gray-200'
          "
        >
          {{ spot.player.toUpperCase() }}
        </div>
        <div class="flex-grow overflow-y-auto">
          <button
            v-for="action of spot.actions"
            :key="action.index"
            :class="
              'flex w-full px-1.5 py-0.5 rounded-md transition-colors ' +
              (action.isSelected 
                ? 'bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300' 
                : 'hover:bg-blue-50 dark:hover:bg-blue-900/20')
            "
            @click.stop="play(spot.index, action.index)"
          >
            <span class="inline-block relative w-4 mr-0.5">
              <span v-if="action.isSelected">
                <CheckIcon class="absolute top-[0.1875rem] -left-0.5 w-4 h-4 text-blue-600 dark:text-blue-400" />
              </span>
            </span>
            <span class="font-medium">
              {{ action.name }}
              {{ action.amount === "0" ? "" : action.amount }}
            </span>
          </button>
          <div
            v-if="spot.actions.length === 0"
            class="flex w-full px-1.5 py-1 text-gray-500 dark:text-gray-400 italic"
          >
            (No actions)
          </div>
        </div>
      </template>

      <!-- Terminal -->
      <template v-else-if="spot.type === 'terminal'">
        <div class="px-1.5 pt-1 pb-0.5 font-semibold text-gray-800 dark:text-gray-200">
          {{ spot.player.toUpperCase() }}
        </div>
        <div class="flex flex-col flex-grow items-center justify-evenly">
          <div v-if="spot.equityOop === 0 || spot.equityOop === 1" class="px-3 font-medium text-gray-700 dark:text-gray-300">
            {{ ["IP", "OOP"][spot.equityOop] }} Wins
          </div>
          <div class="px-3 font-medium text-gray-700 dark:text-gray-300">Pot {{ spot.pot }}</div>
        </div>
      </template>
    </div>
  </div>

  <!-- Invalid lines error -->
  <div
    v-if="!isTreeError && invalidLinesArray.length > 0"
    class="p-4 mb-4 rounded-lg bg-red-50 dark:bg-red-900/20 border border-red-300 dark:border-red-700"
  >
    <div class="flex font-medium text-red-700 dark:text-red-400">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      Invalid Terminal{{ invalidLinesArray.length > 1 ? "s" : "" }}:
    </div>
    <div class="mt-2 ml-7 text-red-600 dark:text-red-400 space-y-1">
      <div v-for="invalidLine in invalidLinesArray" :key="invalidLine" class="flex items-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
        {{ invalidLine }}
      </div>
    </div>
  </div>

  <div v-if="!isTreeError" class="my-6 border-t border-gray-200 dark:border-gray-700"></div>

  <!-- Edit -->

  <div v-if="!isTreeError" class="bg-white dark:bg-gray-700 rounded-lg shadow p-4 mb-6">
    <h3 class="text-lg font-semibold mb-3 text-gray-800 dark:text-gray-200 flex items-center">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-blue-500" viewBox="0 0 20 20" fill="currentColor">
        <path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z" />
      </svg>
      Edit Tree
    </h3>
    
    <div class="flex flex-wrap gap-3 items-center">
      <button
        class="flex items-center justify-center px-4 py-2 rounded-lg font-medium transition-all shadow-sm hover:shadow
          bg-blue-600 text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500
          disabled:opacity-60 disabled:cursor-not-allowed disabled:hover:shadow-none"
        :disabled="
          isSelectedTerminal ||
          isAfterAllin ||
          betAmount < minAmount ||
          betAmount > maxAmount ||
          betAmount % 1 !== 0 ||
          existingAmounts.includes(betAmount)
        "
        @click="addBetAction"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M10 5a1 1 0 011 1v3h3a1 1 0 110 2h-3v3a1 1 0 11-2 0v-3H6a1 1 0 110-2h3V6a1 1 0 011-1z" clip-rule="evenodd" />
        </svg>
        Add Bet Action
      </button>

      <button
        class="flex items-center justify-center px-4 py-2 rounded-lg font-medium transition-all shadow-sm hover:shadow
          bg-red-600 text-white hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500
          disabled:opacity-60 disabled:cursor-not-allowed disabled:hover:shadow-none"
        :disabled="selectedSpotIndex === 1"
        @click="removeSelectedNode"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" />
        </svg>
        Remove Selected Node
      </button>

      <div class="flex items-center gap-2 ml-2">
        <span class="font-medium text-gray-700 dark:text-gray-300">Bet amount:</span>
        <input
          v-model="betAmount"
          type="number"
          :class="
            'w-24 px-3 py-2 rounded-lg text-sm text-center transition-all border border-gray-300 dark:border-gray-600 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 focus:outline-none dark:bg-gray-800 dark:text-gray-200 ' +
            (betAmount < minAmount || betAmount > maxAmount || betAmount % 1 !== 0
              ? 'border-red-500 focus:ring-red-500 focus:border-red-500'
              : '')
          "
          :min="minAmount"
          :max="maxAmount"
          @keydown.enter="addBetAction"
        />
        <span v-if="!isSelectedTerminal && !isAfterAllin" class="text-gray-600 dark:text-gray-400">
          ({{ (amountRate * 100).toFixed(1) }}% of the pot)
        </span>
      </div>
    </div>
  </div>

  <!-- Save/Cancel -->
  <div class="bg-white dark:bg-gray-700 rounded-lg shadow p-4 mb-6">
    <h3 class="text-lg font-semibold mb-3 text-gray-800 dark:text-gray-200 flex items-center">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-blue-500" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
      </svg>
      Finish Editing
    </h3>

    <div class="flex gap-3">
      <button
        class="flex items-center justify-center px-4 py-2 min-w-[120px] rounded-lg font-medium transition-all shadow-sm hover:shadow
          bg-blue-600 text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500
          disabled:opacity-60 disabled:cursor-not-allowed disabled:hover:shadow-none"
        :disabled="isTreeError || invalidLinesArray.length > 0"
        @click="saveEdit"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
        </svg>
        Save Edit
      </button>

      <button 
        class="flex items-center justify-center px-4 py-2 min-w-[120px] rounded-lg font-medium transition-all shadow-sm hover:shadow
          bg-red-600 text-white hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
        @click="cancelEdit"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
        </svg>
        Cancel Edit
      </button>
    </div>
  </div>

  <!-- Lines -->

  <div
    v-if="
      !isTreeError &&
      (addedLinesArray.length > 0 || removedLinesArray.length > 0)
    "
    class="bg-white dark:bg-gray-700 rounded-lg shadow p-4"
  >
    <h3 class="text-lg font-semibold mb-3 text-gray-800 dark:text-gray-200 flex items-center">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-blue-500" viewBox="0 0 20 20" fill="currentColor">
        <path d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9z" />
        <path fill-rule="evenodd" d="M4 5a2 2 0 012-2 3 3 0 003 3h2a3 3 0 003-3 2 2 0 012 2v11a2 2 0 01-2 2H6a2 2 0 01-2-2V5zm3 4a1 1 0 000 2h.01a1 1 0 100-2H7zm3 0a1 1 0 000 2h3a1 1 0 100-2h-3zm-3 4a1 1 0 100 2h.01a1 1 0 100-2H7zm3 0a1 1 0 100 2h3a1 1 0 100-2h-3z" clip-rule="evenodd" />
      </svg>
      Modified Lines
    </h3>

    <div class="space-y-4">
      <div v-if="addedLinesArray.length > 0" class="bg-green-50 dark:bg-green-900/20 p-3 rounded-lg border border-green-200 dark:border-green-800">
        <div class="font-medium text-green-800 dark:text-green-400 mb-2 flex items-center">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1.5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
          </svg>
          Added Line{{ addedLinesArray.length > 1 ? "s" : "" }}
        </div>
        <div class="space-y-1.5 ml-2">
          <div
            v-for="(addedLine, index) in addedLinesArray"
            :key="addedLine"
            class="group flex items-center text-gray-800 dark:text-gray-200 hover:bg-green-100 dark:hover:bg-green-900/30 px-2 py-1 rounded transition-colors"
          >
            <button 
              class="mr-2 p-1 text-gray-400 hover:text-red-500 rounded-full hover:bg-red-100 dark:hover:bg-red-900/30 transition-colors" 
              @click="deleteAddedLine(index)"
              title="Delete this line"
            >
              <TrashIcon class="w-4 h-4" />
            </button>
            <div class="font-medium">{{ addedLine }}</div>
          </div>
        </div>
      </div>

      <div v-if="removedLinesArray.length > 0" class="bg-red-50 dark:bg-red-900/20 p-3 rounded-lg border border-red-200 dark:border-red-800">
        <div class="font-medium text-red-800 dark:text-red-400 mb-2 flex items-center">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1.5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
          </svg>
          Removed Line{{ removedLinesArray.length > 1 ? "s" : "" }}
        </div>
        <div class="space-y-1.5 ml-2">
          <div
            v-for="(removedLine, index) in removedLinesArray"
            :key="removedLine"
            class="group flex items-center text-gray-800 dark:text-gray-200 hover:bg-red-100 dark:hover:bg-red-900/30 px-2 py-1 rounded transition-colors"
          >
            <button 
              class="mr-2 p-1 text-gray-400 hover:text-green-500 rounded-full hover:bg-green-100 dark:hover:bg-green-900/30 transition-colors" 
              @click="deleteRemovedLine(index)"
              title="Restore this line"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd" />
              </svg>
            </button>
            <div class="font-medium line-through">{{ removedLine }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, ref } from "vue";
import { useConfigStore } from "../store";
import { convertBetString, readableLineString } from "../utils";
import { Spot, SpotRoot, SpotChance, SpotPlayer } from "../result-types";
import * as invokes from "../invokes";

import { CheckIcon } from "@heroicons/vue/20/solid";
import { TrashIcon } from "@heroicons/vue/24/outline";

const emit = defineEmits<{
  (event: "save", addedLines: string, removedLines: string): void;
  (event: "cancel"): void;
}>();

const navDiv = ref(null as HTMLDivElement | null);

const config = useConfigStore();
let isLocked = false;

const boardLength = config.expectedBoardLength;

const isTreeError = !(await invokes.treeNew(
  boardLength,
  config.startingPot,
  config.effectiveStack,
  config.donkOption,
  convertBetString(config.oopFlopBet),
  convertBetString(config.oopFlopRaise),
  convertBetString(config.oopTurnBet),
  convertBetString(config.oopTurnRaise),
  config.donkOption ? convertBetString(config.oopTurnDonk) : "",
  convertBetString(config.oopRiverBet),
  convertBetString(config.oopRiverRaise),
  config.donkOption ? convertBetString(config.oopRiverDonk) : "",
  convertBetString(config.ipFlopBet),
  convertBetString(config.ipFlopRaise),
  convertBetString(config.ipTurnBet),
  convertBetString(config.ipTurnRaise),
  convertBetString(config.ipRiverBet),
  convertBetString(config.ipRiverRaise),
  config.addAllInThreshold / 100,
  config.forceAllInThreshold / 100,
  config.mergingThreshold / 100,
  config.addedLines,
  config.removedLines
));

const rootSpot: SpotRoot = {
  type: "root",
  index: 0,
  player: boardLength === 3 ? "flop" : boardLength === 4 ? "turn" : "river",
  selectedIndex: -1,
  board: config.board,
  pot: config.startingPot,
  stack: config.effectiveStack,
};
const spots = ref<Spot[]>([rootSpot]);
const selectedSpotIndex = ref(-1);

const isSelectedTerminal = computed(() => {
  if (isLocked || selectedSpotIndex.value === -1) return false;
  const selectedSpot = spots.value[selectedSpotIndex.value];
  return selectedSpot.type === "terminal";
});

const betAmount = ref(0);
const totalBetAmount = ref([0, 0]);
const prevBetAmount = ref(0);

const isAfterAllin = computed(() => {
  const maxTotalBetAmount = Math.max(...totalBetAmount.value);
  return maxTotalBetAmount === config.effectiveStack;
});

const maxAmount = computed(() => {
  if (isSelectedTerminal.value) return 0;
  const maxTotalBetAmount = Math.max(...totalBetAmount.value);
  return config.effectiveStack - (maxTotalBetAmount - prevBetAmount.value);
});

const minAmount = computed(() => {
  const betMinus = config.effectiveStack - maxAmount.value;
  const min = Math.min(...totalBetAmount.value) - betMinus;
  const max = Math.max(...totalBetAmount.value) - betMinus;
  return Math.min(Math.max(2 * max - min, 1), maxAmount.value);
});

const amountRate = computed(() => {
  const pot = config.startingPot + 2 * Math.max(...totalBetAmount.value);
  const amount = betAmount.value - prevBetAmount.value;
  return amount / pot;
});

const existingAmounts = computed(() => {
  if (isLocked || selectedSpotIndex.value === -1) return [];
  const ret: number[] = [];
  const spot = spots.value[selectedSpotIndex.value] as SpotPlayer;
  for (const action of spot.actions) {
    if (action.amount !== "0") {
      ret.push(Number(action.amount));
    }
  }
  return ret;
});

const addedLines = ref(await invokes.treeAddedLines());
const removedLines = ref(await invokes.treeRemovedLines());
const invalidLines = ref(await invokes.treeInvalidTerminals());

const addedLinesArray = computed(() =>
  addedLines.value === ""
    ? []
    : addedLines.value.split(",").map(readableLineString)
);

const removedLinesArray = computed(() =>
  removedLines.value === ""
    ? []
    : removedLines.value.split(",").map(readableLineString)
);

const invalidLinesArray = computed(() =>
  invalidLines.value === ""
    ? []
    : invalidLines.value.split(",").map(readableLineString)
);

const encodeLine = (spotIndex: number) => {
  const ret: string[] = [];
  for (let i = 1; i < spotIndex; ++i) {
    const spot = spots.value[i];
    if (spot.type === "player") {
      const action = spot.actions[spot.selectedIndex];
      if (action.name === "Fold") {
        ret.push("F");
      } else if (action.name === "Check") {
        ret.push("X");
      } else if (action.name === "Call") {
        ret.push("C");
      } else if (action.name === "Bet") {
        ret.push("B" + action.amount);
      } else if (action.name === "Raise") {
        ret.push("R" + action.amount);
      } else if (action.name === "Allin") {
        ret.push("A" + action.amount);
      }
    }
  }
  return ret;
};

const selectSpot = async (
  spotIndex: number,
  needSplice: boolean,
  needRebuild: boolean,
  needAmountUpdate: boolean
) => {
  if (!needSplice && !needRebuild && spotIndex === selectedSpotIndex.value) {
    return;
  }

  if (spotIndex === 0) {
    await selectSpot(1, true, false, selectedSpotIndex.value !== 1);
    return;
  }

  if (!needSplice && spots.value[spotIndex]?.type === "chance") {
    await selectSpot(spotIndex + 1, false, false, true);
    return;
  }

  isLocked = true;

  if (needRebuild) {
    const selectedSpotIndexTmp = selectedSpotIndex.value;
    const line = encodeLine(spots.value.length - 1);
    spots.value = [rootSpot];

    selectedSpotIndex.value = 1;
    totalBetAmount.value = [0, 0];

    await invokes.treeBackToRoot();
    await pushResultsPlayer();

    for (let i = 0; i < line.length; ++i) {
      const index = await invokes.treePlay(line[i]);
      if (index === -1) {
        needAmountUpdate = true;
        break;
      }

      const spot = spots.value[selectedSpotIndex.value] as SpotPlayer;
      const action = spot.actions[index];
      spot.selectedIndex = index;
      action.isSelected = true;

      ++selectedSpotIndex.value;
      totalBetAmount.value = await invokes.treeTotalBetAmount();

      if (await invokes.treeIsTerminalNode()) {
        pushResultsTerminal();
      } else if (await invokes.treeIsChanceNode()) {
        await pushResultsChance();
        ++selectedSpotIndex.value;
      } else {
        await pushResultsPlayer();
      }
    }

    if (selectedSpotIndexTmp < selectedSpotIndex.value) {
      selectedSpotIndex.value = selectedSpotIndexTmp;
    }
  } else {
    selectedSpotIndex.value = spotIndex;
  }

  const line = encodeLine(selectedSpotIndex.value);
  await invokes.treeApplyHistory(line);
  totalBetAmount.value = await invokes.treeTotalBetAmount();

  if (needSplice) {
    spots.value.splice(selectedSpotIndex.value);
    if (await invokes.treeIsTerminalNode()) {
      pushResultsTerminal();
    } else if (await invokes.treeIsChanceNode()) {
      await pushResultsChance();
      ++selectedSpotIndex.value;
    } else {
      await pushResultsPlayer();
    }
  }

  const prev = spots.value[selectedSpotIndex.value - 1];
  if (prev.type === "player") {
    prevBetAmount.value = Number(prev.actions[prev.selectedIndex].amount);
  } else {
    prevBetAmount.value = 0;
  }

  if (needAmountUpdate) {
    betAmount.value = minAmount.value;
  }

  isLocked = false;
  autoScrollNav();
};

const autoScrollNav = async () => {
  await nextTick();
  if (navDiv.value) {
    const selectedChild = navDiv.value.children[selectedSpotIndex.value];
    if (selectedChild) {
      selectedChild.scrollIntoView({
        behavior: "smooth",
        inline: "center",
      });
    }
  }
};

const pushResultsTerminal = () => {
  const prevSpot = spots.value[selectedSpotIndex.value - 1] as SpotPlayer;
  const prevAction = prevSpot.actions[prevSpot.selectedIndex];

  let equityOop = -1;
  if (prevAction.name === "Fold") {
    equityOop = prevSpot.player === "oop" ? 0 : 1;
  }

  spots.value.push({
    type: "terminal",
    index: selectedSpotIndex.value,
    player: "end",
    selectedIndex: -1,
    prevPlayer: prevSpot.player,
    equityOop,
    pot: config.startingPot + totalBetAmount.value[0] + totalBetAmount.value[1],
  });
};

const pushResultsChance = async () => {
  type SpotTurn = SpotRoot | SpotChance;
  const prevSpot = spots.value[selectedSpotIndex.value - 1] as SpotPlayer;
  const turnSpot = spots.value.find((spot) => spot.player === "turn") as
    | SpotTurn
    | undefined;

  const nextActions = await invokes.treeActions();

  spots.value.push(
    {
      type: "chance",
      index: selectedSpotIndex.value,
      player: turnSpot ? "river" : "turn",
      selectedIndex: -1,
      prevPlayer: prevSpot.player,
      cards: Array.from({ length: 52 }, (_, i) => ({
        card: i,
        isSelected: false,
        isDead: true,
      })),
      pot: config.startingPot + 2 * totalBetAmount.value[0],
      stack: config.effectiveStack - totalBetAmount.value[0],
    },
    {
      type: "player",
      index: selectedSpotIndex.value + 1,
      player: "oop",
      selectedIndex: -1,
      actions: nextActions.map((action, i) => {
        const [name, amount] = action.split(":");
        return {
          index: i,
          name,
          amount,
          rate: -1,
          isSelected: false,
          color: "#000",
        };
      }),
    }
  );
};

const pushResultsPlayer = async () => {
  const prevSpot = spots.value[selectedSpotIndex.value - 1];
  const player = prevSpot.player === "oop" ? "ip" : "oop";

  const actions = await invokes.treeActions();

  spots.value.push({
    type: "player",
    index: selectedSpotIndex.value,
    player,
    selectedIndex: -1,
    actions: actions.map((action, i) => {
      const [name, amount] = action.split(":");
      return {
        index: i,
        name,
        amount,
        isSelected: false,
        color: "#000",
      };
    }),
  });
};

const play = async (spotIndex: number, actionIndex: number) => {
  const spot = spots.value[spotIndex] as SpotPlayer;

  if (spot.selectedIndex !== -1) {
    spot.actions[spot.selectedIndex].isSelected = false;
  }
  spot.actions[actionIndex].isSelected = true;
  spot.selectedIndex = actionIndex;

  await selectSpot(spotIndex + 1, true, false, true);
};

const addBetAction = async () => {
  const isRaise = totalBetAmount.value[0] !== totalBetAmount.value[1];
  await invokes.treeAddBetAction(betAmount.value, isRaise);
  await selectSpot(selectedSpotIndex.value, false, true, false);
  addedLines.value = await invokes.treeAddedLines();
  removedLines.value = await invokes.treeRemovedLines();
  invalidLines.value = await invokes.treeInvalidTerminals();
};

const removeSelectedNode = async () => {
  await invokes.treeRemoveCurrentNode();
  let prevIndex = selectedSpotIndex.value - 1;
  if (spots.value[prevIndex].type === "chance") --prevIndex;
  await selectSpot(prevIndex, false, true, true);
  addedLines.value = await invokes.treeAddedLines();
  removedLines.value = await invokes.treeRemovedLines();
  invalidLines.value = await invokes.treeInvalidTerminals();
};

const saveEdit = () => {
  emit("save", addedLines.value, removedLines.value);
};

const cancelEdit = () => {
  emit("cancel");
};

const deleteAddedLine = async (index: number) => {
  const addedLinesArray = addedLines.value.split(",");
  const line = addedLinesArray[index];

  await invokes.treeDeleteAddedLine(line);
  await selectSpot(selectedSpotIndex.value, false, true, false);

  addedLines.value = await invokes.treeAddedLines();
  removedLines.value = await invokes.treeRemovedLines();
  invalidLines.value = await invokes.treeInvalidTerminals();
};

const deleteRemovedLine = async (index: number) => {
  const removedLinesArray = removedLines.value.split(",");
  const line = removedLinesArray[index];

  await invokes.treeDeleteRemovedLine(line);
  await selectSpot(selectedSpotIndex.value, false, true, false);

  addedLines.value = await invokes.treeAddedLines();
  removedLines.value = await invokes.treeRemovedLines();
  invalidLines.value = await invokes.treeInvalidTerminals();
};

await selectSpot(0, true, false, true);
</script>
