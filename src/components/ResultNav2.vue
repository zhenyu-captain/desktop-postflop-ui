<template>
  <div
    ref="navDiv"
    class="flex shrink-0 h-[10.5rem] gap-2 p-2 overflow-x-auto whitespace-nowrap snug bg-gray-50 dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 transition-colors duration-200 scroll-smooth custom-scrollbar"
    style="scrollbar-width: thin; scrollbar-color: rgba(156, 163, 175, 0.5) transparent;"
  >
    <div
      v-for="spot in spots"
      :key="spot.index"
      :class="
        'flex flex-col relative h-full px-2 py-1 justify-start ' +
        'rounded-lg shadow-md border-[2px] transition-all duration-200 ' +
        (spot.type === 'chance'
          ? isSelectedChanceSkipped && spot.index > selectedChanceIndex
            ? 'border-gray-300 dark:border-gray-600 bg-gray-100 dark:bg-gray-700 opacity-75 '
            : 'hover:border-red-500 bg-white dark:bg-gray-700 hover:shadow-lg '
          : spot.type === 'root' 
            ? 'hover:border-blue-500 bg-white dark:bg-gray-700 hover:shadow-lg '
            : spot.type === 'terminal'
              ? 'hover:border-green-500 bg-white dark:bg-gray-700 hover:shadow-lg '
              : 'hover:border-blue-500 bg-white dark:bg-gray-700 hover:shadow-lg ') +
        (spot.index === selectedChanceIndex
          ? 'border-red-500 cursor-default shadow-lg'
          : spot.index === selectedSpotIndex
          ? 'border-blue-500 cursor-default shadow-lg'
          : spot.type === 'chance' &&
            isSelectedChanceSkipped &&
            spot.index > selectedChanceIndex
          ? 'border-gray-300 dark:border-gray-600 cursor-default'
          : 'border-gray-300 dark:border-gray-600 cursor-pointer')
      "
      style="animation: cardEnter 0.3s ease-out;"
      @click="selectSpot(spot.index, false)"
    >
      <!-- Root or Chance -->
      <template v-if="spot.type === 'root' || spot.type === 'chance'">
        <div
          :class="
            'px-1.5 pt-1 pb-0.5 font-semibold text-gray-800 dark:text-gray-200 ' +
            (spot.type === 'root' ? 'text-blue-700 dark:text-blue-300' : 'text-red-700 dark:text-red-300') + ' ' +
            (spot.index === selectedChanceIndex
              ? ''
              : isSelectedChanceSkipped && spot.index > selectedChanceIndex
              ? 'opacity-70'
              : 'group-hover:opacity-100 opacity-90')
          "
        >
          {{ spot.player.toUpperCase() }}
        </div>
        <div
          class="flex flex-col flex-grow px-3 items-center justify-evenly"
        >
          <div class="relative my-1">
            <span
              v-for="card of spotCards(spot)"
              :key="card.rank + card.suit"
              :class="
                (spot.type === 'root'
                  ? 'mx-1 font-semibold '
                  : 'inline-block w-8 text-center font-semibold ') + card.colorClass
              "
            >
              {{ card.rank + card.suit }}
            </span>
            <template
              v-if="spot.type === 'chance' && spot.selectedIndex !== -1"
            >
              <button
                :class="
                  'absolute -top-[1.375rem] left-1/2 -ml-3 w-6 h-6 rounded-full transition-transform duration-200 ' +
                  (isCardAvailable(spot, 0, 1)
                    ? 'opacity-70 hover:opacity-100 hover:scale-110 text-gray-700 dark:text-gray-300 hover:text-red-500 dark:hover:text-red-400'
                    : 'text-red-300 dark:text-red-500 cursor-not-allowed')
                "
                @click.stop="
                  isCardAvailable(spot, 0, 1) && dealArrow(spot, 0, 1)
                "
              >
                <ChevronUpIcon class="w-full h-full" />
              </button>
              <button
                :class="
                  'absolute -left-[1.375rem] top-1/2 -mt-3 w-6 h-6 rounded-full transition-transform duration-200 ' +
                  (isCardAvailable(spot, 1, 0)
                    ? 'opacity-70 hover:opacity-100 hover:scale-110 text-gray-700 dark:text-gray-300 hover:text-red-500 dark:hover:text-red-400'
                    : 'text-red-300 dark:text-red-500 cursor-not-allowed')
                "
                @click.stop="
                  isCardAvailable(spot, 1, 0) && dealArrow(spot, 1, 0)
                "
              >
                <ChevronLeftIcon class="w-full h-full" />
              </button>
              <button
                :class="
                  'absolute -right-[1.375rem] top-1/2 -mt-3 w-6 h-6 rounded-full transition-transform duration-200 ' +
                  (isCardAvailable(spot, -1, 0)
                    ? 'opacity-70 hover:opacity-100 hover:scale-110 text-gray-700 dark:text-gray-300 hover:text-red-500 dark:hover:text-red-400'
                    : 'text-red-300 dark:text-red-500 cursor-not-allowed')
                "
                @click.stop="
                  isCardAvailable(spot, -1, 0) && dealArrow(spot, -1, 0)
                "
              >
                <ChevronRightIcon class="w-full h-full" />
              </button>
              <button
                :class="
                  'absolute -bottom-[1.375rem] left-1/2 -ml-3 w-6 h-6 rounded-full transition-transform duration-200 ' +
                  (isCardAvailable(spot, 0, -1)
                    ? 'opacity-70 hover:opacity-100 hover:scale-110 text-gray-700 dark:text-gray-300 hover:text-red-500 dark:hover:text-red-400'
                    : 'text-red-300 dark:text-red-500 cursor-not-allowed')
                "
                @click.stop="
                  isCardAvailable(spot, 0, -1) && dealArrow(spot, 0, -1)
                "
              >
                <ChevronDownIcon class="w-full h-full" />
              </button>
            </template>
          </div>
          <div
            :class="
              'text-gray-700 dark:text-gray-300 border-t pt-1 w-full ' +
              (spot.index === selectedChanceIndex
                ? ''
                : isSelectedChanceSkipped && spot.index > selectedChanceIndex
                ? 'opacity-70'
                : 'group-hover:opacity-100 opacity-90')
            "
          >
            <div class="font-medium text-center">Pot {{ spot.pot }}</div>
            <div class="font-medium text-center">Stack {{ spot.stack }}</div>
          </div>
        </div>

        <button
          v-if="
            spot.index === selectedChanceIndex &&
            spot.selectedIndex !== -1 &&
            !isDealing
          "
          class="absolute top-1.5 right-1.5 opacity-70 hover:opacity-100 hover:scale-110 hover:text-red-500 dark:hover:text-red-400 text-gray-700 dark:text-gray-300 rounded-full p-0.5 transition-all duration-200"
          @click="deal(spot.selectedIndex)"
        >
          <XMarkIcon class="w-5 h-5" />
        </button>
      </template>

      <!-- Player -->
      <template v-if="spot.type === 'player'">
        <div
          :class="
            'px-1.5 py-1 font-semibold text-gray-800 dark:text-gray-200 ' +
            (spot.index === selectedSpotIndex ? 'text-blue-700 dark:text-blue-300' : 'opacity-90')
          "
        >
          {{ spot.player.toUpperCase() }}
        </div>
        <div class="flex-grow overflow-y-auto px-1 custom-scrollbar">
          <button
            v-for="action of spot.actions"
            :key="action.index"
            :class="
              'flex w-full px-2 py-1 my-1 rounded-md transition-all duration-200 ' +
              (action.isSelected 
                ? 'border border-blue-500 text-blue-700 dark:text-blue-300 shadow-sm transform translate-x-1'
                : 'border border-transparent hover:border-blue-300 dark:hover:border-blue-600 hover:translate-x-1')
            "
            style="animation: fadeIn 0.2s ease-out;"
            @click.stop="play(spot.index, action.index)"
          >
            <span class="inline-block relative w-4 mr-1 transition-transform duration-200" 
                  :class="action.isSelected ? 'scale-110' : ''">
              <span
                v-if="
                  spot.index === selectedSpotIndex &&
                  !(selectedChanceIndex !== -1 && !canChanceReports)
                "
                class="absolute top-[0.3125rem] left-0 w-3 h-3 rounded-sm"
                :style="{ backgroundColor: action.color }"
              ></span>
              <span v-if="action.isSelected">
                <CheckIcon class="absolute top-[0.3125rem] -left-0.5 w-4 h-4 text-blue-600 dark:text-blue-400" />
              </span>
            </span>
            <span
              :class="
                'pr-0.5 font-medium transition-colors duration-200 ' +
                (action.isSelected || spot.index === selectedSpotIndex
                  ? 'text-gray-800 dark:text-gray-200'
                  : 'text-gray-700 dark:text-gray-300')
              "
            >
              {{ action.name }}
              <span v-if="action.amount !== '0'" 
                    :class="action.isSelected ? 'text-blue-600 dark:text-blue-400 font-semibold' : ''">
                {{ action.amount }}
              </span>
            </span>
            <span
              v-if="spot.index === selectedSpotIndex && rates != null"
              :class="
                'ml-auto pl-1.5 transition-colors duration-200 ' +
                (action.isSelected
                  ? 'text-blue-600 dark:text-blue-400 font-semibold'
                  : 'text-gray-600 dark:text-gray-400')
              "
            >
              [{{ (rates[action.index] * 100).toFixed(1) }}%]
            </span>
          </button>
        </div>
      </template>

      <!-- Terminal -->
      <template v-else-if="spot.type === 'terminal'">
        <div
          :class="
            'px-1.5 pt-1 pb-1 font-semibold text-green-700 dark:text-green-300 border-b'
          "
        >
          {{ spot.player.toUpperCase() }}
        </div>
        <div
          class="flex flex-col flex-grow items-center justify-center gap-2 text-gray-700 dark:text-gray-300 px-2 pt-2"
          style="animation: fadeIn 0.3s ease-out;"
        >
          <div v-if="spot.equityOop === 0 || spot.equityOop === 1" 
               class="px-3 py-1.5 font-medium border border-green-500 text-green-700 dark:text-green-300 rounded-md transition-all duration-200 hover:shadow-sm w-full text-center"
               style="animation: pulse 1.5s infinite ease-in-out;">
            <span class="text-lg font-semibold">{{ ["IP", "OOP"][spot.equityOop] }} Wins</span>
          </div>
          <div v-else-if="spot.equityOop !== -1" class="w-full">
            <div class="flex w-full px-2 py-1 mb-1 border border-green-300 dark:border-green-700 rounded-md transition-all duration-200 hover:border-green-500 dark:hover:border-green-600 hover:shadow-sm">
              <span>OOP</span>
              <span class="ml-auto pl-2 font-medium text-green-700 dark:text-green-300">
                {{ (spot.equityOop * 100).toFixed(1) }}%
              </span>
            </div>
            <div class="flex w-full px-2 py-1 mb-1 border border-green-300 dark:border-green-700 rounded-md transition-all duration-200 hover:border-green-500 dark:hover:border-green-600 hover:shadow-sm">
              <span>IP</span>
              <span class="ml-auto pl-2 font-medium text-green-700 dark:text-green-300">
                {{ ((1 - spot.equityOop) * 100).toFixed(1) }}%
              </span>
            </div>
            <div class="flex w-full px-2 py-1 border border-green-300 dark:border-green-700 rounded-md transition-all duration-200 hover:border-green-500 dark:hover:border-green-600 hover:shadow-sm">
              <span>Pot</span>
              <span class="ml-auto pl-2 font-medium text-green-700 dark:text-green-300">
                {{ spot.pot }}
              </span>
            </div>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, toRefs, ref, watch, onMounted, onUnmounted } from "vue";
import { useSavedConfigStore } from "../store";
import { cardText, average, colorString } from "../utils";
import * as invokes from "../invokes";
import eventBus from "../eventBus";

import {
  Results,
  ChanceReports,
  Spot,
  SpotRoot,
  SpotChance,
  SpotPlayer,
} from "../result-types";

import {
  CheckIcon,
  XMarkIcon,
  ChevronDownIcon,
  ChevronLeftIcon,
  ChevronRightIcon,
  ChevronUpIcon,
} from "@heroicons/vue/20/solid";

const foldColor = { red: 0x3b, green: 0x82, blue: 0xf6 }; // blue-500
const checkColor = { red: 0x10, green: 0xb9, blue: 0x81 }; // emerald-500
const callColor = { red: 0x10, green: 0xb9, blue: 0x81 }; // emerald-500
const betColorGradient = [
  { red: 0x60, green: 0xa5, blue: 0xfa }, // blue-400
  { red: 0x38, green: 0xbf, blue: 0xa8 }, // 中间颜色：蓝绿过渡色
  { red: 0x34, green: 0xd3, blue: 0x99 }, // emerald-400
];

const actionColor = (
  name: string,
  index: number,
  numActions: number,
  numBetActions: number
) => {
  if (name === "Fold") return colorString(foldColor);
  if (name === "Check") return colorString(checkColor);
  if (name === "Call") return colorString(callColor);

  if (numBetActions === 1) return colorString(betColorGradient[0]);
  if (index === numActions - 1) {
    const denom = numBetActions === 2 ? 2 : 1;
    return colorString(betColorGradient[(betColorGradient.length - 1) / denom]);
  }

  const betIndex = index - (numActions - numBetActions);
  const colorRate = betIndex / (numBetActions - 1);

  const gradientRate = colorRate * (betColorGradient.length - 1);
  const gradientIndex = Math.floor(gradientRate);
  const r = gradientRate - gradientIndex;

  const color1 = betColorGradient[gradientIndex];
  const color2 = betColorGradient[gradientIndex + 1];

  const retColor = { red: 0, green: 0, blue: 0 };
  for (const primary of ["red", "green", "blue"] as const) {
    const primary1 = color1[primary];
    const primary2 = color2[primary];
    retColor[primary] = Math.round(primary1 * (1 - r) + primary2 * r);
  }

  return colorString(retColor);
};

const props = defineProps<{
  tabId?: string;
  isHandlerUpdated: boolean;
  isLocked: boolean;
  cards: number[][];
  dealtCard: number;
}>();

const emit = defineEmits<{
  (event: "update:is-handler-updated", value: boolean): void;
  (event: "update:is-locked", value: boolean): void;
  (
    event: "trigger-update",
    selectedSpot: Spot | null,
    selectedChance: SpotChance | null,
    currentBoard: number[],
    results: Results,
    chanceReports: ChanceReports | null,
    totalBetAmount: number[]
  ): void;
}>();

const navDiv = ref<HTMLDivElement | null>(null);

const config = useSavedConfigStore();
const { isHandlerUpdated, dealtCard } = toRefs(props);

const spots = ref<Spot[]>([]);
const rates = ref<number[] | null>(null);
const selectedSpotIndex = ref(-1);
const selectedChanceIndex = ref(-1);
const isDealing = ref(false);
const canChanceReports = ref(false);

const selectedSpot = computed(() =>
  selectedSpotIndex.value === -1 ||
  selectedSpotIndex.value >= spots.value.length
    ? null
    : spots.value[selectedSpotIndex.value]
);

const selectedChance = computed(() =>
  selectedChanceIndex.value === -1
    ? null
    : (spots.value[selectedChanceIndex.value] as SpotChance)
);

const isSelectedChanceSkipped = computed(() => {
  return selectedChance.value?.selectedIndex === -1;
});

const currentBoard = computed(() => {
  const board = [...config.board];
  const endIndex = selectedChance.value
    ? selectedChanceIndex.value
    : selectedSpotIndex.value;
  for (let i = 3; i < endIndex; ++i) {
    const spot = spots.value[i];
    if (spot.type === "chance") {
      const card = spot.selectedIndex;
      if (card !== -1) board.push(card);
      else return board;
    }
  }
  return board;
});

let selectedSpotIndexTmp = -1;
let selectedChanceIndexTmp = -1;

let results: Results | null = null;
let chanceReports: ChanceReports | null = null;
let totalBetAmount = [0, 0];
let totalBetAmountAppended = [0, 0];

watch(isHandlerUpdated, async () => {
  if (!isHandlerUpdated.value) return;

  // 打印调试信息，确认配置正确
  console.log("ResultNav初始化root节点:", {
    boardLength: config.board.length,
    board: config.board.map(card => card),
    pot: config.startingPot,
    stack: config.effectiveStack
  });

  const l = config.board.length;
  const spot: SpotRoot = {
    type: "root",
    index: 0,
    player: l === 3 ? "flop" : l === 4 ? "turn" : "river",
    selectedIndex: -1,
    board: config.board,
    pot: config.startingPot,
    stack: config.effectiveStack,
  };
  spots.value = [spot];

  await selectSpot(1, true);
  emit("update:is-handler-updated", false);
});

const selectSpot = async (
  spotIndex: number,
  needSplice: boolean,
  fromDeal = false
) => {
  if (
    props.isLocked ||
    (!needSplice &&
      ((spotIndex === selectedSpotIndex.value && !fromDeal) ||
        spotIndex === selectedChanceIndex.value ||
        (spots.value[spotIndex].type === "chance" &&
          isSelectedChanceSkipped.value &&
          spotIndex > selectedChanceIndex.value)))
  ) {
    return;
  }

  if (spotIndex === 0) {
    await selectSpot(1, true);
    return;
  }

  // start processing
  emit("update:is-locked", true);

  // avoid unnecessary update of refs
  selectedSpotIndexTmp = selectedSpotIndex.value;
  selectedChanceIndexTmp = selectedChanceIndex.value;

  // when from deal, update river dead cards and terminal equity
  if (fromDeal) {
    const findRiverIndex = spots.value
      .slice(selectedChanceIndexTmp + 3)
      .findIndex((spot) => spot.type === "chance");
    let riverIndex = -1;
    if (findRiverIndex !== -1) {
      riverIndex = findRiverIndex + selectedChanceIndexTmp + 3;
    }
    const riverSpot =
      riverIndex === -1 ? null : (spots.value[riverIndex] as SpotChance);
    selectedChanceIndexTmp = -1;

    if (riverSpot) {
      const history = spots.value
        .slice(1, riverIndex)
        .map((spot) => spot.selectedIndex);
      await invokes.gameApplyHistory(history);
      const possibleCards = await invokes.gamePossibleCards();
      for (let i = 0; i < 52; ++i) {
        const isDead = !(possibleCards & (1n << BigInt(i)));
        riverSpot.cards[i].isDead = isDead;
        if (riverSpot.selectedIndex === i && isDead) {
          riverSpot.cards[i].isSelected = false;
          riverSpot.selectedIndex = -1;
        }
      }
    }

    const riverSkipped = riverSpot?.selectedIndex === -1;
    const lastSpot = spots.value[spots.value.length - 1];

    if (
      !riverSkipped &&
      lastSpot.type === "terminal" &&
      lastSpot.equityOop !== 0 &&
      lastSpot.equityOop !== 1
    ) {
      const history = spots.value
        .slice(1, -1)
        .map((spot) => spot.selectedIndex);
      await invokes.gameApplyHistory(history);
      const results = await invokes.gameGetResults();
      if (!results.isEmpty) {
        lastSpot.equityOop = average(results.equity[0], results.normalizer[0]);
      } else {
        lastSpot.equityOop = -1;
      }
    }
  }

  // update indices of selected spot and selected chance
  if (!needSplice && spots.value[spotIndex].type === "chance") {
    selectedChanceIndexTmp = spotIndex;
    if (selectedSpotIndexTmp < spotIndex + 1) {
      selectedSpotIndexTmp = spotIndex + 1;
    }
  } else {
    selectedSpotIndexTmp = spotIndex;
    if (spotIndex <= selectedChanceIndexTmp) {
      selectedChanceIndexTmp = -1;
    } else if (selectedChanceIndexTmp === -1) {
      selectedChanceIndexTmp = spots.value
        .slice(0, spotIndex)
        .findIndex(
          (spot) => spot.type === "chance" && spot.selectedIndex === -1
        );
    }
  }

  let endIndex: number;
  if (selectedChanceIndexTmp === -1) {
    endIndex = selectedSpotIndexTmp;
  } else {
    endIndex = selectedChanceIndexTmp;
  }

  const history = spots.value
    .slice(1, endIndex)
    .map((spot) => spot.selectedIndex);

  // apply history
  await invokes.gameApplyHistory(history);

  // obtain results
  results = await invokes.gameGetResults();

  let append: number[] = [];
  if (selectedChanceIndexTmp !== -1) {
    append = spots.value
      .slice(selectedChanceIndexTmp, selectedSpotIndexTmp)
      .map((spot) => spot.selectedIndex);
  }

  // obtain actions after skipped chances
  const nextActions = await invokes.gameActionsAfter(append);

  canChanceReports.value =
    selectedChanceIndexTmp !== -1 &&
    spots.value
      .slice(selectedChanceIndexTmp + 3, selectedSpotIndexTmp)
      .every((spot) => spot.type !== "chance") &&
    nextActions[0] !== "chance";

  // if possible, obtain chance reports
  if (canChanceReports.value) {
    let player: "oop" | "ip" | "terminal";
    let numActions: number;

    if (nextActions[0] === "terminal") {
      player = "terminal";
      numActions = 0;
    } else {
      player = append.length % 2 === 1 ? "oop" : "ip";
      numActions = nextActions.length;
    }

    chanceReports = await invokes.gameGetChanceReports(
      append,
      player,
      numActions
    );
  } else {
    chanceReports = null;
  }

  // update total bet amounts
  totalBetAmount = await invokes.gameTotalBetAmount([]);
  totalBetAmountAppended = await invokes.gameTotalBetAmount(append);

  // if need to splice, splice spots
  if (needSplice) {
    if (nextActions[0] === "terminal") {
      spliceSpotsTerminal(spotIndex);
    } else if (nextActions[0] === "chance") {
      await spliceSpotsChance(spotIndex);
    } else {
      spliceSpotsPlayer(spotIndex, nextActions);
    }
  }

  // update action rates if necessary
  const spot = spots.value[selectedSpotIndexTmp];
  if (spot.type === "player" && selectedChanceIndexTmp === -1) {
    const playerIndex = spot.player === "oop" ? 0 : 1;
    if (results.isEmpty & (1 << playerIndex)) {
      rates.value = null;
    } else {
      const n = props.cards[playerIndex].length;
      rates.value = Array.from({ length: spot.actions.length }, (_, i) => {
        if (!results) throw new Error("null results");
        const rates = results.strategy.slice(i * n, (i + 1) * n);
        return average(rates, results.normalizer[playerIndex]);
      });
    }
  } else {
    rates.value = null;
  }

  // update refs
  selectedSpotIndex.value = selectedSpotIndexTmp;
  selectedChanceIndex.value = selectedChanceIndexTmp;
  isDealing.value = false;

  // emit event
  emit(
    "trigger-update",
    selectedSpot.value,
    selectedChance.value,
    currentBoard.value,
    results,
    chanceReports,
    totalBetAmount
  );

  // scroll to selected spot
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

const spliceSpotsTerminal = (spotIndex: number) => {
  if (!results) throw new Error("null results");

  const prevSpot = spots.value[spotIndex - 1] as SpotPlayer;
  const prevAction = prevSpot.actions[prevSpot.selectedIndex];

  const chanceIndex = selectedChanceIndexTmp;
  const chanceSkipped =
    chanceIndex !== -1 &&
    (spots.value[chanceIndex] as SpotChance).selectedIndex === -1;

  let equityOop;
  if (prevAction.name === "Fold") {
    equityOop = prevSpot.player === "oop" ? 0 : 1;
  } else if (chanceSkipped || results.isEmpty) {
    equityOop = -1;
  } else {
    equityOop = average(results.equity[0], results.normalizer[0]);
  }

  const betSum = totalBetAmountAppended[0] + totalBetAmountAppended[1];
  spots.value.splice(spotIndex, spots.value.length, {
    type: "terminal",
    index: spotIndex,
    player: "equity",
    selectedIndex: -1,
    prevPlayer: prevSpot.player,
    equityOop,
    pot: config.startingPot + betSum,
  });
};

const spliceSpotsChance = async (spotIndex: number) => {
  type SpotTurn = SpotRoot | SpotChance;
  const prevSpot = spots.value[spotIndex - 1] as SpotPlayer;
  const turnSpot = spots.value
    .slice(0, spotIndex)
    .find((spot) => spot.player === "turn") as SpotTurn | undefined;

  let append: number[] = [];
  if (selectedChanceIndexTmp !== -1) {
    append = spots.value
      .slice(selectedChanceIndexTmp, spotIndex)
      .map((spot) => spot.selectedIndex);
  }

  let possibleCards = 0n;
  if (!(turnSpot?.type === "chance" && turnSpot.selectedIndex === -1)) {
    possibleCards = await invokes.gamePossibleCards();
  }

  append.push(-1);
  const nextActions = await invokes.gameActionsAfter(append);

  let numBetActions = nextActions.length;
  while (
    numBetActions > 0 &&
    nextActions[nextActions.length - numBetActions].split(":")[1] === "0"
  ) {
    --numBetActions;
  }

  if (selectedChanceIndexTmp === -1) {
    canChanceReports.value = true;
    const numActions = nextActions.length;
    chanceReports = await invokes.gameGetChanceReports(
      append,
      "oop",
      numActions
    );
  }

  spots.value.splice(
    spotIndex,
    spots.value.length,
    {
      type: "chance",
      index: spotIndex,
      player: turnSpot ? "river" : "turn",
      selectedIndex: -1,
      prevPlayer: prevSpot.player,
      cards: Array.from({ length: 52 }, (_, i) => ({
        card: i,
        isSelected: false,
        isDead: !(possibleCards & (1n << BigInt(i))),
      })),
      pot: config.startingPot + 2 * totalBetAmountAppended[0],
      stack: config.effectiveStack - totalBetAmountAppended[0],
    },
    {
      type: "player",
      index: spotIndex + 1,
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
          color: actionColor(name, i, nextActions.length, numBetActions),
        };
      }),
    }
  );

  ++selectedSpotIndexTmp;

  if (selectedChanceIndexTmp === -1) {
    selectedChanceIndexTmp = spotIndex;
  }
};

const spliceSpotsPlayer = (spotIndex: number, actions: string[]) => {
  const prevSpot = spots.value[spotIndex - 1];
  const player = prevSpot.player === "oop" ? "ip" : "oop";

  let numBetActions = actions.length;
  if (actions[0].split(":")[1] === "0") --numBetActions;
  if (actions[1]?.split(":")[1] === "0") --numBetActions;

  spots.value.splice(spotIndex, spots.value.length, {
    type: "player",
    index: spotIndex,
    player,
    selectedIndex: -1,
    actions: actions.map((action, i) => {
      const [name, amount] = action.split(":");
      return {
        index: i,
        name,
        amount,
        isSelected: false,
        color: actionColor(name, i, actions.length, numBetActions),
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

  await selectSpot(spotIndex + 1, true);
};

const deal = async (card: number) => {
  const spot = selectedChance.value;
  if (!spot) throw new Error("null spot");

  isDealing.value = true;

  if (spot.selectedIndex !== -1) {
    spot.cards[spot.selectedIndex].isSelected = false;
  }
  spot.cards[card].isSelected = true;
  spot.selectedIndex = card;

  await selectSpot(selectedSpotIndex.value, false, true);
};

const dealArrow = async (
  spot: SpotChance,
  rankDir: number,
  suitDir: number
) => {
  const offset = rankDir ? 4 * rankDir : suitDir;
  let card = spot.selectedIndex + offset;

  for (; 0 <= card && card < 52; card += offset) {
    if (!spot.cards[card].isDead) break;
  }

  if (card < 0 || 52 <= card) {
    throw new Error("invalid card");
  }

  isDealing.value = true;

  spot.cards[spot.selectedIndex].isSelected = false;
  spot.cards[card].isSelected = true;
  spot.selectedIndex = card;

  const selectedChanceIndexBak = selectedChanceIndex.value;
  selectedChanceIndex.value = spot.index;

  await selectSpot(selectedSpotIndex.value, false, true);

  if (
    selectedChanceIndex.value === -1 &&
    selectedChanceIndexBak !== spot.index
  ) {
    selectedChanceIndex.value = selectedChanceIndexBak;
  }
};

const isCardAvailable = (
  spot: SpotChance,
  rankDir: number,
  suitDir: number
) => {
  let card = spot.selectedIndex;

  if (rankDir) {
    card += 4 * rankDir;
    for (; 0 <= card && card < 52; card += 4 * rankDir) {
      if (!spot.cards[card].isDead) return true;
    }
  } else {
    const rank = card >>> 2;
    card += suitDir;
    for (; card >>> 2 === rank; card += suitDir) {
      if (!spot.cards[card].isDead) return true;
    }
  }

  return false;
};

watch(dealtCard, async (card) => {
  if (card === -1) return;
  await deal(card);
});

const spotCards = (spot: SpotRoot | SpotChance) => {
  if (spot.type === "root") {
    return spot.board.map((card) => cardText(card));
  } else if (spot.selectedIndex === -1) {
    return [{ rank: "?", suit: "", colorClass: "text-black" }];
  } else {
    return [cardText(spot.selectedIndex)];
  }
};

// 添加事件监听器以响应决策路径数据请求
onMounted(() => {
  eventBus.on('request-decision-path-data', provideDecisionPathData);
});

onUnmounted(() => {
  eventBus.off('request-decision-path-data', provideDecisionPathData);
});

// 提供决策路径导航数据
const provideDecisionPathData = () => {
  const pathData = {
    spots: spots.value,
    currentState: {
      selectedSpotIndex: selectedSpotIndex.value,
      selectedChanceIndex: selectedChanceIndex.value,
      currentBoard: currentBoard.value,
      isSelectedChanceSkipped: isSelectedChanceSkipped.value
    },
    navigation: {
      selectedSpot: selectedSpot.value,
      selectedChance: selectedChance.value,
      canChanceReports: canChanceReports.value
    },
    betAmounts: totalBetAmount
  };
  
  eventBus.emit('decision-path-data', pathData);
};
</script>

<style scoped>
::-webkit-scrollbar {
  height: 6px;
  width: 6px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background-color: rgba(156, 163, 175, 0.5);
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background-color: rgba(156, 163, 175, 0.7);
}

.dark ::-webkit-scrollbar-thumb {
  background-color: rgba(75, 85, 99, 0.5);
}

.dark ::-webkit-scrollbar-thumb:hover {
  background-color: rgba(75, 85, 99, 0.7);
}

.custom-scrollbar {
  scrollbar-width: thin;
  scrollbar-color: rgba(156, 163, 175, 0.5) transparent;
}

.dark .custom-scrollbar {
  scrollbar-width: thin;
  scrollbar-color: rgba(75, 85, 99, 0.5) transparent;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(4px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes cardEnter {
  from { transform: scale(0.95); opacity: 0.6; }
  to { transform: scale(1); opacity: 1; }
}

@keyframes pulse {
  0% { box-shadow: 0 0 0 0 rgba(34, 197, 94, 0.2); }
  70% { box-shadow: 0 0 0 6px rgba(34, 197, 94, 0); }
  100% { box-shadow: 0 0 0 0 rgba(34, 197, 94, 0); }
}

button:active {
  transform: scale(0.97);
}

/* 确保跨浏览器一致性 */
button:focus {
  outline: none;
}

button:focus-visible {
  outline: 2px solid #60a5fa;
  outline-offset: 2px;
}
</style>