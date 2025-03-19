<template>
  <div class="min-w-[1080px] bg-white" :style="{ height: clientHeight + 'px' }">
    <!-- 顶部导航栏 - 使用现代UI设计 -->
    <nav class="flex w-full h-12 shadow-lg bg-white text-slate-800 border-b border-gray-200">
      <div class="flex items-center px-4">
        <span class="text-lg font-semibold">Boos SaaS</span>
        <span class="text-sm ml-2 text-gray-500">v0.2.7</span>
      </div>
      
      <div class="flex items-center space-x-1 ml-8">
        <button 
          v-for="item in menuItems" 
          :key="item.view"
          :class="[
            'px-4 h-full transition-colors hover:bg-gray-100',
            store.sideView === item.view ? 'bg-gray-100 text-blue-600' : ''
          ]"
          @click="store.sideView = item.view"
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

        <!-- 保留旧版本的组件结构和条件渲染 -->
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

    <!-- 结果页面 - 使用现代UI样式 -->
    <div v-show="store.navView === 'results'" class="w-full h-[calc(100%-3rem)] overflow-hidden">
      <ResultViewer />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useStore } from "../store";

import RangeEditor from "./RangeEditor.vue";
import BoardSelector from "./BoardSelector.vue";
import TreeConfig from "./TreeConfig.vue";
import BunchingEffect from "./BunchingEffect.vue";
import RunSolver from "./RunSolver.vue";
import AboutPage from "./AboutPage.vue";
import ResultViewer from "./ResultViewer.vue";

const store = useStore();
const header = computed(() => store.headers[store.sideView].join(" > "));

// 添加菜单项配置 - 来自新UI设计
const menuItems = [
  { view: 'oop-range', label: 'OOP Range' },
  { view: 'ip-range', label: 'IP Range' },
  { view: 'board', label: 'Board' },
  { view: 'tree-config', label: 'Tree Config' },
  { view: 'bunching', label: 'Bunching' },
  { view: 'run-solver', label: 'Run Solver' },
  { view: 'about', label: 'About' }
];

const clientHeight = ref(0);
const updateClientHeight = () => {
  clientHeight.value = document.documentElement.clientHeight - 0.01;
};

updateClientHeight();
window.addEventListener("resize", updateClientHeight);
</script>
