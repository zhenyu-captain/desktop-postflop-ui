### 说明
1. clean.sh
2. npm install
3. npm run tauri dev
4. happy
5. shift + command + i 调出浏览器控制台

### bug解决
🔍 问题的真正根源
🎯 核心问题：ResultViewer2.vue的无限重渲染循环

循环链路：
1. Solver完成 → store.isSolverFinished = true
2. 触发ResultViewer2组件渲染
3. 组件渲染 → 调用setNavRef(tabId, ref)
4. setNavRef设置 currentActiveTabId.value = tabId ← 触发响应式更新
5. currentActiveTabId变化 → 组件重新渲染
6. 回到步骤3，无限循环！

解决方案的关键点
三重保护
// 1️⃣ 引用相等检查 - 最关键！
if (existingRef === ref) {
  return; // 同一个ref对象，直接返回，避免任何操作
}

// 2️⃣ 首次设置时不更新currentActiveTabId
if (!existingRef) {
  navRefs.value.set(tabId, ref);
  // ❌ 不在这里设置 currentActiveTabId
}

// 3️⃣ 在onMounted中初始化
onMounted(() => {
  currentActiveTabId.value = activeId; // 只执行一次
});

// 移除所有setTimeout(resolve, 0)
// 替换为nextTick()
await nextTick(); // 使用微任务，更可靠


/ 统一Rust命令的锁顺序
// 移除错误的#[tauri::command(async)]装饰器
// 添加详细的调试日志