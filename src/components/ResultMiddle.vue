<template>
  <div class="flex shrink-0 h-12 border-y border-gray-500">
    <button
      v-for="mode in ['basics', 'graphs', 'compare'] as const"
      :key="mode"
      :class="
        'flex w-[9%] h-full items-center justify-center font-semibold text-lg transition ' +
        (chanceMode === ''
          ? displayMode === mode
            ? 'bg-blue-100 underline'
            : 'hover:bg-blue-100'
          : displayMode === mode
          ? 'bg-red-100 underline'
          : 'hover:bg-red-100')
      "
      @click="updateDisplayMode(mode)"
    >
      {{ capitalize(mode) }}
    </button>
    <button
      :class="
        'flex w-[9%] h-full items-center justify-center font-semibold text-lg transition enabled:hover:bg-blue-100 ' +
        (displayMode === 'chance' ? 'bg-blue-100 underline' : '')
      "
      :disabled="chanceMode === ''"
      @click="updateDisplayMode('chance')"
    >
      {{ capitalize(chanceMode) }}
    </button>

    <div class="flex ml-auto h-full items-center gap-1 pr-2">
      <!-- Player Button -->
      <div v-if="['basics', 'graphs', 'chance'].includes(displayMode)" class="relative">
        <button 
          class="flex h-8 px-2 items-center justify-center rounded-md border border-blue-300 bg-blue-50 hover:bg-blue-100 transition-colors text-sm"
          @click="togglePopup('player')"
        >
          <span>Player</span>
          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 ml-1" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
        
        <div v-if="activePopup === 'player'" class="absolute top-full left-0 mt-1 w-40 bg-white rounded-md shadow-lg border border-gray-200 z-50">
          <div class="p-2">
            <div class="font-medium text-xs pb-1 mb-1 border-b">{{ displayMode === 'chance' ? 'Player (Chance)' : 'Player' }}</div>
            
            <div v-if="displayMode !== 'chance'" class="flex flex-col gap-1">
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.playerBasics" value="auto" @change="updateDisplayOptions(); closePopup()">
                <span>Auto ({{ autoPlayerBasics.toUpperCase() }})</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.playerBasics" value="oop" @change="updateDisplayOptions(); closePopup()">
                <span>OOP</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.playerBasics" value="ip" @change="updateDisplayOptions(); closePopup()">
                <span>IP</span>
              </label>
            </div>
            
            <div v-else class="flex flex-col gap-1">
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.playerChance" value="auto" @change="updateDisplayOptions(); closePopup()">
                <span>Auto ({{ autoPlayerChance.toUpperCase() }})</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.playerChance" value="oop" @change="updateDisplayOptions(); closePopup()">
                <span>OOP</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.playerChance" value="ip" @change="updateDisplayOptions(); closePopup()">
                <span>IP</span>
              </label>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Bar Height Button -->
      <div v-if="['basics', 'compare'].includes(displayMode)" class="relative">
        <button 
          class="flex h-8 px-2 items-center justify-center rounded-md border border-blue-300 bg-blue-50 hover:bg-blue-100 transition-colors text-sm"
          @click="togglePopup('barHeight')"
        >
          <span>Height</span>
          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 ml-1" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
        
        <div v-if="activePopup === 'barHeight'" class="absolute top-full left-0 mt-1 w-40 bg-white rounded-md shadow-lg border border-gray-200 z-50">
          <div class="p-2">
            <div class="font-medium text-xs pb-1 mb-1 border-b">Bar Height</div>
            
            <div class="flex flex-col gap-1">
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.barHeight" value="normalized" @change="updateDisplayOptions(); closePopup()">
                <span>Normalized</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.barHeight" value="absolute" @change="updateDisplayOptions(); closePopup()">
                <span>Absolute</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.barHeight" value="full" @change="updateDisplayOptions(); closePopup()">
                <span>Full</span>
              </label>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Suit Button -->
      <div v-if="['basics', 'compare'].includes(displayMode)" class="relative">
        <button 
          class="flex h-8 px-2 items-center justify-center rounded-md border border-blue-300 bg-blue-50 hover:bg-blue-100 transition-colors text-sm"
          @click="togglePopup('suit')"
        >
          <span>Suit</span>
          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 ml-1" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
        
        <div v-if="activePopup === 'suit'" class="absolute top-full left-0 mt-1 w-40 bg-white rounded-md shadow-lg border border-gray-200 z-50">
          <div class="p-2">
            <div class="font-medium text-xs pb-1 mb-1 border-b">Suit</div>
            
            <div class="flex flex-col gap-1">
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.suit" value="grouped" @change="updateDisplayOptions(); closePopup()">
                <span>Grouped</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.suit" value="individual" @change="updateDisplayOptions(); closePopup()">
                <span>Individual</span>
              </label>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Display Button -->
      <div v-if="['basics', 'compare', 'graphs'].includes(displayMode)" class="relative">
        <button 
          class="flex h-8 px-2 items-center justify-center rounded-md border border-blue-300 bg-blue-50 hover:bg-blue-100 transition-colors text-sm"
          @click="togglePopup('display')"
        >
          <span>Display</span>
          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 ml-1" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
        
        <div v-if="activePopup === 'display'" class="absolute top-full left-0 mt-1 w-40 bg-white rounded-md shadow-lg border border-gray-200 z-50">
          <div class="p-2">
            <div class="font-medium text-xs pb-1 mb-1 border-b">Display</div>
            
            <div v-if="['basics', 'compare'].includes(displayMode)" class="flex flex-col gap-1">
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="strategyContentPair" value="show,default" @change="updateDisplayOptions(); closePopup()">
                <span>Strategy</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="strategyContentPair" value="show,eq" @change="updateDisplayOptions(); closePopup()">
                <span>Strategy + EQ</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="strategyContentPair" value="show,ev" @change="updateDisplayOptions(); closePopup()">
                <span>Strategy + EV</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="strategyContentPair" value="show,eqr" @change="updateDisplayOptions(); closePopup()">
                <span>Strategy + EQR</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="strategyContentPair" value="none,default" @change="updateDisplayOptions(); closePopup()">
                <span>Weight</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="strategyContentPair" value="none,eq" @change="updateDisplayOptions(); closePopup()">
                <span>EQ</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="strategyContentPair" value="none,ev" @change="updateDisplayOptions(); closePopup()">
                <span>EV</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="strategyContentPair" value="none,eqr" @change="updateDisplayOptions(); closePopup()">
                <span>EQR</span>
              </label>
            </div>
            
            <div v-if="displayMode === 'graphs'" class="flex flex-col gap-1">
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.contentGraphs" value="eq" @change="updateDisplayOptions(); closePopup()">
                <span>EQ</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.contentGraphs" value="ev" @change="updateDisplayOptions(); closePopup()">
                <span>EV</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.contentGraphs" value="eqr" @change="updateDisplayOptions(); closePopup()">
                <span>EQR</span>
              </label>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Chart Button (for Chance mode) -->
      <div v-if="displayMode === 'chance'" class="relative">
        <button 
          class="flex h-8 px-2 items-center justify-center rounded-md border border-blue-300 bg-blue-50 hover:bg-blue-100 transition-colors text-sm"
          @click="togglePopup('chart')"
        >
          <span>Chart</span>
          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 ml-1" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
        
        <div v-if="activePopup === 'chart'" class="absolute top-full left-0 mt-1 w-40 bg-white rounded-md shadow-lg border border-gray-200 z-50">
          <div class="p-2">
            <div class="font-medium text-xs pb-1 mb-1 border-b">Chart</div>
            
            <div class="flex flex-col gap-1">
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.chartChance" value="strategy-combos" @change="updateDisplayOptions(); closePopup()">
                <span>Strategy (Combos)</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.chartChance" value="strategy" @change="updateDisplayOptions(); closePopup()">
                <span>Strategy (%)</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.chartChance" value="eq" @change="updateDisplayOptions(); closePopup()">
                <span>Equity</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.chartChance" value="ev" @change="updateDisplayOptions(); closePopup()">
                <span>EV</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.chartChance" value="eqr" @change="updateDisplayOptions(); closePopup()">
                <span>EQR</span>
              </label>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Bar Width Button -->
      <div v-if="['basics', 'graphs', 'chance'].includes(displayMode)" class="relative">
        <button 
          class="flex h-8 px-2 items-center justify-center rounded-md border border-blue-300 bg-blue-50 hover:bg-blue-100 transition-colors text-sm"
          @click="togglePopup('barWidth')"
        >
          <span>Width</span>
          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 ml-1" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
        
        <div v-if="activePopup === 'barWidth'" class="absolute top-full left-0 mt-1 w-40 bg-white rounded-md shadow-lg border border-gray-200 z-50">
          <div class="p-2">
            <div class="font-medium text-xs pb-1 mb-1 border-b">Bar Width</div>
            
            <div class="flex flex-col gap-1">
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.barWidth" value="normalized" @change="updateDisplayOptions(); closePopup()">
                <span>Normalized</span>
              </label>
              <label v-if="displayMode !== 'chance'" class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.barWidth" value="absolute" @change="updateDisplayOptions(); closePopup()">
                <span>Absolute</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.barWidth" value="full" @change="updateDisplayOptions(); closePopup()">
                <span>Full</span>
              </label>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Action Button -->
      <div v-if="['basics', 'graphs'].includes(displayMode)" class="relative">
        <button 
          class="flex h-8 px-2 items-center justify-center rounded-md border border-blue-300 bg-blue-50 hover:bg-blue-100 transition-colors text-sm"
          @click="togglePopup('action')"
        >
          <span>Action</span>
          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 ml-1" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
        
        <div v-if="activePopup === 'action'" class="absolute top-full left-0 mt-1 w-40 bg-white rounded-md shadow-lg border border-gray-200 z-50">
          <div class="p-2">
            <div class="font-medium text-xs pb-1 mb-1 border-b">Action</div>
            
            <div class="flex flex-col gap-1">
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.actionContent" value="percentage" @change="updateDisplayOptions(); closePopup()">
                <span> %</span>
              </label>
              <label class="flex items-center gap-2 p-1 rounded hover:bg-gray-100 cursor-pointer text-sm">
                <input type="radio" v-model="displayOptions.actionContent" value="ev" @change="updateDisplayOptions(); closePopup()">
                <span> EV</span>
              </label>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, toRefs, watch, computed } from "vue";
import { capitalize } from "../utils";
import * as Types from "../result-types";

// import { Tippy } from "vue-tippy";
// import { ComputerDesktopIcon } from "@heroicons/vue/24/solid";
// import {
//   ClipboardDocumentIcon,
//   ClipboardDocumentCheckIcon,
// } from "@heroicons/vue/24/outline";

const props = defineProps<{
  displayMode: Types.DisplayMode;
  chanceMode: string;
  autoPlayerBasics: "oop" | "ip";
  autoPlayerChance: "oop" | "ip";
  copySuccess: number;
}>();

const emit = defineEmits<{
  (event: "update:display-mode", displayMode: Types.DisplayMode): void;
  (event: "update:display-options", displayOptions: Types.DisplayOptions): void;
  // (event: "copy-to-clipboard"): void;
  // (event: "reset-copy-success"): void;
}>();

const { chanceMode } = toRefs(props);
let displayModeOld = "basics" as Types.DisplayMode;
const activePopup = ref("");

// 弹出式菜单控制
const togglePopup = (name: string) => {
  if (activePopup.value === name) {
    activePopup.value = "";
  } else {
    activePopup.value = name;
  }
};

const closePopup = () => {
  activePopup.value = "";
};

// 点击外部关闭弹出菜单
const handleClickOutside = (event: MouseEvent) => {
  const target = event.target as HTMLElement;
  if (!target.closest('.relative') && activePopup.value) {
    closePopup();
  }
};

// 添加全局点击事件监听
window.addEventListener('click', handleClickOutside);

// Player 显示标签
const displayPlayerLabel = computed(() => {
  if (props.displayMode === 'chance') {
    return displayOptions.playerChance === 'auto' 
      ? `Auto (${props.autoPlayerChance.toUpperCase()})` 
      : displayOptions.playerChance.toUpperCase();
  } else {
    return displayOptions.playerBasics === 'auto' 
      ? `Auto (${props.autoPlayerBasics.toUpperCase()})` 
      : displayOptions.playerBasics.toUpperCase();
  }
});

// Display 显示标签
const getDisplayLabel = () => {
  if (['basics', 'compare'].includes(props.displayMode)) {
    const [strategy, content] = strategyContentPair.value.split(',');
    if (strategy === 'show') {
      if (content === 'default') return 'Strategy';
      if (content === 'eq') return 'Strategy+EQ';
      if (content === 'ev') return 'Strategy+EV';
      if (content === 'eqr') return 'Strategy+EQR';
    } else {
      if (content === 'default') return 'Weight';
      return content.toUpperCase();
    }
  } else if (props.displayMode === 'graphs') {
    return displayOptions.contentGraphs.toUpperCase();
  }
  return '';
};

// Chart 显示标签
const getChartLabel = () => {
  const chart = displayOptions.chartChance;
  if (chart === 'strategy-combos') return 'Strategy (Combos)';
  if (chart === 'strategy') return 'Strategy (%)';
  if (chart === 'eq') return 'Equity';
  if (chart === 'ev') return 'EV';
  if (chart === 'eqr') return 'EQR';
  return chart;
};

watch(chanceMode, (newValue, oldValue) => {
  if (newValue && !oldValue) {
    displayModeOld = props.displayMode;
    emit("update:display-mode", "chance");
  } else if (!newValue && oldValue) {
    emit("update:display-mode", displayModeOld);
  }
});

const displayOptions = reactive<Types.DisplayOptions>({
  playerBasics: "auto",
  playerChance: "auto",
  barHeight: "normalized",
  suit: "grouped",
  strategy: "show",
  contentBasics: "default",
  contentGraphs: "eq",
  chartChance: "strategy-combos",
  barWidth: "normalized",
  actionContent: "percentage",
});

const strategyContentPair = ref("show,default");

const savedDisplayOptions = localStorage.getItem("display-options");

if (savedDisplayOptions) {
  const saved: Types.DisplayOptions = JSON.parse(savedDisplayOptions);

  if (Types.barHeightList.includes(saved?.barHeight)) {
    displayOptions.barHeight = saved.barHeight;
  }
  if (Types.suitList.includes(saved?.suit)) {
    displayOptions.suit = saved.suit;
  }
  if (Types.strategyList.includes(saved?.strategy)) {
    displayOptions.strategy = saved.strategy;
  }
  if (Types.contentBasicsList.includes(saved?.contentBasics)) {
    displayOptions.contentBasics = saved.contentBasics;
  }
  if (Types.contentGraphsList.includes(saved?.contentGraphs)) {
    displayOptions.contentGraphs = saved.contentGraphs;
  }
  if (Types.chartChanceList.includes(saved?.chartChance)) {
    displayOptions.chartChance = saved.chartChance;
  }

  strategyContentPair.value = [
    displayOptions.strategy,
    displayOptions.contentBasics,
  ].join(",");

  emit("update:display-options", displayOptions);
}

const updateDisplayMode = (displayMode: Types.DisplayMode) => {
  if (displayMode !== "chance") {
    displayModeOld = displayMode;
  }
  emit("update:display-mode", displayMode);
  closePopup(); // 切换模式时关闭所有弹出菜单
};

const updateDisplayOptions = () => {
  const options = displayOptions;
  const [strategy, content] = strategyContentPair.value.split(",");
  options.strategy = strategy as Types.DisplayOptions["strategy"];
  options.contentBasics = content as Types.DisplayOptions["contentBasics"];
  localStorage.setItem("display-options", JSON.stringify(options));
  emit("update:display-options", options);
};

// const copyToClipboard = () => {
//   emit("copy-to-clipboard");
// };

// const onCopyTooltipLeave = () => {
//   emit("reset-copy-success");
// };
</script>

<style scoped>
.relative {
  position: relative;
}
</style>