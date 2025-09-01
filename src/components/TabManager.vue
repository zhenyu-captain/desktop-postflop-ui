<template>
  <div class="tab-manager">
    <!-- 标签页导航栏 -->
    <div class="tab-nav bg-gray-100 border-b border-gray-300 px-4 py-2 flex items-center overflow-x-auto">
      <div class="flex space-x-2 flex-shrink-0">
        <div
          v-for="tab in tabs"
          :key="tab.id"
          :class="[
            'tab-item flex items-center px-4 py-2 rounded-t-lg border border-b-0 cursor-pointer transition-all duration-200 min-w-0',
            activeTabId === tab.id 
              ? 'bg-white border-gray-300 text-blue-600 shadow-sm' 
              : 'bg-gray-50 border-gray-200 text-gray-600 hover:bg-gray-100'
          ]"
          @click="switchTab(tab.id)"
          @dblclick="startRename(tab.id)"
        >
          <span class="flex-shrink-0 mr-2 text-sm">📊</span>
          
          <!-- 编辑模式：输入框 -->
          <input
            v-if="editingTabId === tab.id"
            v-model="editingName"
            @keydown.enter="finishRename"
            @keydown.esc="cancelRename"
            @blur="finishRename"
            class="flex-1 min-w-0 px-1 py-0 text-sm font-medium bg-transparent border-none outline-none focus:ring-1 focus:ring-blue-500 rounded"
            :maxlength="40"
            ref="renameInput"
          />
          
          <!-- 正常模式：显示名称 -->
          <span v-else class="truncate max-w-32 text-sm font-medium">{{ tab.name }}</span>
          
          <button
            v-if="tabs.length > 1"
            @click.stop="closeTab(tab.id)"
            class="ml-2 flex-shrink-0 w-4 h-4 rounded-full hover:bg-gray-200 dark:hover:bg-gray-600 flex items-center justify-center transition-colors"
          >
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </div>
      
      <!-- 新建标签页按钮 -->
      <button
        @click="createNewTab"
        class="ml-4 flex-shrink-0 w-8 h-8 rounded-lg bg-blue-500 text-white hover:bg-blue-600 flex items-center justify-center transition-colors"
        title="新建标签页"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
      </button>
      
      <!-- 标签页操作菜单 -->
      <div class="ml-auto flex items-center space-x-2">
        <span class="text-sm text-gray-500">{{ tabs.length }} 个标签页</span>
        <button
          @click="showTabMenu = !showTabMenu"
          class="w-6 h-6 rounded hover:bg-gray-200 flex items-center justify-center transition-colors relative"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z" />
          </svg>
          
          <!-- 下拉菜单 -->
          <div
            v-if="showTabMenu"
            class="absolute top-full right-0 mt-1 w-48 bg-white border border-gray-200 rounded-lg shadow-lg z-50"
            @click.stop
          >
            <button
              @click="duplicateCurrentTab"
              class="w-full px-4 py-2 text-left text-sm hover:bg-gray-50 flex items-center"
            >
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
              </svg>
              复制当前标签页
            </button>
            <button
              @click="closeOtherTabs"
              class="w-full px-4 py-2 text-left text-sm hover:bg-gray-50 flex items-center"
              :disabled="tabs.length <= 1"
            >
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
              关闭其他标签页
            </button>
            <button
              @click="closeAllTabs"
              class="w-full px-4 py-2 text-left text-sm hover:bg-gray-50 flex items-center text-red-600"
            >
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
              关闭所有标签页
            </button>
          </div>
        </button>
      </div>
    </div>



    <!-- 标签页内容区域 -->
    <div class="tab-content flex-1 relative">
      <div
        v-for="tab in tabs"
        :key="tab.id"
        v-show="activeTabId === tab.id"
        class="tab-panel absolute inset-0"
      >
        <slot :tab="tab" :isActive="activeTabId === tab.id" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';

// 标签页接口定义
export interface TabData {
  id: string;
  name: string;
  created: Date;
  lastAccessed: Date;
  // 标签页特有的状态数据
  navigationState?: any;
  decisionPath?: any;
  selectedSpot?: any;
  selectedChance?: any;
  results?: any;
  chanceReports?: any;
}

// Props
const props = defineProps<{
  initialTabs?: TabData[];
}>();

// Emits
const emit = defineEmits<{
  (event: 'tab-switched', tabId: string, tab: TabData): void;
  (event: 'tab-created', tabId: string, tab: TabData): void;
  (event: 'tab-closed', tabId: string): void;
  (event: 'tab-duplicated', originalTabId: string, newTabId: string): void;
  (event: 'tab-renamed', tabId: string, newName: string): void;
}>();

// 状态管理
const tabs = ref<TabData[]>([]);
const activeTabId = ref<string>('');
const showTabMenu = ref(false);
const nextTabId = ref(1);

// 重命名相关状态
const editingTabId = ref<string>('');
const editingName = ref<string>('');

// 计算属性
const activeTab = computed(() => tabs.value.find(tab => tab.id === activeTabId.value));

// 初始化
onMounted(() => {
  if (props.initialTabs && props.initialTabs.length > 0) {
    tabs.value = [...props.initialTabs];
    activeTabId.value = tabs.value[0].id;
  } else {
    createNewTab();
  }
  
  // 点击外部关闭菜单
  document.addEventListener('click', handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside);
});

// 方法
const generateTabId = (): string => {
  return `tab-${nextTabId.value++}`;
};

const generateTabName = (): string => {
  const count = tabs.value.length + 1;
  return `决策路径 ${count}`;
};

const createNewTab = (): string => {
  const newTab: TabData = {
    id: generateTabId(),
    name: generateTabName(),
    created: new Date(),
    lastAccessed: new Date(),
    navigationState: null,
    decisionPath: null,
    selectedSpot: null,
    selectedChance: null,
    results: null,
    chanceReports: null,
  };
  
  tabs.value.push(newTab);
  activeTabId.value = newTab.id;
  
  emit('tab-created', newTab.id, newTab);
  return newTab.id;
};

const switchTab = (tabId: string) => {
  if (tabId === activeTabId.value) return;
  
  const tab = tabs.value.find(t => t.id === tabId);
  if (tab) {
    tab.lastAccessed = new Date();
    activeTabId.value = tabId;
    emit('tab-switched', tabId, tab);
  }
};

const closeTab = (tabId: string) => {
  const index = tabs.value.findIndex(tab => tab.id === tabId);
  if (index === -1) return;
  
  // 如果只有一个标签页，不允许关闭
  if (tabs.value.length === 1) {
    return;
  }
  
  // 如果关闭的是当前活跃标签页，需要切换到其他标签页
  if (tabId === activeTabId.value) {
    const newActiveIndex = index > 0 ? index - 1 : index + 1;
    if (newActiveIndex < tabs.value.length) {
      activeTabId.value = tabs.value[newActiveIndex].id;
    }
  }
  
  tabs.value.splice(index, 1);
  emit('tab-closed', tabId);
};

const duplicateCurrentTab = () => {
  const currentTab = activeTab.value;
  if (!currentTab) return;
  
  const newTab: TabData = {
    id: generateTabId(),
    name: `${currentTab.name} (副本)`,
    created: new Date(),
    lastAccessed: new Date(),
    navigationState: currentTab.navigationState ? JSON.parse(JSON.stringify(currentTab.navigationState)) : null,
    decisionPath: currentTab.decisionPath ? JSON.parse(JSON.stringify(currentTab.decisionPath)) : null,
    selectedSpot: currentTab.selectedSpot ? JSON.parse(JSON.stringify(currentTab.selectedSpot)) : null,
    selectedChance: currentTab.selectedChance ? JSON.parse(JSON.stringify(currentTab.selectedChance)) : null,
    results: currentTab.results ? JSON.parse(JSON.stringify(currentTab.results)) : null,
    chanceReports: currentTab.chanceReports ? JSON.parse(JSON.stringify(currentTab.chanceReports)) : null,
  };
  
  tabs.value.push(newTab);
  activeTabId.value = newTab.id;
  
  emit('tab-duplicated', currentTab.id, newTab.id);
  showTabMenu.value = false;
};

const closeOtherTabs = () => {
  if (tabs.value.length <= 1) return;
  
  const currentTab = activeTab.value;
  if (!currentTab) return;
  
  const tabsToClose = tabs.value.filter(tab => tab.id !== currentTab.id);
  tabs.value = [currentTab];
  
  tabsToClose.forEach(tab => {
    emit('tab-closed', tab.id);
  });
  
  showTabMenu.value = false;
};

const closeAllTabs = () => {
  const tabsToClose = [...tabs.value];
  tabs.value = [];
  
  tabsToClose.forEach(tab => {
    emit('tab-closed', tab.id);
  });
  
  // 创建新的默认标签页
  createNewTab();
  showTabMenu.value = false;
};

const handleClickOutside = (event: Event) => {
  const target = event.target as HTMLElement;
  if (!target.closest('.tab-manager')) {
    showTabMenu.value = false;
  }
};

// 更新标签页数据的方法
const updateTabData = (tabId: string, data: Partial<TabData>) => {
  const tab = tabs.value.find(t => t.id === tabId);
  if (tab) {
    Object.assign(tab, data);
    tab.lastAccessed = new Date();
  }
};

// 获取标签页数据的方法
const getTabData = (tabId: string): TabData | undefined => {
  return tabs.value.find(t => t.id === tabId);
};

// 重命名相关方法
const startRename = (tabId: string) => {
  const tab = tabs.value.find(t => t.id === tabId);
  if (!tab) return;
  
  editingTabId.value = tabId;
  editingName.value = tab.name;
  
  // 下一个 tick 后聚焦输入框
  nextTick(() => {
    const input = document.querySelector(`[data-tab-id="${tabId}"] input`) as HTMLInputElement;
    if (input) {
      input.focus();
      input.select();
    }
  });
};

const finishRename = () => {
  if (!editingTabId.value) return;
  
  const newName = editingName.value.trim();
  const tab = tabs.value.find(t => t.id === editingTabId.value);
  
  if (tab && newName && newName !== tab.name) {
    // 检查名称长度
    if (newName.length > 40) {
      editingName.value = newName.substring(0, 40);
      return;
    }
    
    // 更新标签页名称
    tab.name = newName;
    tab.lastAccessed = new Date();
    
    // 触发重命名事件
    emit('tab-renamed', editingTabId.value, newName);
    
    console.log(`Tab renamed: ${editingTabId.value} -> "${newName}"`);
  }
  
  // 退出编辑模式
  editingTabId.value = '';
  editingName.value = '';
};

const cancelRename = () => {
  editingTabId.value = '';
  editingName.value = '';
};



// 重命名标签页的公共方法
const renameTab = (tabId: string, newName: string) => {
  const tab = tabs.value.find(t => t.id === tabId);
  if (!tab) return false;
  
  const trimmedName = newName.trim();
  if (!trimmedName || trimmedName.length > 40) return false;
  
  tab.name = trimmedName;
  tab.lastAccessed = new Date();
  emit('tab-renamed', tabId, trimmedName);
  
  return true;
};

// 暴露给父组件的方法
defineExpose({
  createNewTab,
  switchTab,
  closeTab,
  updateTabData,
  getTabData,
  activeTab,
  tabs,
});
</script>

<style scoped>
.tab-manager {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.tab-nav {
  flex-shrink: 0;
  scrollbar-width: thin;
  scrollbar-color: rgba(156, 163, 175, 0.5) transparent;
}

.tab-nav::-webkit-scrollbar {
  height: 4px;
}

.tab-nav::-webkit-scrollbar-track {
  background: transparent;
}

.tab-nav::-webkit-scrollbar-thumb {
  background-color: rgba(156, 163, 175, 0.5);
  border-radius: 2px;
}

.tab-item {
  max-width: 200px;
  position: relative;
}

.tab-content {
  flex: 1;
  overflow: hidden;
}

.tab-panel {
  overflow: hidden;
}

/* 动画效果 */
.tab-item {
  transition: all 0.2s ease-in-out;
}

.tab-item:hover {
  transform: translateY(-1px);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .tab-item {
    min-width: 120px;
    max-width: 150px;
  }

  .tab-item span {
    font-size: 0.8rem;
  }
}



/* 输入框样式改进 */
.tab-item input {
  background-color: rgba(255, 255, 255, 0.9);
  border-radius: 4px;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}

.tab-item input:focus {
  background-color: white;
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.5);
}
</style>
