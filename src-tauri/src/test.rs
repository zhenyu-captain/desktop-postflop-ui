use crate::solver::*;
use crate::tree::*;
use postflop_solver::*;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;

// 记录决策树节点信息的结构体
#[derive(Serialize, Clone)]
pub struct TreeNodeInfo {
    pub node_id: String,
    pub current_player: String, // "oop", "ip", "terminal", "chance"
    pub actions: Vec<String>,   // 可用动作列表
    pub strategy: Vec<f64>,     // 当前节点的策略
    pub children: HashMap<String, TreeNodeInfo>, // 子节点集合，键为动作
    pub ev: [Vec<f64>; 2],      // 双方的期望值
    pub equity: [Vec<f64>; 2],  // 双方的权益
    pub is_terminal: bool,
    pub is_chance: bool,
    pub total_bet_amount: [i32; 2], // 双方的总下注额
}

// 获取决策树的命令
#[tauri::command(async)]
pub fn get_decision_tree(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    tree_state: tauri::State<Mutex<ActionTree>>,
    max_depth: usize, // 限制树的深度，防止过大
) -> TreeNodeInfo {
    let mut game = game_state.lock().unwrap();
    let mut tree = tree_state.lock().unwrap();
    
    // 回到根节点
    tree.back_to_root();
    let history = game.history().to_vec();
    game.apply_history(&[]);
    
    // 确保规范化权重已被缓存
    game.cache_normalized_weights();
    
    // 递归构建决策树
    let tree_info = build_tree_node(&mut game, &mut tree, String::from("root"), max_depth, 0);
    
    // 恢复原始状态
    tree.back_to_root();
    game.apply_history(&history);
    
    tree_info
}

// 递归构建树节点
fn build_tree_node(
    game: &mut PostFlopGame,
    tree: &mut ActionTree,
    node_id: String,
    max_depth: usize,
    current_depth: usize,
) -> TreeNodeInfo {
    // 获取当前节点信息
    let is_terminal = tree.is_terminal_node();
    let is_chance = tree.is_chance_node();
    
    // ActionTree没有current_player方法，我们需要根据游戏状态来判断
    let current_player = if is_terminal {
        "terminal"
    } else if is_chance {
        "chance"
    } else if game.current_player() == 0 {
        // 使用game的current_player方法，而不是tree的
        "oop"
    } else {
        "ip"
    };
    
    // 获取可用动作
    let actions = if !is_terminal && !is_chance {
        tree.available_actions()
            .iter()
            .map(|&action| match action {
                Action::Fold => "F".to_string(),
                Action::Check => "X".to_string(),
                Action::Call => "C".to_string(),
                Action::Bet(amount) => format!("B{}", amount),
                Action::Raise(amount) => format!("R{}", amount),
                Action::AllIn(amount) => format!("A{}", amount),
                _ => "unknown".to_string(),
            })
            .collect()
    } else {
        vec![]
    };
    
    // 获取当前节点的策略和EV
    let results = if !is_terminal && !is_chance {
        game.cache_normalized_weights();
        
        let strategy = if !game.is_terminal_node() && !game.is_chance_node() {
            game.strategy().iter().map(|&x| x as f64).collect()
        } else {
            vec![]
        };
        
        let equity = [
            game.equity(0).iter().map(|&x| x as f64).collect(),
            game.equity(1).iter().map(|&x| x as f64).collect(),
        ];
        
        let ev = [
            game.expected_values(0).iter().map(|&x| x as f64).collect(),
            game.expected_values(1).iter().map(|&x| x as f64).collect(),
        ];
        
        (strategy, equity, ev)
    } else {
        (vec![], [vec![], vec![]], [vec![], vec![]])
    };
    
    let mut node_info = TreeNodeInfo {
        node_id,
        current_player: current_player.to_string(),
        actions: actions.clone(),
        strategy: results.0,
        children: HashMap::new(),
        equity: results.1,
        ev: results.2,
        is_terminal,
        is_chance,
        total_bet_amount: game.total_bet_amount(),
    };
    
    // 如果达到最大深度或是终止节点，则不再递归
    if current_depth >= max_depth || is_terminal {
        return node_info;
    }
    
    // 为每个可用动作创建子节点
    for (i, action_str) in actions.iter().enumerate() {
        let action = match action_str.chars().next().unwrap() {
            'F' => Action::Fold,
            'X' => Action::Check,
            'C' => Action::Call,
            'B' => {
                let amount = action_str[1..].parse().unwrap();
                Action::Bet(amount)
            },
            'R' => {
                let amount = action_str[1..].parse().unwrap();
                Action::Raise(amount)
            },
            'A' => {
                let amount = action_str[1..].parse().unwrap();
                Action::AllIn(amount)
            },
            _ => continue, // 跳过未知动作
        };
        
        // 记录当前历史
        let tree_history = tree.history().to_vec();
        let game_history = game.history().to_vec();
        
        // 执行动作
        tree.play(action);
        game.play(i);
        
        // 递归构建子节点
        let child_id = format!("{}_{}", node_info.node_id, action_str);
        let child_info = build_tree_node(game, tree, child_id, max_depth, current_depth + 1);
        node_info.children.insert(action_str.clone(), child_info);
        
        // 恢复状态
        tree.back_to_root();
        tree.apply_history(&tree_history).unwrap();
        game.apply_history(&game_history);
    }
    
    node_info
}

// 获取当前节点及其子节点的命令
#[tauri::command]
pub fn get_current_node_with_children(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    tree_state: tauri::State<Mutex<ActionTree>>,
) -> TreeNodeInfo {
    let mut game = game_state.lock().unwrap();
    let tree = tree_state.lock().unwrap();
    
    // 获取当前节点信息
    let is_terminal = tree.is_terminal_node();
    let is_chance = tree.is_chance_node();
    
    // 使用game的current_player方法获取当前玩家
    let current_player = if is_terminal {
        "terminal"
    } else if is_chance {
        "chance"
    } else if game.current_player() == 0 {
        "oop"
    } else {
        "ip"
    };
    
    // 获取可用动作
    let actions = if !is_terminal && !is_chance {
        tree.available_actions()
            .iter()
            .map(|&action| match action {
                Action::Fold => "F".to_string(),
                Action::Check => "X".to_string(),
                Action::Call => "C".to_string(),
                Action::Bet(amount) => format!("B{}", amount),
                Action::Raise(amount) => format!("R{}", amount),
                Action::AllIn(amount) => format!("A{}", amount),
                _ => "unknown".to_string(),
            })
            .collect()
    } else {
        vec![]
    };
    
    // 获取策略和EV
    let mut strategy = vec![];
    let mut equity = [vec![], vec![]];
    let mut ev = [vec![], vec![]];
    
    if !is_terminal && !is_chance {
        // 确保在访问规范化权重前先缓存它们
        game.cache_normalized_weights();
        
        if !game.is_terminal_node() && !game.is_chance_node() {
            strategy = game.strategy().iter().map(|&x| x as f64).collect();
        }
        
        equity[0] = game.equity(0).iter().map(|&x| x as f64).collect();
        equity[1] = game.equity(1).iter().map(|&x| x as f64).collect();
        
        ev[0] = game.expected_values(0).iter().map(|&x| x as f64).collect();
        ev[1] = game.expected_values(1).iter().map(|&x| x as f64).collect();
    }
    
    // 构建节点信息
    TreeNodeInfo {
        node_id: tree.history()
            .iter()
            .map(|&action| match action {
                Action::Fold => "F".to_string(),
                Action::Check => "X".to_string(),
                Action::Call => "C".to_string(),
                Action::Bet(amount) => format!("B{}", amount),
                Action::Raise(amount) => format!("R{}", amount),
                Action::AllIn(amount) => format!("A{}", amount),
                _ => "unknown".to_string(),
            })
            .collect::<Vec<_>>()
            .join("-"),
        current_player: current_player.to_string(),
        actions,
        strategy,
        children: HashMap::new(), // 只返回当前节点，不包含子节点
        equity,
        ev,
        is_terminal,
        is_chance,
        total_bet_amount: game.total_bet_amount(),
    }
} 