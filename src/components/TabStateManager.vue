<template>
  <!-- 这是一个无UI的状态管理组件 -->
  <div style="display: none;"></div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue';
import * as invokes from '../invokes';
import { Spot, SpotChance, Results, ChanceReports } from '../result-types';

// Props
const props = defineProps<{
  tabId: string;
  isActive: boolean;
  savedState?: any;
}>();

// Emits
const emit = defineEmits<{
  (event: 'state-changed', tabId: string, state: any): void;
  (event: 'request-restoration', tabId: string): void;
}>();

// 内部状态
const gameHistory = ref<number[]>([]);
const isInitialized = ref(false);

// 监听活跃状态变化
watch(() => props.isActive, async (newActive, oldActive) => {
  if (newActive && !oldActive) {
    // 标签页变为活跃状态，恢复其状态
    await restoreTabState();
  } else if (!newActive && oldActive) {
    // 标签页变为非活跃状态，保存其状态
    await saveTabState();
  }
});

// 保存标签页状态
const saveTabState = async () => {
  try {
    // 这里我们可以通过后端API获取当前的游戏状态
    // 暂时使用一个简化的保存机制
    const currentState = {
      gameHistory: gameHistory.value,
      timestamp: new Date(),
      tabId: props.tabId,
    };
    
    emit('state-changed', props.tabId, currentState);
    console.log(`Saved state for tab ${props.tabId}:`, currentState);
  } catch (error) {
    console.error(`Error saving state for tab ${props.tabId}:`, error);
  }
};

// 恢复标签页状态
const restoreTabState = async () => {
  try {
    if (props.savedState?.gameHistory) {
      gameHistory.value = props.savedState.gameHistory;
      // 应用游戏历史
      await invokes.gameApplyHistory(gameHistory.value);
      console.log(`Restored state for tab ${props.tabId}:`, props.savedState);
    } else {
      // 重置到初始状态
      await invokes.gameApplyHistory([]);
      gameHistory.value = [];
    }
    
    isInitialized.value = true;
    emit('request-restoration', props.tabId);
  } catch (error) {
    console.error(`Error restoring state for tab ${props.tabId}:`, error);
  }
};

// 更新游戏历史
const updateGameHistory = (history: number[]) => {
  gameHistory.value = [...history];
};

// 暴露方法给父组件
defineExpose({
  saveTabState,
  restoreTabState,
  updateGameHistory,
});

// 组件挂载时的初始化
onMounted(async () => {
  if (props.isActive) {
    await restoreTabState();
  }
});

// 组件卸载时保存状态
onUnmounted(async () => {
  if (props.isActive) {
    await saveTabState();
  }
});
</script>
