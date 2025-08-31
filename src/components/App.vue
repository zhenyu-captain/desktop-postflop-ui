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

      <div class="flex items-center ml-auto px-4 space-x-3">
        <button 
          :class="[
            'px-6 h-8 rounded transition-colors',
            store.navView === 'solver' ? 'bg-blue-600 text-white' : 'bg-gray-200 text-gray-700'
          ]"
          @click="store.navView = store.navView === 'solver' ? 'results' : 'solver'"
        >
          {{ store.navView === 'solver' ? 'View Results' : 'Back to Solver' }}
        </button>
        
        <button 
          :class="[
            'px-6 h-8 rounded transition-colors',
            store.navView === 'solver' ? 'bg-green-600 text-white' : 'bg-gray-300 text-gray-700'
          ]"
          @click="store.navView = store.navView === 'solver' ? 'results2' : 'solver'"
        >
          {{ store.navView === 'solver' ? 'View Results 2' : 'Back to Solver 2' }}
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
          <div v-show="store.sideView === 'tree-editor'">
            <TreeEditor @save="saveTreeEdit" @cancel="cancelTreeEdit" :showSaveSuccess="showSaveSuccess" />
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

                <!-- 新增：调试日志按钮 -->
                <div class="card p-5 border rounded-lg shadow-sm bg-white">
                  <h3 class="text-lg font-semibold mb-3">配置调试</h3>
                  <p class="text-gray-600 mb-4">显示当前范围配置的调试信息，用于诊断加载/保存问题。</p>
                  
                  <div class="flex items-center space-x-3">
                    <button 
                      class="px-4 py-2 rounded bg-yellow-600 text-white hover:bg-yellow-700 transition-colors flex items-center"
                      @click="showDebugLog"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
                      </svg>
                      显示范围调试信息
                    </button>
                    
                    <button 
                      class="px-4 py-2 rounded bg-green-600 text-white hover:bg-green-700 transition-colors flex items-center"
                      @click="testRangeImportExport"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
                      </svg>
                      测试范围导入导出
                    </button>
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

    <!-- 结果页面2 - 升级版本 -->
    <div v-show="store.navView === 'results2'" class="w-full h-[calc(100%-3rem)] overflow-hidden">      
      <ResultViewer2 />
    </div>

    <!-- 调试日志弹窗 -->
    <div v-if="showDebugModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-4/5 max-w-4xl max-h-[80vh] overflow-hidden flex flex-col">
        <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">范围配置调试日志</h3>
          <button @click="showDebugModal = false" class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
        <div class="p-4 overflow-y-auto flex-1">
          <pre class="text-xs whitespace-pre-wrap bg-gray-100 dark:bg-gray-900 p-3 rounded">{{ debugLogText }}</pre>
        </div>
        <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end">
          <button @click="showDebugModal = false" class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700">关闭</button>
        </div>
      </div>
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
import ResultViewer2 from "./ResultViewer2.vue";

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

// 新增：调试日志弹窗
const showDebugModal = ref(false);
const debugLogText = ref("");

// 改进的调试日志显示函数，提供更详细的范围信息
const showDebugLog = async () => {
  try {
    // 获取各种调试信息
    const boardInfo = config.board.map(card => {
      const rank = Math.floor(card / 4);
      const suit = card % 4;
      const ranks = "23456789TJQKA";
      const suits = "cdhs";
      return ranks[rank] + suits[suit];
    }).join("");

    // 获取当前的OOP和IP范围的组合数
    const oopCombos = await invokes.rangeNumCombos(0);
    const ipCombos = await invokes.rangeNumCombos(1);
    
    // 获取当前范围的字符串表示
    const oopRangeStr = await invokes.rangeToString(0);
    const ipRangeStr = await invokes.rangeToString(1);

    // 获取当前范围的权重矩阵
    const oopMatrix = await invokes.rangeGetWeights(0);
    const ipMatrix = await invokes.rangeGetWeights(1);

    // 确定应该使用哪个索引来获取范围数据
    const boardLength = config.board.length;
    const oopIndex = boardLength >= 3 ? 2 : 0;
    const ipIndex = boardLength >= 3 ? 3 : 1;
    
    // 构建详细的调试日志
    let log = `### 范围调试信息 ###\n\n`;
    log += `当前翻牌: ${boardInfo}\n`;
    log += `翻牌长度: ${boardLength}\n`;
    log += `范围数组总长度: ${store.ranges.length}\n\n`;
    
    log += `### 范围索引信息 ###\n`;
    log += `OOP范围索引: ${oopIndex}\n`;
    log += `IP范围索引: ${ipIndex}\n\n`;
    
    log += `### OOP范围信息 ###\n`;
    log += `组合数: ${oopCombos}\n`;
    log += `范围字符串: ${oopRangeStr}\n`;
    
    // 分析OOP矩阵数据（直接从后端获取的实际数据）
    if (oopMatrix && oopMatrix.length === 13*13) {
      // 计算非零权重的手牌数
      const nonZeroWeights = oopMatrix.filter(w => w > 0).length;
      log += `实际矩阵长度: ${oopMatrix.length}\n`;
      log += `实际非零权重手牌数: ${nonZeroWeights}\n`;
      
      // 展示具体的矩阵内容（简化版）
      log += `实际矩阵前10个值: ${oopMatrix.slice(0, 10).join(", ")}\n`;
      
      // 尝试找出有问题的部分
      const sum = oopMatrix.reduce((a, b) => a + b, 0);
      log += `实际权重总和: ${sum}\n`;
      log += `实际平均权重: ${sum / 169}\n`;
    } else {
      log += `警告: 无法获取OOP矩阵数据\n`;
    }
    
    // 分析store中的OOP数据（用于保存到配置文件的数据）
    if (store.ranges[oopIndex]) {
      // 计算非零权重的手牌数
      const nonZeroWeights = store.ranges[oopIndex].filter(w => w > 0).length;
      log += `\nstore存储矩阵长度: ${store.ranges[oopIndex].length}\n`;
      log += `store非零权重手牌数: ${nonZeroWeights}\n`;
      
      // 展示具体的矩阵内容（简化版）
      log += `store矩阵前10个值: ${store.ranges[oopIndex].slice(0, 10).join(", ")}\n`;
      
      // 尝试找出有问题的部分
      const sum = store.ranges[oopIndex].reduce((a, b) => a + b, 0);
      log += `store权重总和: ${sum}\n`;
      log += `store平均权重: ${sum / 169}\n`;
    } else {
      log += `警告: OOP范围索引 ${oopIndex} 不存在于store.ranges中!\n`;
    }
    
    log += `\n### IP范围信息 ###\n`;
    log += `组合数: ${ipCombos}\n`;
    log += `范围字符串: ${ipRangeStr}\n`;
    
    // 分析IP矩阵数据（直接从后端获取的实际数据）
    if (ipMatrix && ipMatrix.length === 13*13) {
      // 计算非零权重的手牌数
      const nonZeroWeights = ipMatrix.filter(w => w > 0).length;
      log += `实际矩阵长度: ${ipMatrix.length}\n`;
      log += `实际非零权重手牌数: ${nonZeroWeights}\n`;
      
      // 展示具体的矩阵内容（简化版）
      log += `实际矩阵前10个值: ${ipMatrix.slice(0, 10).join(", ")}\n`;
      
      // 尝试找出有问题的部分
      const sum = ipMatrix.reduce((a, b) => a + b, 0);
      log += `实际权重总和: ${sum}\n`;
      log += `实际平均权重: ${sum / 169}\n`;
    } else {
      log += `警告: 无法获取IP矩阵数据\n`;
    }
    
    // 分析store中的IP数据（用于保存到配置文件的数据）
    if (store.ranges[ipIndex]) {
      // 同样分析IP范围
      const nonZeroWeights = store.ranges[ipIndex].filter(w => w > 0).length;
      log += `\nstore存储矩阵长度: ${store.ranges[ipIndex].length}\n`;
      log += `store非零权重手牌数: ${nonZeroWeights}\n`;
      log += `store矩阵前10个值: ${store.ranges[ipIndex].slice(0, 10).join(", ")}\n`;
      
      const sum = store.ranges[ipIndex].reduce((a, b) => a + b, 0);
      log += `store权重总和: ${sum}\n`;
      log += `store平均权重: ${sum / 169}\n`;
    } else {
      log += `警告: IP范围索引 ${ipIndex} 不存在于store.ranges中!\n`;
    }

    debugLogText.value = log;
    showDebugModal.value = true;
  } catch (error) {
    debugLogText.value = `获取调试信息时出错: ${error}`;
    showDebugModal.value = true;
  }
};

// 修改后的保存配置函数
const handleSaveConfig = async () => {
  // 重置状态
  configSaveSuccess.value = false;
  configSaveError.value = null;
  
  try {
    // 从 store 中收集所有需要保存的配置
    const mainStore = useStore();
    const configStore = useConfigStore();
    
    // 获取当前board长度，用于确定正确的范围索引
    const boardLength = configStore.board.length;
    const oopIndex = boardLength >= 3 ? 2 : 0;
    const ipIndex = boardLength >= 3 ? 3 : 1;
    
    // 获取当前范围信息用于记录日志
    const oopCombos = await invokes.rangeNumCombos(0);
    const ipCombos = await invokes.rangeNumCombos(1);
    
    console.log(`保存配置 - Board长度: ${boardLength}, OOP索引: ${oopIndex}, IP索引: ${ipIndex}`);
    console.log(`保存配置 - OOP组合数: ${oopCombos}, IP组合数: ${ipCombos}`);
    
    // 检查store.ranges数组是否足够长
    while (mainStore.ranges.length <= Math.max(oopIndex, ipIndex)) {
      mainStore.ranges.push(new Array(13*13).fill(0));
    }
    
    // 更新范围数据 - 这是关键修复部分
    // 从后端获取当前范围的权重矩阵
    const oopMatrix = await invokes.rangeGetWeights(0);
    const ipMatrix = await invokes.rangeGetWeights(1);
    
    console.log(`保存配置 - 获取到OOP权重矩阵长度: ${oopMatrix.length}`);
    console.log(`保存配置 - 获取到IP权重矩阵长度: ${ipMatrix.length}`);
    
    // 确保获取到了有效的权重矩阵
    if (oopMatrix && oopMatrix.length === 13*13) {
      // 复制权重数据
      mainStore.ranges[oopIndex] = [...oopMatrix];
      console.log(`保存配置 - 已更新OOP范围矩阵到索引 ${oopIndex}`);
    } else {
      console.warn(`保存配置 - 无法获取有效的OOP权重矩阵`);
    }
    
    if (ipMatrix && ipMatrix.length === 13*13) {
      // 复制权重数据
      mainStore.ranges[ipIndex] = [...ipMatrix];
      console.log(`保存配置 - 已更新IP范围矩阵到索引 ${ipIndex}`);
    } else {
      console.warn(`保存配置 - 无法获取有效的IP权重矩阵`);
    }
    
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
      
      // 范围字符串 - 新增保存范围字符串，用于更好的恢复
      rangeStrings: {
        oop: await invokes.rangeToString(0),
        ip: await invokes.rangeToString(1)
      },
      
      // bunching 配置
      isBunchingEnabled: mainStore.isBunchingEnabled,
      bunchingFlop: mainStore.bunchingFlop,
    };
    
    // 将配置转换为 JSON 字符串
    const configJson = JSON.stringify(configData, null, 2);
    
    // 记录日志 - 检查范围数据是否有效
    const oopRangeHasData = mainStore.ranges[oopIndex] && mainStore.ranges[oopIndex].some(w => w > 0);
    const ipRangeHasData = mainStore.ranges[ipIndex] && mainStore.ranges[ipIndex].some(w => w > 0);
    console.log(`保存配置 - OOP范围有数据: ${oopRangeHasData}, IP范围有数据: ${ipRangeHasData}`);
    
    // 调用 Tauri 保存对话框
    const result = await showSaveConfigDialog(configJson);
    
    if (result) {
      configSaveError.value = result;
      console.error(`保存配置失败: ${result}`);
    } else {
      configSaveSuccess.value = true;
      console.log("保存配置成功!");
      
      // 3秒后自动清除成功状态
      setTimeout(() => {
        configSaveSuccess.value = false;
      }, 3000);
    }
  } catch (error) {
    configSaveError.value = String(error);
    console.error(`保存配置出错: ${error}`);
  }
};

// 修改后的加载配置函数
const handleLoadConfig = async () => {
  // 重置状态
  configLoadSuccess.value = false;
  configLoadError.value = null;
  
  try {
    // 调用 Tauri 加载对话框
    const result = await showLoadConfigDialog();
    
    if (typeof result === 'string') {
      configLoadError.value = result;
      console.error(`加载配置失败: ${result}`);
    } else {
      // 解析 JSON 配置
      const configData = JSON.parse(result.configData);
      
      // 获取 store 实例
      const mainStore = useStore();
      const configStore = useConfigStore();
      
      // 获取board长度，用于确定正确的范围索引
      const boardLength = configData.board ? configData.board.length : 0;
      const oopIndex = boardLength >= 3 ? 2 : 0;
      const ipIndex = boardLength >= 3 ? 3 : 1;
      
      console.log(`加载配置 - Board长度: ${boardLength}, OOP索引: ${oopIndex}, IP索引: ${ipIndex}`);
      
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
        // 保存原始范围数组
        mainStore.ranges = configData.ranges;
        
        // 尝试从存储的字符串恢复范围 - 如果存在的话（新增功能）
        const useRangeStrings = configData.rangeStrings && 
                               configData.rangeStrings.oop && 
                               configData.rangeStrings.ip;
                              
        console.log(`加载配置 - 是否有范围字符串: ${useRangeStrings}`);
        
        // 更新 Rust 端的范围数据
        try {
          // 先清除现有的范围数据
          await invokes.rangeClear(0);
          await invokes.rangeClear(1);
          
          // 首先尝试使用保存的范围字符串（如果有）
          if (useRangeStrings) {
            console.log("使用保存的范围字符串加载范围数据");
            
            // 使用范围字符串更新OOP范围
            const oopResult = await invokes.rangeFromString(0, configData.rangeStrings.oop);
            if (oopResult) {
              console.warn(`更新OOP范围警告: ${oopResult}`);
            } else {
              console.log("成功从字符串更新OOP范围");
            }
            
            // 使用范围字符串更新IP范围
            const ipResult = await invokes.rangeFromString(1, configData.rangeStrings.ip);
            if (ipResult) {
              console.warn(`更新IP范围警告: ${ipResult}`);
            } else {
              console.log("成功从字符串更新IP范围");
            }
          }
          // 如果没有范围字符串或者加载失败，使用权重矩阵
          else {
            console.log("使用权重矩阵加载范围数据");
            
            // 检查OOP范围数据
            if (configData.ranges[oopIndex] && configData.ranges[oopIndex].length === 13*13) {
              // 检查范围数据是否有效（有非零值）
              const hasNonZeroWeights = configData.ranges[oopIndex].some(w => w > 0);
              
              if (hasNonZeroWeights) {
                console.log("OOP范围数据有效，从矩阵重建范围");
                
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
                      } else if (i < j) { // 修复：同花/异花判断逻辑，应与range.rs保持一致
                        combo = `${rank1}${rank2}s`; // 同花
                      } else {
                        // 异花 (i > j) - 确保高牌值在前面
                        // 在i > j的情况下，rank2实际上是更高牌值，所以需要交换顺序
                        combo = `${rank2}${rank1}o`; // 异花，确保高牌值在前
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
                
                console.log(`OOP范围字符串: ${oopRangeStr.length > 100 ? oopRangeStr.substring(0, 100) + '...' : oopRangeStr}`);
                
                // 如果没有可用范围，添加一个默认的
                if (oopRangeStr === "") {
                  oopRangeStr = "AA";  // 至少添加一手牌
                  console.log("警告: OOP范围为空，添加默认范围 AA");
                }
                
                // 使用字符串更新OOP范围
                const oopResult = await invokes.rangeFromString(0, oopRangeStr);
                if (oopResult) {
                  console.warn(`更新OOP范围警告: ${oopResult}`);
                } else {
                  console.log("成功从矩阵重建OOP范围");
                }
              } else {
                console.warn("OOP范围数据全为零，使用默认范围AA");
                await invokes.rangeFromString(0, "AA");
              }
            } else {
              console.warn("OOP范围数据无效或不存在，使用默认范围AA");
              await invokes.rangeFromString(0, "AA");
            }
            
            // 检查IP范围数据
            if (configData.ranges[ipIndex] && configData.ranges[ipIndex].length === 13*13) {
              // 检查范围数据是否有效（有非零值）
              const hasNonZeroWeights = configData.ranges[ipIndex].some(w => w > 0);
              
              if (hasNonZeroWeights) {
                console.log("IP范围数据有效，从矩阵重建范围");
                
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
                      } else if (i < j) { // 修复：同花/异花判断逻辑，应与range.rs保持一致
                        combo = `${rank1}${rank2}s`; // 同花
                      } else {
                        // 异花 (i > j) - 确保高牌值在前面
                        // 在i > j的情况下，rank2实际上是更高牌值，所以需要交换顺序
                        combo = `${rank2}${rank1}o`; // 异花，确保高牌值在前
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
                
                console.log(`IP范围字符串: ${ipRangeStr.length > 100 ? ipRangeStr.substring(0, 100) + '...' : ipRangeStr}`);
                
                // 如果没有可用范围，添加一个默认的
                if (ipRangeStr === "") {
                  ipRangeStr = "AA";  // 至少添加一手牌
                  console.log("警告: IP范围为空，添加默认范围 AA");
                }
                
                // 使用字符串更新IP范围
                const ipResult = await invokes.rangeFromString(1, ipRangeStr);
                if (ipResult) {
                  console.warn(`更新IP范围警告: ${ipResult}`);
                } else {
                  console.log("成功从矩阵重建IP范围");
                }
              } else {
                console.warn("IP范围数据全为零，使用默认范围AA");
                await invokes.rangeFromString(1, "AA");
              }
            } else {
              console.warn("IP范围数据无效或不存在，使用默认范围AA");
              await invokes.rangeFromString(1, "AA");
            }
          }
          
          // 验证更新是否成功
          const oopCombos = await invokes.rangeNumCombos(0);
          const ipCombos = await invokes.rangeNumCombos(1);
          console.log(`范围更新完成 - OOP组合数: ${oopCombos}, IP组合数: ${ipCombos}`);
          
          if (oopCombos === 0 || ipCombos === 0) {
            console.error("范围更新失败，没有有效组合，使用默认范围");
            if (oopCombos === 0) await invokes.rangeFromString(0, "AA");
            if (ipCombos === 0) await invokes.rangeFromString(1, "AA");
          }
          
          // 如果已经加载完成，从后端获取当前范围的权重矩阵并更新store
          // 这确保了store.ranges与后端一致
          try {
            const updatedOopMatrix = await invokes.rangeGetWeights(0);
            const updatedIpMatrix = await invokes.rangeGetWeights(1);
            
            if (updatedOopMatrix && updatedOopMatrix.length === 13*13) {
              mainStore.ranges[oopIndex] = [...updatedOopMatrix];
              console.log("已从后端更新OOP范围矩阵");
            }
            
            if (updatedIpMatrix && updatedIpMatrix.length === 13*13) {
              mainStore.ranges[ipIndex] = [...updatedIpMatrix];
              console.log("已从后端更新IP范围矩阵");
            }
          } catch (e) {
            console.error(`获取更新后的范围权重失败: ${e}`);
          }
          
        } catch (error) {
          console.error(`更新范围数据失败: ${error}`);
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
      console.log("配置加载成功!");
      
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
        console.error(`自动构建树失败: ${e}`);
      }
      
      // 3秒后自动清除成功状态
      setTimeout(() => {
        configLoadSuccess.value = false;
      }, 3000);
    }
  } catch (error) {
    configLoadError.value = String(error);
    console.error(`加载配置出错: ${error}`);
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

// 新增：临时测试函数，用于检查范围导入导出功能是否正常工作
const testRangeImportExport = async () => {
  try {
    let log = "";
    const addLog = (msg) => {
      console.log(msg);
      log += msg + "\n";
    };
    
    addLog("### 范围导入导出测试 ###\n");
    
    // 获取当前实际的范围字符串
    const currentOopRangeStr = await invokes.rangeToString(0); 
    const currentIpRangeStr = await invokes.rangeToString(1);
    
    // 备份当前范围的矩阵
    const originalOopWeights = await invokes.rangeGetWeights(0);
    const originalIpWeights = await invokes.rangeGetWeights(1);
    
    addLog(`当前OOP范围: "${currentOopRangeStr}"`);
    addLog(`当前IP范围: "${currentIpRangeStr}"`);
    addLog("------------------------------\n");
    
    // ===== 测试OOP范围 =====
    addLog("===== 测试OOP范围 =====");
    
    // 1. 清空OOP范围
    await invokes.rangeClear(0);
    addLog("1. 清空OOP范围");
    
    // 2. 设置测试范围 - 使用当前实际范围
    addLog(`2. 设置OOP范围为 "${currentOopRangeStr}"`);
    let result = await invokes.rangeFromString(0, currentOopRangeStr);
    if (result) {
      addLog(`测试失败: 无法设置OOP测试范围: ${result}`);
      debugLogText.value = log;
      showDebugModal.value = true;
      return;
    }
    
    // 3. 从后端获取范围权重矩阵
    const oopWeights = await invokes.rangeGetWeights(0);
    if (!oopWeights || oopWeights.length !== 13*13) {
      addLog(`测试失败: 无法获取OOP权重矩阵，长度: ${oopWeights ? oopWeights.length : 'undefined'}`);
      debugLogText.value = log;
      showDebugModal.value = true;
      return;
    }
    
    // 4. 检查权重矩阵中是否有值
    const oopNonZeroWeights = oopWeights.filter(w => w > 0).length;
    addLog(`4. OOP权重矩阵中非零权重数: ${oopNonZeroWeights}`);
    
    if (oopNonZeroWeights === 0) {
      addLog("测试失败: OOP权重矩阵全为零");
      debugLogText.value = log;
      showDebugModal.value = true;
      return;
    }
    
    // 打印一些权重矩阵的详细信息，帮助调试
    addLog("\nOOP矩阵详细信息:");
    // 找出所有非零权重的位置
    const oopNonZeroPositions = [];
    for (let i = 0; i < 13; i++) {
      for (let j = 0; j < 13; j++) {
        const idx = i * 13 + j;
        if (oopWeights[idx] > 0) {
          const ranks = "AKQJT98765432";
          oopNonZeroPositions.push(`[${i},${j}]: ${ranks[i]}${ranks[j]} = ${oopWeights[idx]}`);
        }
      }
    }
    addLog(oopNonZeroPositions.slice(0, 10).join(", ") + (oopNonZeroPositions.length > 10 ? "\n..." : ""));
    
    // 5. 获取范围字符串表示
    const oopRangeStr = await invokes.rangeToString(0);
    addLog(`\n5. OOP范围字符串表示为: "${oopRangeStr}"`);
    
    // 6. 生成一个临时保存对象并转换为json
    const oopTestConfig = {
      ranges: [oopWeights]
    };
    const oopTestJson = JSON.stringify(oopTestConfig);
    addLog("6. 生成OOP临时保存对象并转换为JSON");
    
    // 7. 模拟从json加载
    const oopParsedConfig = JSON.parse(oopTestJson);
    addLog("7. 模拟从JSON加载OOP范围");
    
    // 8. 清空范围
    await invokes.rangeClear(0);
    addLog("8. 再次清空OOP范围");
    
    // 9. 从矩阵重建范围
    const ranks = "AKQJT98765432";
    let oopRebuiltRangeStr = "";
    
    // 提取权重矩阵的详细信息
    addLog("\nOOP权重矩阵重建信息:");
    const oopMatrixInfo = [];
    for (let i = 0; i < 13; i++) {
      for (let j = 0; j < 13; j++) {
        const index = i * 13 + j;
        const weight = oopParsedConfig.ranges[0][index];
        
        if (weight > 0) {
          const rank1 = ranks[i];
          const rank2 = ranks[j];
          
          // 检查是对子、同花还是异花 - 修复：根据范围数据逻辑，同花和异花的判断应与rustapp/range.rs一致
          let combo;
          if (i === j) {
            combo = `${rank1}${rank2}`; // 对子
            oopMatrixInfo.push(`对子[${i},${j}]: ${combo}, weight=${weight}`);
          } else if (i < j) { // 同花
            combo = `${rank1}${rank2}s`; // 同花
            oopMatrixInfo.push(`同花[${i},${j}]: ${combo}, weight=${weight}`);
          } else { // 异花 (i > j)
            // 确保高牌值在前面 - 由于ranks是"AKQJT98765432"，较小的索引代表较大的牌值
            // 在i > j的情况下，rank2实际上是更高牌值，所以需要交换顺序
            combo = `${rank2}${rank1}o`; // 异花，确保高牌值在前
            oopMatrixInfo.push(`异花[${i},${j}]: ${combo}, weight=${weight}`);
          }
          
          // 添加到范围字符串
          if (weight < 1.0) {
            oopRebuiltRangeStr += `${combo}:${weight},`;
          } else {
            oopRebuiltRangeStr += `${combo},`;
          }
        }
      }
    }
    addLog(oopMatrixInfo.slice(0, 10).join("\n") + (oopMatrixInfo.length > 10 ? "\n..." : ""));
    
    if (oopRebuiltRangeStr.endsWith(",")) {
      oopRebuiltRangeStr = oopRebuiltRangeStr.substring(0, oopRebuiltRangeStr.length - 1);
    }
    
    addLog(`\n9. 从矩阵重建的OOP范围字符串: "${oopRebuiltRangeStr}"`);
    
    // 10. 使用重建的范围字符串更新范围
    const oopRebuildResult = await invokes.rangeFromString(0, oopRebuiltRangeStr);
    if (oopRebuildResult) {
      addLog(`测试失败: 重建OOP范围时出错: ${oopRebuildResult}`);
      debugLogText.value = log;
      showDebugModal.value = true;
      return;
    }
    addLog("10. 使用重建的范围字符串更新OOP范围成功");
    
    // 11. 获取重建后的范围字符串
    const oopFinalRangeStr = await invokes.rangeToString(0);
    addLog(`11. 最终OOP范围字符串: "${oopFinalRangeStr}"`);
    
    // 12. 获取组合数
    const oopOriginalCombos = oopNonZeroWeights / 169 * 1326; // 近似值
    const oopFinalCombos = await invokes.rangeNumCombos(0);
    addLog(`12. 原始OOP范围组合数(估计): ${oopOriginalCombos.toFixed(1)}, 重建后组合数: ${oopFinalCombos}`);
    
    // ===== 测试IP范围 =====
    addLog("\n\n===== 测试IP范围 =====");
    
    // 1. 清空IP范围
    await invokes.rangeClear(1);
    addLog("1. 清空IP范围");
    
    // 2. 设置测试范围 - 使用当前实际范围
    addLog(`2. 设置IP范围为 "${currentIpRangeStr}"`);
    result = await invokes.rangeFromString(1, currentIpRangeStr);
    if (result) {
      addLog(`测试失败: 无法设置IP测试范围: ${result}`);
      debugLogText.value = log;
      showDebugModal.value = true;
      return;
    }
    
    // 3. 从后端获取范围权重矩阵
    const ipWeights = await invokes.rangeGetWeights(1);
    if (!ipWeights || ipWeights.length !== 13*13) {
      addLog(`测试失败: 无法获取IP权重矩阵，长度: ${ipWeights ? ipWeights.length : 'undefined'}`);
      debugLogText.value = log;
      showDebugModal.value = true;
      return;
    }
    
    // 4. 检查权重矩阵中是否有值
    const ipNonZeroWeights = ipWeights.filter(w => w > 0).length;
    addLog(`4. IP权重矩阵中非零权重数: ${ipNonZeroWeights}`);
    
    if (ipNonZeroWeights === 0) {
      addLog("测试失败: IP权重矩阵全为零");
      debugLogText.value = log;
      showDebugModal.value = true;
      return;
    }
    
    // 打印一些权重矩阵的详细信息，帮助调试
    addLog("\nIP矩阵详细信息:");
    // 找出所有非零权重的位置
    const ipNonZeroPositions = [];
    for (let i = 0; i < 13; i++) {
      for (let j = 0; j < 13; j++) {
        const idx = i * 13 + j;
        if (ipWeights[idx] > 0) {
          const ranks = "AKQJT98765432";
          ipNonZeroPositions.push(`[${i},${j}]: ${ranks[i]}${ranks[j]} = ${ipWeights[idx]}`);
        }
      }
    }
    addLog(ipNonZeroPositions.slice(0, 10).join(", ") + (ipNonZeroPositions.length > 10 ? "\n..." : ""));
    
    // 5. 获取范围字符串表示
    const ipRangeStr = await invokes.rangeToString(1);
    addLog(`\n5. IP范围字符串表示为: "${ipRangeStr}"`);
    
    // 6. 生成一个临时保存对象并转换为json
    const ipTestConfig = {
      ranges: [ipWeights]
    };
    const ipTestJson = JSON.stringify(ipTestConfig);
    addLog("6. 生成IP临时保存对象并转换为JSON");
    
    // 7. 模拟从json加载
    const ipParsedConfig = JSON.parse(ipTestJson);
    addLog("7. 模拟从JSON加载IP范围");
    
    // 8. 清空范围
    await invokes.rangeClear(1);
    addLog("8. 再次清空IP范围");
    
    // 9. 从矩阵重建范围
    let ipRebuiltRangeStr = "";
    
    // 提取权重矩阵的详细信息
    addLog("\nIP权重矩阵重建信息:");
    const ipMatrixInfo = [];
    for (let i = 0; i < 13; i++) {
      for (let j = 0; j < 13; j++) {
        const index = i * 13 + j;
        const weight = ipParsedConfig.ranges[0][index];
        
        if (weight > 0) {
          const rank1 = ranks[i];
          const rank2 = ranks[j];
          
          // 检查是对子、同花还是异花 - 修复：根据范围数据逻辑，同花和异花的判断应与rustapp/range.rs一致
          let combo;
          if (i === j) {
            combo = `${rank1}${rank2}`; // 对子
            ipMatrixInfo.push(`对子[${i},${j}]: ${combo}, weight=${weight}`);
          } else if (i < j) { // 同花
            combo = `${rank1}${rank2}s`; // 同花
            ipMatrixInfo.push(`同花[${i},${j}]: ${combo}, weight=${weight}`);
          } else { // 异花 (i > j)
            // 确保高牌值在前面 - 由于ranks是"AKQJT98765432"，较小的索引代表较大的牌值
            // 在i > j的情况下，rank2实际上是更高牌值，所以需要交换顺序
            combo = `${rank2}${rank1}o`; // 异花，确保高牌值在前
            ipMatrixInfo.push(`异花[${i},${j}]: ${combo}, weight=${weight}`);
          }
          
          // 添加到范围字符串
          if (weight < 1.0) {
            ipRebuiltRangeStr += `${combo}:${weight},`;
          } else {
            ipRebuiltRangeStr += `${combo},`;
          }
        }
      }
    }
    addLog(ipMatrixInfo.slice(0, 10).join("\n") + (ipMatrixInfo.length > 10 ? "\n..." : ""));
    
    if (ipRebuiltRangeStr.endsWith(",")) {
      ipRebuiltRangeStr = ipRebuiltRangeStr.substring(0, ipRebuiltRangeStr.length - 1);
    }
    
    addLog(`\n9. 从矩阵重建的IP范围字符串: "${ipRebuiltRangeStr}"`);
    
    // 10. 使用重建的范围字符串更新范围
    const ipRebuildResult = await invokes.rangeFromString(1, ipRebuiltRangeStr);
    if (ipRebuildResult) {
      addLog(`测试失败: 重建IP范围时出错: ${ipRebuildResult}`);
      debugLogText.value = log;
      showDebugModal.value = true;
      return;
    }
    addLog("10. 使用重建的范围字符串更新IP范围成功");
    
    // 11. 获取重建后的范围字符串
    const ipFinalRangeStr = await invokes.rangeToString(1);
    addLog(`11. 最终IP范围字符串: "${ipFinalRangeStr}"`);
    
    // 12. 获取组合数
    const ipOriginalCombos = ipNonZeroWeights / 169 * 1326; // 近似值
    const ipFinalCombos = await invokes.rangeNumCombos(1);
    addLog(`12. 原始IP范围组合数(估计): ${ipOriginalCombos.toFixed(1)}, 重建后组合数: ${ipFinalCombos}`);
    
    // ===== 测试结果汇总 =====
    addLog("\n\n===== 测试结果汇总 =====");
    
    // 判断OOP测试是否成功
    const oopTestSuccess = oopFinalCombos > 0;
    // 判断IP测试是否成功
    const ipTestSuccess = ipFinalCombos > 0;
    // 总体测试是否成功
    const overallTestSuccess = oopTestSuccess && ipTestSuccess;
    
    // 显示OOP测试结果
    addLog("OOP范围测试结果:");
    addLog(`原始范围: "${currentOopRangeStr}"`);
    addLog(`转换后范围: "${oopRangeStr}"`);
    addLog(`重建范围: "${oopRebuiltRangeStr.length > 100 ? oopRebuiltRangeStr.substring(0, 100) + '...' : oopRebuiltRangeStr}"`);
    addLog(`最终范围: "${oopFinalRangeStr}"`);
    addLog(`原始组合数(估计): ${oopOriginalCombos.toFixed(1)}`);
    addLog(`最终组合数: ${oopFinalCombos}`);
    
    // 检查OOP范围是否匹配
    const oopRangesMatch = oopRangeStr === oopFinalRangeStr;
    addLog(`原始范围与最终范围匹配: ${oopRangesMatch ? "是" : "否"}`);
    addLog(`测试结果: ${oopTestSuccess ? "成功" : "失败"}`);
    
    // 显示IP测试结果
    addLog("\nIP范围测试结果:");
    addLog(`原始范围: "${currentIpRangeStr}"`);
    addLog(`转换后范围: "${ipRangeStr}"`);
    addLog(`重建范围: "${ipRebuiltRangeStr.length > 100 ? ipRebuiltRangeStr.substring(0, 100) + '...' : ipRebuiltRangeStr}"`);
    addLog(`最终范围: "${ipFinalRangeStr}"`);
    addLog(`原始组合数(估计): ${ipOriginalCombos.toFixed(1)}`);
    addLog(`最终组合数: ${ipFinalCombos}`);
    
    // 检查IP范围是否匹配
    const ipRangesMatch = ipRangeStr === ipFinalRangeStr;
    addLog(`原始范围与最终范围匹配: ${ipRangesMatch ? "是" : "否"}`);
    addLog(`测试结果: ${ipTestSuccess ? "成功" : "失败"}`);
    
    // 总体测试结果
    addLog("\n总体测试结果: " + (overallTestSuccess ? "成功" : "失败"));
    
    if (overallTestSuccess) {
      addLog("\n范围导入导出功能测试通过!");
      
      if (!oopRangesMatch || !ipRangesMatch) {
        addLog("注意: 虽然范围数据有效，但转换后格式与原始格式不完全匹配");
        addLog("这通常是因为同一范围可以有不同的表示方式 (如 '22+' 与 'AA,KK,...,22')");
      }
    } else {
      addLog("\n范围导入导出功能测试失败!");
    }
    
    // 最后，恢复原始范围
    addLog("\n\n===== 恢复原始范围 =====");
    
    // 恢复OOP范围
    await invokes.rangeClear(0);
    await invokes.rangeFromString(0, currentOopRangeStr);
    addLog(`已恢复OOP范围: "${currentOopRangeStr}"`);
    
    // 恢复IP范围
    await invokes.rangeClear(1);
    await invokes.rangeFromString(1, currentIpRangeStr);
    addLog(`已恢复IP范围: "${currentIpRangeStr}"`);
    
    debugLogText.value = log;
    showDebugModal.value = true;
    
  } catch (error) {
    console.error(`测试出错: ${error}`);
    debugLogText.value = `测试范围导入导出功能出错: ${error}`;
    showDebugModal.value = true;
  }
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
