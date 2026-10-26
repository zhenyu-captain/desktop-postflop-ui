# desktop-postflop-ui

Secondary development (customisation) of the open-source GTO poker study tool
[desktop-postflop](https://github.com/b-inary/desktop-postflop) by b-inary,
upstream licensed under **AGPL-3.0** (see LICENSE). Credit for the upstream
solver and UI codebase belongs to its original authors.

Custom work in this repository (2025-03 ~ 2025-09, ~97 commits):

- remote version-control / free-shutdown gating (`version_control.json`, `remote_open_shutdown_gto.sh`)
- network connectivity check with gated entry window
- Results Pro / Results Classic views, default-EV display fixes
- build alignment for Windows / macOS distribution

Distributed under AGPL-3.0, same as upstream.

---

# 使用说明
1. clean.sh
2. npm install
3. npm run tauri dev
4. happy
5. shift + command + i 调出浏览器控制台
6. npm run tauri build 构建分发版本


# 重要新增
1. remote_open_shutdown_gto.sh控制了free字段值为shutdown还是free，在shutdown的时候弹出“版本检测停用”窗口，free的时候直接进入。
2. 网络通的情况下直接进入，不通的时候“Network Connection Check”窗口弹出，用户需要与gtosim.com和www.gtosim.com的网络是通的情况下才能使用。



## 重要错误调试记录
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