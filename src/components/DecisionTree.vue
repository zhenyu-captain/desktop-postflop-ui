<template>
  <div class="decision-tree-container">
    <div class="controls">
      <button @click="fetchDecisionTree(3)">获取决策树 (最大深度3)</button>
      <button @click="fetchCurrentNode">获取当前节点</button>
      <button @click="saveTreeToFile">保存树到文件</button>
      <button @click="printToRustConsole" class="console-button">打印到控制台</button>
      <span v-if="consoleStatus === 1" class="success-message">已发送到控制台!</span>
      <span v-if="consoleStatus === -1" class="error-message">发送失败</span>
    </div>
    
    <div v-if="loading" class="loading">加载中...</div>
    
    <div v-if="error" class="error">{{ error }}</div>
    
    <div v-if="treeData" class="tree-view">
      <h3>决策树数据</h3>
      <div class="tree-node">
        <div class="node-info">
          <div class="node-header">
            <strong>节点ID:</strong> {{ treeData.node_id }} 
            <span class="player-badge" :class="treeData.current_player">
              {{ playerLabels[treeData.current_player] }}
            </span>
          </div>
          
          <div v-if="treeData.total_bet_amount" class="bet-info">
            <strong>总下注:</strong> OOP: {{ treeData.total_bet_amount[0] }}, IP: {{ treeData.total_bet_amount[1] }}
          </div>
          
          <div v-if="treeData.actions.length > 0" class="actions">
            <strong>可用动作:</strong>
            <span v-for="action in treeData.actions" :key="action" class="action-badge">
              {{ action }}
            </span>
          </div>
          
          <div v-if="treeData.strategy.length > 0" class="strategy">
            <strong>策略:</strong>
            <div class="strategy-grid">
              <div v-for="(prob, index) in treeData.strategy" :key="index" class="strategy-item">
                {{ treeData.actions[index % treeData.actions.length] }}: {{ (prob * 100).toFixed(2) }}%
              </div>
            </div>
          </div>
        </div>
        
        <div v-if="Object.keys(treeData.children).length > 0" class="children">
          <tree-node 
            v-for="(child, action) in treeData.children" 
            :key="action"
            :node="child"
            :action="action"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, defineComponent } from 'vue';
import { getDecisionTree, getCurrentNodeWithChildren } from '../invokes';
import { invoke } from "@tauri-apps/api/tauri";

// 递归组件用于显示树节点
const TreeNode = {
  name: 'TreeNode',
  props: {
    node: Object,
    action: String
  },
  setup(props) {
    const playerLabels = {
      'oop': '场外玩家',
      'ip': '场内玩家',
      'terminal': '终止节点',
      'chance': '机会节点',
      'unknown': '未知'
    };
    
    return { playerLabels };
  },
  template: `
    <div class="tree-node">
      <div class="node-link">{{ action }}</div>
      <div class="node-info">
        <div class="node-header">
          <strong>节点ID:</strong> {{ node.node_id }}
          <span class="player-badge" :class="node.current_player">
            {{ playerLabels[node.current_player] }}
          </span>
        </div>
        
        <div v-if="node.total_bet_amount" class="bet-info">
          <strong>总下注:</strong> OOP: {{ node.total_bet_amount[0] }}, IP: {{ node.total_bet_amount[1] }}
        </div>
        
        <div v-if="node.actions.length > 0" class="actions">
          <strong>可用动作:</strong>
          <span v-for="action in node.actions" :key="action" class="action-badge">
            {{ action }}
          </span>
        </div>
        
        <div v-if="node.strategy.length > 0" class="strategy">
          <strong>策略:</strong>
          <div class="strategy-grid">
            <div v-for="(prob, index) in node.strategy" :key="index" class="strategy-item">
              {{ node.actions[index % node.actions.length] }}: {{ (prob * 100).toFixed(2) }}%
            </div>
          </div>
        </div>
      </div>
      
      <div v-if="Object.keys(node.children).length > 0" class="children">
        <tree-node 
          v-for="(child, action) in node.children" 
          :key="action"
          :node="child"
          :action="action"
        />
      </div>
    </div>
  `
};

export default defineComponent({
  name: 'DecisionTree',
  components: {
    TreeNode
  },
  setup() {
    const treeData = ref(null);
    const loading = ref(false);
    const error = ref(null);
    const consoleStatus = ref(0);
    
    const playerLabels = {
      'oop': '场外玩家',
      'ip': '场内玩家',
      'terminal': '终止节点',
      'chance': '机会节点',
      'unknown': '未知'
    };
    
    const fetchDecisionTree = async (maxDepth) => {
      loading.value = true;
      error.value = null;
      
      try {
        treeData.value = await getDecisionTree(maxDepth);
      } catch (err) {
        error.value = `获取决策树出错: ${err.message}`;
        console.error(err);
      } finally {
        loading.value = false;
      }
    };
    
    const fetchCurrentNode = async () => {
      loading.value = true;
      error.value = null;
      
      try {
        treeData.value = await getCurrentNodeWithChildren();
      } catch (err) {
        error.value = `获取当前节点出错: ${err.message}`;
        console.error(err);
      } finally {
        loading.value = false;
      }
    };
    
    const saveTreeToFile = async () => {
      if (!treeData.value) {
        error.value = "没有树数据可保存";
        return;
      }
      
      try {
        const jsonString = JSON.stringify(treeData.value, null, 2);
        const blob = new Blob([jsonString], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        
        const a = document.createElement('a');
        a.href = url;
        a.download = 'decision_tree.json';
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
      } catch (err) {
        error.value = `保存文件出错: ${err.message}`;
        console.error(err);
      }
    };
    
    // 添加打印到Rust控制台的功能
    const printToRustConsole = async () => {
      if (!treeData.value) {
        error.value = "没有树数据可打印";
        return;
      }
      
      try {
        // 构建包含元数据的调试信息
        const debugInfo = {
          timestamp: new Date().toISOString(),
          source: "GTO Decision Tree",
          data: treeData.value
        };
        
        // 调用后端命令将数据发送到控制台
        await invoke('log_to_console', { 
          message: JSON.stringify(debugInfo, null, 2)
        });
        
        // 显示成功状态
        consoleStatus.value = 1;
        setTimeout(() => {
          consoleStatus.value = 0;
        }, 2000);
      } catch (err) {
        // 显示错误状态
        error.value = `发送到控制台出错: ${err.message}`;
        console.error(err);
        consoleStatus.value = -1;
        setTimeout(() => {
          consoleStatus.value = 0;
        }, 2000);
      }
    };
    
    return {
      treeData,
      loading,
      error,
      consoleStatus,
      playerLabels,
      fetchDecisionTree,
      fetchCurrentNode,
      saveTreeToFile,
      printToRustConsole
    };
  }
});
</script>

<style scoped>
.decision-tree-container {
  padding: 20px;
  font-family: Arial, sans-serif;
}

.controls {
  margin-bottom: 20px;
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 10px;
}

button {
  padding: 8px 16px;
  background-color: #4CAF50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:hover {
  background-color: #45a049;
}

.console-button {
  background-color: #27ae60;
}

.console-button:hover {
  background-color: #219653;
}

.success-message {
  color: #2ecc71;
  margin-left: 10px;
}

.error-message {
  color: #e74c3c;
  margin-left: 10px;
}

.loading {
  color: #666;
  font-style: italic;
}

.error {
  color: red;
  margin: 10px 0;
}

.tree-view {
  margin-top: 20px;
}

.tree-node {
  margin: 10px 0;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  background-color: #f9f9f9;
}

.node-link {
  font-weight: bold;
  margin-bottom: 5px;
  padding: 2px 6px;
  background-color: #eee;
  border-radius: 3px;
  display: inline-block;
}

.node-info {
  margin-bottom: 10px;
}

.node-header {
  margin-bottom: 10px;
  display: flex;
  align-items: center;
}

.player-badge {
  margin-left: 10px;
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 0.8em;
  color: white;
}

.player-badge.oop {
  background-color: #2196F3;
}

.player-badge.ip {
  background-color: #FF9800;
}

.player-badge.terminal {
  background-color: #F44336;
}

.player-badge.chance {
  background-color: #9C27B0;
}

.player-badge.unknown {
  background-color: #607D8B;
}

.bet-info {
  margin-bottom: 5px;
}

.actions {
  margin-bottom: 10px;
}

.action-badge {
  display: inline-block;
  margin-right: 5px;
  padding: 2px 6px;
  background-color: #eee;
  border-radius: 3px;
}

.strategy {
  margin-top: 10px;
}

.strategy-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  gap: 5px;
}

.strategy-item {
  padding: 4px;
  background-color: #e9e9e9;
  border-radius: 3px;
}

.children {
  margin-left: 20px;
  padding-left: 10px;
  border-left: 2px dashed #ccc;
}
</style> 