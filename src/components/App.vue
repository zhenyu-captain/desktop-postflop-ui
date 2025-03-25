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
import { computed, ref } from "vue";
import { useStore, useConfigStore } from "../store";

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
    return "Tree Configuration > Tree Preview & Edit";
  }
  return store.headers[store.sideView].join(" > ");
});

// 添加菜单项配置 - 添加了tree-editor选项在tree-config后面
const menuItems = [
  { view: 'oop-range', label: 'OOP Range' },
  { view: 'ip-range', label: 'IP Range' },
  { view: 'board', label: 'Board' },
  { view: 'tree-config', label: 'Tree Config' },
  { view: 'tree-editor', label: 'Tree Editor' },
  { view: 'bunching', label: 'Bunching' },
  { view: 'run-solver', label: 'Run Solver' },
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
  // 只有当前在tree-config页面或者已经在tree editor模式下，才能使用tree editor
  return store.sideView === 'tree-config' || store.treeEditMode;
});

const toggleTreeEditor = () => {
  if (store.treeEditMode) {
    // 退出tree editor模式
    cancelTreeEdit();
  } else if (canToggleTreeEditor.value) {
    // 进入tree editor模式
    if (config.expectedBoardLength === 0) {
      config.expectedBoardLength = Math.max(config.board.length, 3);
    }
    store.treeEditMode = true;
    if (!inTreeEditor.value) {
      // 如果不是从Tree Editor菜单进入，则是从Tree Config进入
      store.headers["tree-config"].push("Tree Preview & Edit");
    }
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
    // 只有从Tree Config进入时才需要弹出
    store.headers["tree-config"].pop();
  }
  inTreeEditor.value = false; // 重置标记
};

updateClientHeight();
window.addEventListener("resize", updateClientHeight);
</script>
