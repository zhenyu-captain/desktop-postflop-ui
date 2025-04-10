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
                <p class="text-gray-600 mt-1">保存和加载求解方案</p>
              </div>
              
              <div class="flex flex-col space-y-6">
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
              <div v-if="store.lastSaveSuccess || store.lastLoadSuccess || store.lastSaveError || store.lastLoadError" 
                   class="mt-6 p-4 rounded-lg" 
                   :class="{
                     'bg-green-100 border-l-4 border-green-500': store.lastSaveSuccess || store.lastLoadSuccess,
                     'bg-red-100 border-l-4 border-red-500': store.lastSaveError || store.lastLoadError
                   }">
                <div class="flex">
                  <div class="flex-shrink-0">
                    <svg v-if="store.lastSaveSuccess || store.lastLoadSuccess" class="h-5 w-5 text-green-500" viewBox="0 0 20 20" fill="currentColor">
                      <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                    </svg>
                    <svg v-else class="h-5 w-5 text-red-500" viewBox="0 0 20 20" fill="currentColor">
                      <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
                    </svg>
                  </div>
                  <div class="ml-3">
                    <p class="text-sm leading-5 font-medium" 
                       :class="{
                         'text-green-800': store.lastSaveSuccess || store.lastLoadSuccess,
                         'text-red-800': store.lastSaveError || store.lastLoadError
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
import { showSaveDialog, showLoadDialog } from "../invokes";

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
      store.lastLoadSuccess = true;
      
      // 设置为已完成求解状态
      store.isSolverFinished = true;
      
      // 触发数据刷新 - 这是关键！告诉 ResultViewer 重新加载数据
      store.triggerDataRefresh();
      
      // 3秒后自动清除成功状态
      setTimeout(() => {
        store.lastLoadSuccess = false;
      }, 3000);
    }
  } catch (error) {
    store.lastLoadError = String(error);
  } finally {
    store.isLoading = false;
  }
};

// 获取状态消息
const getStatusMessage = () => {
  if (store.lastSaveSuccess) return "保存成功！";
  if (store.lastLoadSuccess) return "加载成功！";
  if (store.lastSaveError) return `保存失败: ${store.lastSaveError}`;
  if (store.lastLoadError) return `加载失败: ${store.lastLoadError}`;
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
