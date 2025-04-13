<template>
  <div class="min-w-[1080px] bg-white" :style="{ height: clientHeight + 'px' }">
    <!-- 顶部导航栏 - 使用现代UI设计 -->
    <nav class="flex w-full h-12 shadow-lg bg-white text-slate-800 border-b border-gray-200">
      <div class="flex items-center px-4">
        <span class="text-lg font-semibold">Pro</span>
        <span class="text-sm ml-2 text-gray-500">v0.1.2</span>
      </div>
      
      <div class="flex items-center space-x-1 ml-8">
        <button 
          v-for="item in menuItems" 
          :key="item.view"
          :class="[
            'px-4 h-full transition-colors hover:bg-gray-100',
            (store.sideView === item.view && !store.treeEditMode) || 
            (item.view === 'tree-editor' && store.treeEditMode) ? 'bg-gray-100 text-blue-600' : ''
          ]"
          @click="handleMenuItemClick(item)"
          :disabled="item.view === 'tree-editor' && !canToggleTreeEditor"
        >
          {{ item.label }}
        </button>
      </div>

      <div class="flex items-center ml-auto px-4">
        <button 
          :class="[
            'px-6 h-8 rounded transition-colors',
            store.navView === 'solver' ? 'bg-blue-600 text-white' : 'bg-gray-200 text-gray-700'
          ]"
          @click="store.navView = store.navView === 'solver' ? 'results' : 'solver'"
        >
          {{ store.navView === 'solver' ? '查看结果' : '返回求解器' }}
        </button>
      </div>
    </nav>

    <!-- 主内容区域 - 移除左侧菜单栏，保留主要内容 -->
    <div v-show="store.navView === 'solver'" class="h-[calc(100%-3rem)] overflow-auto">
      <div class="w-full py-6 px-8 overflow-y-auto">
        <div class="flex mb-4">
          <div
            :class="
              'mb-5 pl-2 pr-3 pb-0.5 text-lg font-bold border-l-8 border-b-2 ' +
              'border-blue-600 rounded rounded-br-none'
            "
          >
            {{ header }}
          </div>
        </div>

        <!-- Tree Editor Component -->
        <div v-if="store.treeEditMode">
          <Suspense>
            <template #default>
              <TreeEditor @save="saveTreeEdit" @cancel="cancelTreeEdit" :showSaveSuccess="showSaveSuccess" />
            </template>
            <template #fallback>
              <div class="flex items-center justify-center h-40">
                <div class="text-lg">Loading...</div>
              </div>
            </template>
          </Suspense>
        </div>
        <!-- Regular Components -->
        <div v-else>
          <div v-show="store.sideView === 'oop-range'">
            <RangeEditor :player="0" />
          </div>
          <div v-show="store.sideView === 'ip-range'">
            <RangeEditor :player="1" />
          </div>
          <div v-show="store.sideView === 'board'">
            <BoardSelector />
          </div>
          <div v-show="store.sideView === 'tree-config'">
            <TreeConfig />
          </div>
          <div v-show="store.sideView === 'bunching'">
            <BunchingEffect />
          </div>
          <div v-show="store.sideView === 'run-solver'">
            <RunSolver />
          </div>
          <div v-show="store.sideView === 'file-operations'">
            <div class="file-operations">
              <div class="section-title mb-6">
                <h2 class="text-xl font-bold">文件操作</h2>
                <p class="text-gray-600 mt-1">保存和加载求解方案与配置</p>
              </div>
              
              <div class="flex flex-col space-y-6">
                <!-- 保存配置区域 -->
                <div class="card p-5 border rounded-lg shadow-sm bg-white">
                  <h3 class="text-lg font-semibold mb-3">保存配置</h3>
                  <p class="text-gray-600 mb-4">将当前的所有配置（牌局设置、范围、游戏树等）保存为JSON文件，以便日后复用。</p>
                  
                  <div class="flex items-center">
                    <button 
                      class="px-4 py-2 rounded bg-indigo-600 text-white hover:bg-indigo-700 transition-colors flex items-center"
                      @click="handleSaveConfig"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" />
                      </svg>
                      保存配置
                    </button>
                  </div>
                </div>
                
                <!-- 加载配置区域 -->
                <div class="card p-5 border rounded-lg shadow-sm bg-white">
                  <h3 class="text-lg font-semibold mb-3">加载配置</h3>
                  <p class="text-gray-600 mb-4">从之前保存的JSON文件中加载所有配置（牌局设置、范围、游戏树等）。</p>
                  
                  <div class="flex items-center">
                    <button 
                      class="px-4 py-2 rounded bg-purple-600 text-white hover:bg-purple-700 transition-colors flex items-center"
                      @click="handleLoadConfig"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
                      </svg>
                      加载配置
                    </button>
                  </div>
                </div>
                
                <!-- 保存方案区域 -->
                <div class="card p-5 border rounded-lg shadow-sm bg-white">
                  <h3 class="text-lg font-semibold mb-3">保存方案</h3>
                  <p class="text-gray-600 mb-4">将当前的求解结果保存为二进制文件，以便日后快速载入。</p>
                  
                  <div class="flex items-center">
                    <button 
                      class="px-4 py-2 rounded bg-green-600 text-white hover:bg-green-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center"
                      @click="handleSaveBin"
                      :disabled="!store.canSaveSolution"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" />
                      </svg>
                      保存解决方案
                    </button>
                    <div class="ml-4">
                      <span v-if="!store.canSaveSolution" class="text-orange-500">
                        需要先完成求解才能保存
                      </span>
                    </div>
                  </div>
                </div>
                
                <!-- 加载方案区域 -->
                <div class="card p-5 border rounded-lg shadow-sm bg-white">
                  <h3 class="text-lg font-semibold mb-3">加载方案</h3>
                  <p class="text-gray-600 mb-4">从之前保存的二进制文件中加载求解结果，无需重新计算。</p>
                  
                  <div class="flex items-center">
                    <button 
                      class="px-4 py-2 rounded bg-blue-600 text-white hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center"
                      @click="handleLoadBin"
                      :disabled="!store.canLoadSolution"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
                      </svg>
                      加载解决方案
                    </button>
                    <div class="ml-4">
                      <span v-if="store.isLoading" class="text-blue-500">
                        正在加载...
                      </span>
                    </div>
                  </div>
                </div>
              </div>
              
              <!-- 状态消息 -->
              <div v-if="store.lastSaveSuccess || store.lastLoadSuccess || configSaveSuccess || configLoadSuccess || store.lastSaveError || store.lastLoadError || configSaveError || configLoadError" 
                   class="mt-6 p-4 rounded-lg" 
                   :class="{
                     'bg-green-100 border-l-4 border-green-500': store.lastSaveSuccess || store.lastLoadSuccess || configSaveSuccess || configLoadSuccess,
                     'bg-red-100 border-l-4 border-red-500': store.lastSaveError || store.lastLoadError || configSaveError || configLoadError
                   }">
                <div class="flex">
                  <div class="flex-shrink-0">
                    <svg v-if="store.lastSaveSuccess || store.lastLoadSuccess || configSaveSuccess || configLoadSuccess" class="h-5 w-5 text-green-500" viewBox="0 0 20 20" fill="currentColor">
                      <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                    </svg>
                    <svg v-else class="h-5 w-5 text-red-500" viewBox="0 0 20 20" fill="currentColor">
                      <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
                    </svg>
                  </div>
                  <div class="ml-3">
                    <p class="text-sm leading-5 font-medium" 
                       :class="{
                         'text-green-800': store.lastSaveSuccess || store.lastLoadSuccess || configSaveSuccess || configLoadSuccess,
                         'text-red-800': store.lastSaveError || store.lastLoadError || configSaveError || configLoadError
                       }">
                      {{ getStatusMessage() }}
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>
          <div v-if="store.sideView === 'about'">
            <AboutPage />
          </div>
        </div>
      </div>
    </div>

    <!-- 结果页面 - 使用现代UI样式 -->
    <div v-show="store.navView === 'results'" class="w-full h-[calc(100%-3rem)] overflow-hidden">      
      <ResultViewer />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted } from "vue";
import { useStore, useConfigStore } from "../store";
import * as invokes from "../invokes";
import { showSaveDialog, showLoadDialog, showSaveConfigDialog, showLoadConfigDialog } from "../invokes";

import RangeEditor from "./RangeEditor.vue";
import BoardSelector from "./BoardSelector.vue";
import TreeConfig from "./TreeConfig.vue";
import TreeEditor from "./TreeEditor.vue";
import BunchingEffect from "./BunchingEffect.vue";
import RunSolver from "./RunSolver.vue";
import AboutPage from "./AboutPage.vue";
import ResultViewer from "./ResultViewer.vue";

const store = useStore();
const config = useConfigStore();
const showSaveSuccess = ref(false);
const inTreeEditor = ref(false);
const header = computed(() => {
  if (store.treeEditMode) {
    if (inTreeEditor.value) {
      // 直接从菜单进入Tree Editor
      return "Tree Editor";
    } else {
      // 从Tree Config进入Tree Editor
      return "Tree Configuration > Tree Preview & Edit";
    }
  }
  return store.headers[store.sideView].join(" > ");
});

// 添加菜单项配置 - 添加了tree-editor选项在tree-config后面和文件操作菜单项
const menuItems = [
  { view: 'oop-range', label: 'OOP Range' },
  { view: 'ip-range', label: 'IP Range' },
  { view: 'board', label: 'Board' },
  { view: 'tree-config', label: 'Tree Config' },
  { view: 'tree-editor', label: 'Tree Editor' },
  { view: 'bunching', label: 'Bunching' },
  { view: 'run-solver', label: 'Run Solver' },
  { view: 'file-operations', label: '文件操作' },
  { view: 'about', label: 'About' }
];

const clientHeight = ref(0);
const updateClientHeight = () => {
  clientHeight.value = document.documentElement.clientHeight - 0.01;
};

// 处理菜单项点击
const handleMenuItemClick = (item) => {
  if (item.view === 'tree-editor') {
    // 处理Tree Editor按钮点击
    inTreeEditor.value = true;
    toggleTreeEditor();
  } else if (store.treeEditMode) {
    // 如果在Tree Editor中，先退出
    cancelTreeEdit();
    store.sideView = item.view;
  } else {
    // 正常导航
    store.sideView = item.view;
    // 如果当前进入了Tree Config页面，需要记录下来以便支持从Tree Config进入Tree Editor
    if (item.view === 'tree-config') {
      inTreeEditor.value = false;
    }
  }
};

// 之前的handleMenuClick函数废弃，保留函数签名避免其他地方报错
const handleMenuClick = (view: string) => {
  // 为兼容性保留此函数
  const item = menuItems.find(i => i.view === view);
  if (item) {
    handleMenuItemClick(item);
  }
};

const canToggleTreeEditor = computed(() => {
  // 修改为永远返回true，这样从任何页面都可以点击Tree Editor按钮
  // return store.sideView === 'tree-config' || store.treeEditMode;
  return true;
});

const toggleTreeEditor = () => {
  if (store.treeEditMode) {
    // 退出tree editor模式
    cancelTreeEdit();
  } else {
    // 进入tree editor模式 - 移除条件检查
    // else if (canToggleTreeEditor.value) {
    // 从任何页面都可以进入Tree Editor
    inTreeEditor.value = true;
    if (config.expectedBoardLength === 0) {
      config.expectedBoardLength = Math.max(config.board.length, 3);
    }
    store.treeEditMode = true;
    // 不需要加入导航路径，因为是直接从菜单进入
    // if (!inTreeEditor.value) {
    //   store.headers["tree-config"].push("Tree Preview & Edit");
    // }
  }
};

const saveTreeEdit = (addedLines: string, removedLines: string) => {
  config.addedLines = addedLines;
  config.removedLines = removedLines;
  if (config.addedLines === "" && config.removedLines === "") {
    config.expectedBoardLength = 0;
  }
  
  if (inTreeEditor.value) {
    // 如果是直接从Tree Editor菜单进入的，保存后保持在Tree Editor页面
    // 只显示保存成功提示
    showSaveSuccess.value = true;
    setTimeout(() => {
      showSaveSuccess.value = false;
    }, 3000); // 3秒后自动隐藏提示
  } else {
    // 如果是从Tree Config进入的，保存后返回Tree Config
    store.treeEditMode = false;
    store.headers["tree-config"].pop();
    // 不显示保存成功提示，因为已经返回到Tree Config页面
  }
};

const cancelTreeEdit = () => {
  store.treeEditMode = false;
  if (config.addedLines === "" && config.removedLines === "") {
    config.expectedBoardLength = 0;
  }
  
  if (!inTreeEditor.value) {
    // 如果是从Tree Config进入的，需要弹出导航路径
    store.headers["tree-config"].pop();
  } else {
    // 如果是直接从菜单进入的，返回到上一个页面
    // 这里可以选择返回到一个默认页面，比如Tree Config
    store.sideView = 'tree-config';
  }
  
  inTreeEditor.value = false; // 重置标记
};

// 添加配置保存状态变量
const configSaveSuccess = ref(false);
const configLoadSuccess = ref(false);
const configSaveError = ref<string | null>(null);
const configLoadError = ref<string | null>(null);

// 保存配置文件
const handleSaveConfig = async () => {
  // 重置状态
  configSaveSuccess.value = false;
  configSaveError.value = null;
  
  try {
    // 从 store 中收集所有需要保存的配置
    const mainStore = useStore();
    const configStore = useConfigStore();
    
    // 创建包含所有需要保存的配置的对象
    const configData = {
      // 主要配置
      board: configStore.board,
      startingPot: configStore.startingPot,
      effectiveStack: configStore.effectiveStack,
      rakePercent: configStore.rakePercent,
      rakeCap: configStore.rakeCap,
      donkOption: configStore.donkOption,
      
      // 下注配置
      oopFlopBet: configStore.oopFlopBet,
      oopFlopRaise: configStore.oopFlopRaise,
      oopTurnBet: configStore.oopTurnBet,
      oopTurnRaise: configStore.oopTurnRaise,
      oopTurnDonk: configStore.oopTurnDonk,
      oopRiverBet: configStore.oopRiverBet,
      oopRiverRaise: configStore.oopRiverRaise,
      oopRiverDonk: configStore.oopRiverDonk,
      ipFlopBet: configStore.ipFlopBet,
      ipFlopRaise: configStore.ipFlopRaise,
      ipTurnBet: configStore.ipTurnBet,
      ipTurnRaise: configStore.ipTurnRaise,
      ipRiverBet: configStore.ipRiverBet,
      ipRiverRaise: configStore.ipRiverRaise,
      
      // 阈值配置
      addAllInThreshold: configStore.addAllInThreshold,
      forceAllInThreshold: configStore.forceAllInThreshold,
      mergingThreshold: configStore.mergingThreshold,
      
      // 特殊游戏树配置
      expectedBoardLength: configStore.expectedBoardLength,
      addedLines: configStore.addedLines,
      removedLines: configStore.removedLines,
      
      // 范围配置
      ranges: mainStore.ranges,
      
      // bunching 配置
      isBunchingEnabled: mainStore.isBunchingEnabled,
      bunchingFlop: mainStore.bunchingFlop,
    };
    
    // 将配置转换为 JSON 字符串
    const configJson = JSON.stringify(configData, null, 2);
    
    // 调用 Tauri 保存对话框
    const result = await showSaveConfigDialog(configJson);
    
    if (result) {
      configSaveError.value = result;
    } else {
      configSaveSuccess.value = true;
      
      // 3秒后自动清除成功状态
      setTimeout(() => {
        configSaveSuccess.value = false;
      }, 3000);
    }
  } catch (error) {
    configSaveError.value = String(error);
  }
};

// 加载配置文件
const handleLoadConfig = async () => {
  // 重置状态
  configLoadSuccess.value = false;
  configLoadError.value = null;
  
  try {
    // 调用 Tauri 加载对话框
    const result = await showLoadConfigDialog();
    
    if (typeof result === 'string') {
      configLoadError.value = result;
    } else {
      // 解析 JSON 配置
      const configData = JSON.parse(result.configData);
      
      // 获取 store 实例
      const mainStore = useStore();
      const configStore = useConfigStore();
      
      // 更新主要配置
      configStore.$patch({
        board: configData.board || [],
        startingPot: configData.startingPot || 20,
        effectiveStack: configData.effectiveStack || 100,
        rakePercent: configData.rakePercent || 0,
        rakeCap: configData.rakeCap || 0,
        donkOption: configData.donkOption || false,
        
        // 下注配置
        oopFlopBet: configData.oopFlopBet || "",
        oopFlopRaise: configData.oopFlopRaise || "",
        oopTurnBet: configData.oopTurnBet || "",
        oopTurnRaise: configData.oopTurnRaise || "",
        oopTurnDonk: configData.oopTurnDonk || "",
        oopRiverBet: configData.oopRiverBet || "",
        oopRiverRaise: configData.oopRiverRaise || "",
        oopRiverDonk: configData.oopRiverDonk || "",
        ipFlopBet: configData.ipFlopBet || "",
        ipFlopRaise: configData.ipFlopRaise || "",
        ipTurnBet: configData.ipTurnBet || "",
        ipTurnRaise: configData.ipTurnRaise || "",
        ipRiverBet: configData.ipRiverBet || "",
        ipRiverRaise: configData.ipRiverRaise || "",
        
        // 阈值配置
        addAllInThreshold: configData.addAllInThreshold || 150,
        forceAllInThreshold: configData.forceAllInThreshold || 20,
        mergingThreshold: configData.mergingThreshold || 10,
        
        // 特殊游戏树配置
        expectedBoardLength: configData.expectedBoardLength || 0,
        addedLines: configData.addedLines || "",
        removedLines: configData.removedLines || "",
      });
      
      // 更新范围配置
      if (configData.ranges && Array.isArray(configData.ranges)) {
        mainStore.ranges = configData.ranges;
        
        // 更新 Rust 端的范围数据
        try {
          // 确定使用哪个索引来获取OOP和IP的范围
          const oopIndex = configData.board.length >= 3 ? 2 : 0;
          const ipIndex = configData.board.length >= 3 ? 3 : 1;
          
          // 使用更简单可靠的方法 - 直接使用字符串表示
          console.log("正在更新OOP范围...");
          
          // 构建OOP范围字符串 (更可靠的方法)
          if (configData.ranges[oopIndex] && configData.ranges[oopIndex].length === 13*13) {
            // 先清除现有的范围数据
            await invokes.rangeClear(0);
            
            // 从一维数组构建范围字符串
            const oopWeights = configData.ranges[oopIndex];
            let oopRangeStr = "";
            
            // 使用正确的映射关系来构建范围字符串
            const ranks = "AKQJT98765432"; // 注意这里是倒序的！
            
            // 遍历所有可能的牌型组合
            for (let i = 0; i < 13; i++) {
              for (let j = 0; j < 13; j++) {
                const index = i * 13 + j;
                const weight = oopWeights[index];
                
                // 只添加权重大于0的组合
                if (weight > 0) {
                  const rank1 = ranks[i];
                  const rank2 = ranks[j];
                  
                  // 判断是对子、同花还是异花
                  let combo;
                  if (i === j) {
                    combo = `${rank1}${rank2}`; // 对子
                  } else if (i < j) {
                    combo = `${rank1}${rank2}o`; // 异花 (注意这里与标准约定可能不同)
                  } else {
                    combo = `${rank1}${rank2}s`; // 同花
                  }
                  
                  // 添加到范围字符串
                  if (weight < 100) {
                    oopRangeStr += `${combo}:${weight/100},`;
                  } else {
                    oopRangeStr += `${combo},`;
                  }
                }
              }
            }
            
            // 移除最后一个逗号
            if (oopRangeStr.endsWith(",")) {
              oopRangeStr = oopRangeStr.substring(0, oopRangeStr.length - 1);
            }
            
            console.log("OOP范围字符串:", oopRangeStr);
            
            // 如果没有可用范围，添加一个默认的
            if (oopRangeStr === "") {
              oopRangeStr = "AA";  // 至少添加一手牌
              console.log("警告: OOP范围为空，添加默认范围 AA");
            }
            
            // 使用字符串更新OOP范围
            const oopResult = await invokes.rangeFromString(0, oopRangeStr);
            if (oopResult) {
              console.warn("更新OOP范围警告:", oopResult);
              configLoadError.value = `OOP范围更新失败: ${oopResult}`;
              return;
            }
            
            // 验证更新是否成功
            const oopCombos = await invokes.rangeNumCombos(0);
            console.log(`OOP范围更新完成，组合数: ${oopCombos}`);
            
            if (oopCombos === 0) {
              console.error("OOP范围更新失败，没有有效组合");
              configLoadError.value = "OOP范围更新失败，无法构建树";
              return;
            }
          } else {
            console.warn("OOP范围数据无效或不存在");
          }
          
          // 对IP执行相同的操作
          console.log("正在更新IP范围...");
          
          if (configData.ranges[ipIndex] && configData.ranges[ipIndex].length === 13*13) {
            // 先清除现有的范围数据
            await invokes.rangeClear(1);
            
            // 从一维数组构建范围字符串  
            const ipWeights = configData.ranges[ipIndex];
            let ipRangeStr = "";
            
            // 使用正确的映射关系来构建范围字符串
            const ranks = "AKQJT98765432"; // 注意这里是倒序的！
            
            // 遍历所有可能的牌型组合
            for (let i = 0; i < 13; i++) {
              for (let j = 0; j < 13; j++) {
                const index = i * 13 + j;
                const weight = ipWeights[index];
                
                // 只添加权重大于0的组合
                if (weight > 0) {
                  const rank1 = ranks[i];
                  const rank2 = ranks[j];
                  
                  // 判断是对子、同花还是异花
                  let combo;
                  if (i === j) {
                    combo = `${rank1}${rank2}`; // 对子
                  } else if (i < j) {
                    combo = `${rank1}${rank2}o`; // 异花 (注意这里与标准约定可能不同)
                  } else {
                    combo = `${rank1}${rank2}s`; // 同花
                  }
                  
                  // 添加到范围字符串
                  if (weight < 100) {
                    ipRangeStr += `${combo}:${weight/100},`;
                  } else {
                    ipRangeStr += `${combo},`;
                  }
                }
              }
            }
            
            // 移除最后一个逗号
            if (ipRangeStr.endsWith(",")) {
              ipRangeStr = ipRangeStr.substring(0, ipRangeStr.length - 1);
            }
            
            console.log("IP范围字符串:", ipRangeStr);
            
            // 如果没有可用范围，添加一个默认的
            if (ipRangeStr === "") {
              ipRangeStr = "AA";  // 至少添加一手牌
              console.log("警告: IP范围为空，添加默认范围 AA");
            }
            
            // 使用字符串更新IP范围
            const ipResult = await invokes.rangeFromString(1, ipRangeStr);
            if (ipResult) {
              console.warn("更新IP范围警告:", ipResult);
              configLoadError.value = `IP范围更新失败: ${ipResult}`;
              return;
            }
            
            // 验证更新是否成功
            const ipCombos = await invokes.rangeNumCombos(1);
            console.log(`IP范围更新完成，组合数: ${ipCombos}`);
            
            if (ipCombos === 0) {
              console.error("IP范围更新失败，没有有效组合");
              configLoadError.value = "IP范围更新失败，无法构建树";
              return;
            }
          } else {
            console.warn("IP范围数据无效或不存在");
          }
          
        } catch (error) {
          console.error("更新范围数据失败:", error);
          configLoadError.value = `更新范围数据失败: ${error}`;
          return; // 出错时提前返回
        }
      }
      
      // 更新 bunching 配置
      if (configData.isBunchingEnabled !== undefined) {
        mainStore.isBunchingEnabled = configData.isBunchingEnabled;
      }
      if (configData.bunchingFlop && Array.isArray(configData.bunchingFlop)) {
        mainStore.bunchingFlop = configData.bunchingFlop;
      }
      
      configLoadSuccess.value = true;
      
      // 尝试自动构建游戏树
      try {
        // 检查RunSolver组件是否已存在
        const runSolverInstance = document.querySelector('.solver-container');
        if (runSolverInstance) {
          console.log("即将自动构建游戏树...");
          
          // 如果不在RunSolver视图，切换到RunSolver视图
          if (store.sideView !== 'run-solver') {
            store.sideView = 'run-solver';
          }
          
          // 给UI一点时间来更新
          setTimeout(async () => {
            // 创建一个自定义事件，让RunSolver组件知道应该构建树
            const buildTreeEvent = new CustomEvent('auto-build-tree', {
              detail: { source: 'config-load' }
            });
            document.dispatchEvent(buildTreeEvent);
            
            console.log("已触发自动构建树事件");
          }, 500);
        }
      } catch (e) {
        console.error("自动构建树失败:", e);
      }
      
      // 3秒后自动清除成功状态
      setTimeout(() => {
        configLoadSuccess.value = false;
      }, 3000);
    }
  } catch (error) {
    configLoadError.value = String(error);
  }
};

// 保存二进制文件
const handleSaveBin = async () => {
  if (!store.canSaveSolution) return;
  
  store.isSaving = true;
  store.lastSaveSuccess = false;
  store.lastSaveError = null;
  
  try {
    // 为保存文件准备描述信息
    const boardInfo = config.board.map(card => {
      const rank = Math.floor(card / 4);
      const suit = card % 4;
      const ranks = "23456789TJQKA";
      const suits = "cdhs";
      return ranks[rank] + suits[suit];
    }).join("");
    
    const memo = `Board: ${boardInfo} | Pot: ${config.startingPot} | Stack: ${config.effectiveStack}`;
    const result = await showSaveDialog(memo);
    
    if (result) {
      store.lastSaveError = result;
    } else {
      store.lastSaveSuccess = true;
      
      // 3秒后自动清除成功状态
      setTimeout(() => {
        store.lastSaveSuccess = false;
      }, 3000);
    }
  } catch (error) {
    store.lastSaveError = String(error);
  } finally {
    store.isSaving = false;
  }
};

// 加载二进制文件
const handleLoadBin = async () => {
  if (!store.canLoadSolution) return;
  
  store.isLoading = true;
  store.lastLoadSuccess = false;
  store.lastLoadError = null;
  
  try {
    const result = await showLoadDialog();
    
    if (result) {
      store.lastLoadError = result;
    } else {
      // 加载成功后获取游戏配置信息
      try {
        const configInfo = await invokes.gameGetConfigInfo();
        
        // 打印调试信息，确认从二进制文件加载的配置信息正确
        console.log("从二进制文件加载的游戏配置:", {
          board: configInfo.board.map(card => card),
          startingPot: configInfo.starting_pot,
          effectiveStack: configInfo.effective_stack
        });
        
        // 更新 ConfigStore 中的配置
        config.board = configInfo.board;
        config.startingPot = configInfo.starting_pot;
        config.effectiveStack = configInfo.effective_stack;
        
        // 保证回到根节点
        await invokes.gameApplyHistory([]);
        
        // 设置为已完成求解状态
        store.isSolverFinished = true;
        
        // 触发数据刷新 - 这是关键！告诉 ResultViewer 重新加载数据
        store.triggerDataRefresh();
        
        // 加载成功后自动切换到结果页面
        store.navView = "results";
        
        store.lastLoadSuccess = true;
        
        // 3秒后自动清除成功状态
        setTimeout(() => {
          store.lastLoadSuccess = false;
        }, 3000);
      } catch (configError) {
        console.error("Failed to get game config info:", configError);
        store.lastLoadError = `加载成功但获取配置失败: ${configError}`;
      }
    }
  } catch (error) {
    store.lastLoadError = String(error);
  } finally {
    store.isLoading = false;
  }
};

// 修改状态信息获取函数
const getStatusMessage = () => {
  if (store.lastSaveSuccess) return "保存方案成功！";
  if (store.lastLoadSuccess) return "加载方案成功！";
  if (configSaveSuccess.value) return "保存配置成功！";
  if (configLoadSuccess.value) return "加载配置成功！";
  if (store.lastSaveError) return `保存方案失败: ${store.lastSaveError}`;
  if (store.lastLoadError) return `加载方案失败: ${store.lastLoadError}`;
  if (configSaveError.value) return `保存配置失败: ${configSaveError.value}`;
  if (configLoadError.value) return `加载配置失败: ${configLoadError.value}`;
  return "";
};

// 生命周期方法
onMounted(() => {
  updateClientHeight();
  window.addEventListener("resize", updateClientHeight);
});
</script>

<style scoped>
.file-operations {
  max-width: 800px;
  margin: 0 auto;
}

.section-title {
  border-bottom: 2px solid #e5e7eb;
  padding-bottom: 1rem;
}
</style>
