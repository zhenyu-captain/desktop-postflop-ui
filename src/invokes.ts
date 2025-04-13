import { invoke } from "@tauri-apps/api";
import { Results, ChanceReports } from "./result-types";
import { open, save } from "@tauri-apps/api/dialog";
import { appWindow } from "@tauri-apps/api/window";

export const osName = async (): Promise<"windows" | "macos" | "linux"> => {
  return await invoke("os_name");
};

export const memory = async (): Promise<number[]> => {
  return await invoke("memory");
};

export const setNumThreads = async (numThreads: number) => {
  await invoke("set_num_threads", { numThreads });
};

/* Ranges */

export const rangeNumCombos = async (player: number): Promise<number> => {
  return await invoke("range_num_combos", { player });
};

export const rangeClear = async (player: number) => {
  await invoke("range_clear", { player });
};

export const rangeInvert = async (player: number) => {
  await invoke("range_invert", { player });
};

export const rangeUpdate = async (
  player: number,
  row: number,
  col: number,
  weight: number
) => {
  await invoke("range_update", { player, row, col, weight });
};

export const rangeFromString = async (
  player: number,
  str: string
): Promise<string | null> => {
  return await invoke("range_from_string", { player, str });
};

export const rangeToString = async (player: number): Promise<string> => {
  return await invoke("range_to_string", { player });
};

export const rangeGetWeights = async (player: number): Promise<number[]> => {
  return await invoke("range_get_weights", { player });
};

export const rangeRawData = async (player: number): Promise<number[]> => {
  return await invoke("range_raw_data", { player });
};

// 添加rangeInfo方法
export const rangeInfo = async (player: number): Promise<any> => {
  try {
    const weights = await rangeGetWeights(player);
    const numCombos = await rangeNumCombos(player);
    const rangeString = await rangeToString(player);
    
    return {
      player,
      weights,
      numCombos,
      rangeString
    };
  } catch (error) {
    return {
      player,
      error: String(error)
    };
  }
};

/* Action Tree */

export const treeNew = async (
  boardLen: number,
  startingPot: number,
  effectiveStack: number,
  donkOption: boolean,
  oopFlopBet: string,
  oopFlopRaise: string,
  oopTurnBet: string,
  oopTurnRaise: string,
  oopTurnDonk: string,
  oopRiverBet: string,
  oopRiverRaise: string,
  oopRiverDonk: string,
  ipFlopBet: string,
  ipFlopRaise: string,
  ipTurnBet: string,
  ipTurnRaise: string,
  ipRiverBet: string,
  ipRiverRaise: string,
  addAllinThreshold: number,
  forceAllinThreshold: number,
  mergingThreshold: number,
  addedLines: string,
  removedLines: string
): Promise<boolean> => {
  return await invoke("tree_new", {
    boardLen,
    startingPot,
    effectiveStack,
    donkOption,
    oopFlopBet,
    oopFlopRaise,
    oopTurnBet,
    oopTurnRaise,
    oopTurnDonk,
    oopRiverBet,
    oopRiverRaise,
    oopRiverDonk,
    ipFlopBet,
    ipFlopRaise,
    ipTurnBet,
    ipTurnRaise,
    ipRiverBet,
    ipRiverRaise,
    addAllinThreshold,
    forceAllinThreshold,
    mergingThreshold,
    addedLines,
    removedLines,
  });
};

export const treeAddedLines = async (): Promise<string> => {
  return await invoke("tree_added_lines");
};

export const treeRemovedLines = async (): Promise<string> => {
  return await invoke("tree_removed_lines");
};

export const treeInvalidTerminals = async (): Promise<string> => {
  return await invoke("tree_invalid_terminals");
};

export const treeActions = async (): Promise<string[]> => {
  return await invoke("tree_actions");
};

export const treeIsTerminalNode = async (): Promise<boolean> => {
  return await invoke("tree_is_terminal_node");
};

export const treeIsChanceNode = async (): Promise<boolean> => {
  return await invoke("tree_is_chance_node");
};

export const treeBackToRoot = async () => {
  await invoke("tree_back_to_root");
};

export const treeApplyHistory = async (line: string[]) => {
  await invoke("tree_apply_history", { line });
};

export const treePlay = async (action: string): Promise<number> => {
  return await invoke("tree_play", { action });
};

export const treeTotalBetAmount = async (): Promise<number[]> => {
  return await invoke("tree_total_bet_amount");
};

export const treeAddBetAction = async (amount: number, isRaise: boolean) => {
  await invoke("tree_add_bet_action", { amount, isRaise });
};

export const treeRemoveCurrentNode = async () => {
  await invoke("tree_remove_current_node");
};

export const treeDeleteAddedLine = async (line: string) => {
  await invoke("tree_delete_added_line", { line });
};

export const treeDeleteRemovedLine = async (line: string) => {
  await invoke("tree_delete_removed_line", { line });
};

/* Bunching effect */

export const bunchingInit = async (board: number[]): Promise<string | null> => {
  return await invoke("bunching_init", { board });
};

export const bunchingClear = async () => {
  await invoke("bunching_clear");
};

export const bunchingProgress = async (): Promise<number[]> => {
  return await invoke("bunching_progress");
};

/* Game */

export const gameInit = async (
  board: number[],
  startingPot: number,
  effectiveStack: number,
  rakeRate: number,
  rakeCap: number,
  donkOption: boolean,
  oopFlopBet: string,
  oopFlopRaise: string,
  oopTurnBet: string,
  oopTurnRaise: string,
  oopTurnDonk: string,
  oopRiverBet: string,
  oopRiverRaise: string,
  oopRiverDonk: string,
  ipFlopBet: string,
  ipFlopRaise: string,
  ipTurnBet: string,
  ipTurnRaise: string,
  ipRiverBet: string,
  ipRiverRaise: string,
  addAllinThreshold: number,
  forceAllinThreshold: number,
  mergingThreshold: number,
  addedLines: string,
  removedLines: string
): Promise<string | null> => {
  return await invoke("game_init", {
    board,
    startingPot,
    effectiveStack,
    rakeRate,
    rakeCap,
    donkOption,
    oopFlopBet,
    oopFlopRaise,
    oopTurnBet,
    oopTurnRaise,
    oopTurnDonk,
    oopRiverBet,
    oopRiverRaise,
    oopRiverDonk,
    ipFlopBet,
    ipFlopRaise,
    ipTurnBet,
    ipTurnRaise,
    ipRiverBet,
    ipRiverRaise,
    addAllinThreshold,
    forceAllinThreshold,
    mergingThreshold,
    addedLines,
    removedLines,
  });
};

export const gamePrivateCards = async (): Promise<number[][]> => {
  return await invoke("game_private_cards");
};

export const gameMemoryUsage = async (): Promise<number[]> => {
  return await invoke("game_memory_usage");
};

export const gameMemoryUsageBunching = async (): Promise<number> => {
  return await invoke("game_memory_usage_bunching");
};

export const gameAllocateMemory = async (enableCompression: boolean) => {
  await invoke("game_allocate_memory", { enableCompression });
};

export const gameSetBunching = async (): Promise<string | null> => {
  return await invoke("game_set_bunching");
};

export const gameSolveStep = async (currentIteration: number) => {
  await invoke("game_solve_step", { currentIteration });
};

export const gameExploitability = async (): Promise<number> => {
  return await invoke("game_exploitability");
};

export const gameFinalize = async () => {
  await invoke("game_finalize");
};

export const gameApplyHistory = async (history: number[]) => {
  await invoke("game_apply_history", { history });
};

export const gameTotalBetAmount = async (
  append: number[]
): Promise<number[]> => {
  return await invoke("game_total_bet_amount", { append });
};

export const gameActionsAfter = async (append: number[]): Promise<string[]> => {
  return await invoke("game_actions_after", { append });
};

export const gamePossibleCards = async (): Promise<bigint> => {
  return BigInt(await invoke("game_possible_cards"));
};

type ResultsResponse = {
  current_player: "oop" | "ip" | "chance" | "terminal";
  num_actions: number;
  is_empty: number;
  eqr_base: number[];
  weights: number[][];
  normalizer: number[][];
  equity: number[][];
  ev: number[][];
  eqr: number[][];
  strategy: number[];
  action_ev: number[];
};

export const gameGetResults = async (): Promise<Results> => {
  const results: ResultsResponse = await invoke("game_get_results");
  return {
    currentPlayer: results.current_player,
    numActions: results.num_actions,
    isEmpty: results.is_empty,
    eqrBase: results.eqr_base,
    weights: results.weights,
    normalizer: results.normalizer,
    equity: results.equity,
    ev: results.ev,
    eqr: results.eqr,
    strategy: results.strategy,
    actionEv: results.action_ev,
  };
};

type ChanceReportsResponse = {
  num_actions: number;
  status: number[];
  combos: number[][];
  equity: number[][];
  ev: number[][];
  eqr: number[][];
  strategy: number[];
};

export const gameGetChanceReports = async (
  append: number[],
  currentPlayer: "oop" | "ip" | "terminal",
  numActions: number
): Promise<ChanceReports> => {
  const reports: ChanceReportsResponse = await invoke(
    "game_get_chance_reports",
    { append, numActions }
  );
  return {
    currentPlayer,
    numActions,
    status: reports.status,
    combos: reports.combos,
    equity: reports.equity,
    ev: reports.ev,
    eqr: reports.eqr,
    strategy: reports.strategy,
  };
};

export const getDecisionTree = async (maxDepth: number) => {
  return await invoke("get_decision_tree", { maxDepth });
};

export const getCurrentNodeWithChildren = async () => {
  return await invoke("get_current_node_with_children");
};

// 获取完整的决策路径数据
export async function getCompleteDecisionPath(): Promise<any> {
  return invoke("get_complete_decision_path");
}

// 添加保存游戏状态到二进制文件的函数
export const gameSaveToBin = async (
  filePath: string,
  memo: string
): Promise<string | null> => {
  try {
    await invoke("game_save_to_bin", { filePath, memo });
    return null; // 成功返回 null
  } catch (error) {
    return error as string; // 失败返回错误信息
  }
};

// 添加从二进制文件加载游戏状态的函数
export const gameLoadFromBin = async (
  filePath: string
): Promise<string | null> => {
  try {
    await invoke("game_load_from_bin", { filePath });
    return null; // 成功返回 null
  } catch (error) {
    return error as string; // 失败返回错误信息
  }
};

// 添加弹出保存对话框并保存的辅助函数
export const showSaveDialog = async (
  memo: string = ""
): Promise<string | null> => {
  try {
    const filePath = await save({
      filters: [{
        name: "GTO Solver Files",
        extensions: ["bin"]
      }],
      defaultPath: "solver_state.bin",
      title: "保存求解状态",
    });
    
    if (!filePath) return "用户取消了保存操作";
    
    return await gameSaveToBin(filePath as string, memo);
  } catch (error) {
    return error as string;
  }
};

// 添加弹出加载对话框并加载的辅助函数
export const showLoadDialog = async (): Promise<string | null> => {
  try {
    const selected = await open({
      filters: [{
        name: "GTO Solver Files",
        extensions: ["bin"]
      }],
      multiple: false,
      directory: false,
      title: "加载求解状态",
    });
    
    if (!selected || Array.isArray(selected)) return "未选择文件或选择了多个文件";
    
    return await gameLoadFromBin(selected as string);
  } catch (error) {
    return error as string;
  }
};

// 添加获取游戏配置信息的函数
export interface GameConfigInfo {
  board: number[];
  starting_pot: number;
  effective_stack: number;
}

export const gameGetConfigInfo = async (): Promise<GameConfigInfo> => {
  return await invoke("game_get_config_info");
};

// 添加保存和加载配置的函数
export const showSaveConfigDialog = async (
  configData: string
): Promise<string | null> => {
  try {
    const filePath = await save({
      filters: [{
        name: "GTO Config Files",
        extensions: ["json"]
      }],
      defaultPath: "poker_config.json",
      title: "保存配置文件",
    });
    
    if (!filePath) return "用户取消了保存操作";
    
    await invoke("save_config_to_file", { 
      filePath, 
      configData 
    });
    
    return null;
  } catch (error) {
    return error as string;
  }
};

export const showLoadConfigDialog = async (): Promise<{ configData: string } | string> => {
  try {
    const selected = await open({
      filters: [{
        name: "GTO Config Files",
        extensions: ["json"]
      }],
      multiple: false,
      directory: false,
      title: "加载配置文件",
    });
    
    if (!selected || Array.isArray(selected)) return "未选择文件或选择了多个文件";
    
    const configData = await invoke("load_config_from_file", { 
      filePath: selected as string 
    });
    
    return { configData: configData as string };
  } catch (error) {
    return error as string;
  }
};
