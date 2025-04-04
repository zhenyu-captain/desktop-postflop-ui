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

// 决策路径中的动作数据结构
#[derive(Serialize, Clone)]
pub struct ActionInfo {
    pub name: String,
    pub amount: String,
    pub index: usize,
}

// 基础路径节点
#[derive(Serialize, Clone)]
pub struct PathNodeBase {
    pub r#type: String,  // "root", "player", "chance", "terminal"
    pub index: usize,
    pub player: String,
}

// 根节点
#[derive(Serialize, Clone)]
pub struct RootNode {
    pub r#type: String,
    pub index: usize,
    pub player: String,
    pub pot: i32,
    pub board: Vec<String>,
    pub stack: i32,
    pub equity: Option<f32>,
    pub ev: Option<f32>,
    pub hands: Option<Vec<HandData>>, // 添加手牌数据
}

// 玩家动作节点
#[derive(Serialize, Clone)]
pub struct PlayerNode {
    pub r#type: String,
    pub index: usize,
    pub player: String,
    pub action: ActionInfo,
    pub equity: Option<f32>,
    pub ev: Option<f32>,
    pub hands: Option<Vec<HandData>>, // 添加手牌数据
}

// 机会节点（转牌、河牌）
#[derive(Serialize, Clone)]
pub struct ChanceNode {
    pub r#type: String,
    pub index: usize,
    pub player: String,
    pub pot: i32,
    pub selected_card: u8,
    pub card_display: String,
    pub stack: i32,
}

// 终端节点
#[derive(Serialize, Clone)]
pub struct TerminalNode {
    pub r#type: String,
    pub index: usize,
    pub player: String,
    pub pot: i32,
    pub equity_oop: f64,
    pub equity_ip: f64,
    pub equity_percentage: HashMap<String, String>,
}

// 决策路径位置信息
#[derive(Serialize, Clone)]
pub struct PathPosition {
    pub selected_spot_index: usize,
    pub selected_chance_index: i32,
}

// 下注信息
#[derive(Serialize, Clone)]
pub struct BettingInfo {
    pub total_bet: [i32; 2],
    pub pot_size: i32,
    pub starting_pot: i32,
    pub stack: i32,
}

// 完整的决策路径数据
#[derive(Serialize, Clone)]
pub struct CompleteDecisionPath {
    pub decision_path: Vec<serde_json::Value>,
    pub board: Vec<String>,
    pub current_position: PathPosition,
    pub betting_info: BettingInfo,
    pub path_summary: String,
}

// 手牌数据结构
#[derive(Serialize, Clone)]
pub struct HandData {
    pub hand: String,       // 手牌，如"AA", "KK"等
    pub equity: f32,        // 手牌权益
    pub ev: f32,            // 期望值
    pub weight: f32,        // 权重
    pub combos: u8,         // 组合数
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
            }
            'R' => {
                let amount = action_str[1..].parse().unwrap();
                Action::Raise(amount)
            }
            'A' => {
                let amount = action_str[1..].parse().unwrap();
                Action::AllIn(amount)
            }
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
        node_id: if tree.history().is_empty() {
            "root".to_string()
        } else {
            tree.history()
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
                .join("-")
        },
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

// 获取完整决策路径的命令
#[tauri::command]
pub fn get_complete_decision_path(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    _tree_state: tauri::State<Mutex<ActionTree>>,
) -> Result<CompleteDecisionPath, String> {
    // 1. 一次性获取所有必要的游戏数据，避免频繁获取锁
    let (config, card_config, original_history, total_bet) = {
        let game = game_state.lock().unwrap();
        
        let config = game.tree_config().clone();
        let card_config = game.card_config().clone();
        let history = game.history().to_vec();
        let total_bet = game.total_bet_amount();
        
        (config, card_config, history, total_bet)
    };
    
    // 2. 计算底池相关值
    let starting_pot = config.starting_pot;
    let effective_stack = config.effective_stack;
    let current_pot = starting_pot + total_bet[0] + total_bet[1];
    let remaining_stack = effective_stack - total_bet[0].max(total_bet[1]);
    
    // 3. 构建牌面信息
    let mut board_cards = Vec::new();
    
    // 添加翻牌
    for &card in &card_config.flop {
        board_cards.push(card_to_string(card));
    }
    
    // 添加转牌和河牌(如果已发牌)
    if card_config.turn != NOT_DEALT {
        board_cards.push(card_to_string(card_config.turn));
    }
    
    if card_config.river != NOT_DEALT {
        board_cards.push(card_to_string(card_config.river));
    }
    
    // 4. 构建决策路径
    let mut decision_path = Vec::new();
    let mut path_index = 0;
    
    // 4.1 添加根节点
    let (root_equity, root_ev, root_hands) = {
        let mut game = game_state.lock().unwrap();
        game.apply_history(&[]); // 回到根节点
        game.cache_normalized_weights();
        
        // 获取整体节点的EQ和EV - 使用加权平均值计算
        let equity_values = game.equity(0); // OOP玩家的权益
        let ev_values = game.expected_values(0); // OOP玩家的EV
        let weights = game.normalized_weights(0); // OOP玩家的权重
        
        // 如果有数据，使用加权平均计算整体EQ和EV
        let equity_val = if !equity_values.is_empty() && !weights.is_empty() {
            let avg = weighted_average(&equity_values, &weights);
            Some(avg as f32)
        } else {
            None
        };
        
        let ev_val = if !ev_values.is_empty() && !weights.is_empty() {
            let avg = weighted_average(&ev_values, &weights);
            Some(avg as f32)
        } else {
            None
        };
        
        // 获取手牌数据
        let mut hands_data = Vec::new();
        
        // 添加AA手牌数据
        let aa_indices = get_hand_indices("AA", &game);
        if !aa_indices.is_empty() {
            let mut total_equity = 0.0;
            let mut total_ev = 0.0;
            let mut total_weight = 0.0;
            let mut combo_count = 0;
            
            for &hand_idx in &aa_indices {
                if hand_idx < weights.len() {
                    let weight = weights[hand_idx];
                    if weight > 0.0 {
                        combo_count += 1;
                        total_weight += weight;
                        
                        if hand_idx < equity_values.len() {
                            total_equity += equity_values[hand_idx] * weight;
                        }
                        
                        if hand_idx < ev_values.len() {
                            total_ev += ev_values[hand_idx] * weight;
                        }
                    }
                }
            }
            
            // 只有当有实际权重时才添加手牌数据
            if total_weight > 0.0 {
                hands_data.push(HandData {
                    hand: "AA".to_string(),
                    equity: (total_equity / total_weight) as f32,
                    ev: (total_ev / total_weight) as f32,
                    weight: total_weight,
                    combos: combo_count as u8,
                });
            }
        }
        
        // 添加所有AA花色组合
        let aa_combos = [
            ("AsAh", "A♠A♥"), 
            ("AsAd", "A♠A♦"), 
            ("AsAc", "A♠A♣"),
            ("AhAd", "A♥A♦"), 
            ("AhAc", "A♥A♣"), 
            ("AdAc", "A♦A♣")
        ];
        
        for (code, display) in aa_combos.iter() {
            let combo_indices = get_hand_indices(code, &game);
            if !combo_indices.is_empty() {
                let mut combo_equity = 0.0;
                let mut combo_ev = 0.0;
                let mut combo_weight = 0.0;
                let mut combo_count = 0;
                
                for &hand_idx in &combo_indices {
                    if hand_idx < weights.len() {
                        let weight = weights[hand_idx];
                        if weight > 0.0 {
                            combo_count += 1;
                            combo_weight += weight;
                            
                            if hand_idx < equity_values.len() {
                                combo_equity += equity_values[hand_idx] * weight;
                            }
                            
                            if hand_idx < ev_values.len() {
                                combo_ev += ev_values[hand_idx] * weight;
                            }
                        }
                    }
                }
                
                // 只有当有实际权重时才添加手牌数据
                if combo_weight > 0.0 {
                    hands_data.push(HandData {
                        hand: display.to_string(),
                        equity: (combo_equity / combo_weight) as f32,
                        ev: (combo_ev / combo_weight) as f32,
                        weight: combo_weight,
                        combos: combo_count as u8,
                    });
                }
            }
        }
        
        let hands_opt = if !hands_data.is_empty() { Some(hands_data) } else { None };
        
        (equity_val, ev_val, hands_opt)
    };
    
    let root_node = RootNode {
        r#type: "root".to_string(),
        index: path_index,
        player: "flop".to_string(),
        pot: starting_pot,
        board: board_cards[0..3].to_vec(),
        stack: effective_stack,
        equity: root_equity,
        ev: root_ev,
        hands: root_hands,
    };
    path_index += 1;
    decision_path.push(serde_json::to_value(root_node).unwrap());
    
    // 5. 获取游戏状态历史
    let game_states = {
        let mut game = game_state.lock().unwrap();
        game.apply_history(&[]); // 回到根节点
        
        let mut states = Vec::new();
        states.push((
            game.is_chance_node(),
            game.is_terminal_node(),
            game.current_player(),
            game.available_actions().to_vec()
        ));
        
        // 逐步应用历史并记录状态
        for &action_idx in &original_history {
            game.play(action_idx);
            states.push((
                game.is_chance_node(),
                game.is_terminal_node(),
                game.current_player(),
                game.available_actions().to_vec()
            ));
        }
        
        // 恢复原始状态
        game.apply_history(&original_history);
        states
    };
    
    // 6. 使用游戏状态历史构建决策路径
    if !original_history.is_empty() {
        // 初始化变量
        let mut street = 0; // 0 = 翻牌圈, 1 = 转牌圈, 2 = 河牌圈
        let mut pot = starting_pot;
        let mut current_player = 0; // 0 = OOP, 1 = IP
        let mut player_bet = [0, 0]; // 每个玩家的下注金额
        
        // 遍历历史动作，构建路径
        for (i, &action_index) in original_history.iter().enumerate() {
            // 获取当前状态
            let (is_chance, is_terminal, _, available_actions) = &game_states[i];
            
            // 如果是街道变化，添加机会节点
            if *is_chance {
                street += 1;
                let street_name = match street {
                    1 => "turn",
                    2 => "river",
                    _ => "unknown",
                };
                
                let card_id = if street == 1 { card_config.turn } else { card_config.river };
                // 添加边界检查，确保访问有效索引
                let card_display = if 2 + street < board_cards.len() {
                    board_cards[2 + street].clone()
                } else {
                    "?".to_string() // 如果牌面不存在，使用占位符
                };
                
                let chance_node = ChanceNode {
                    r#type: "chance".to_string(),
                    index: path_index,
                    player: street_name.to_string(),
                    pot: pot,
                    selected_card: card_id,
                    card_display,
                    stack: remaining_stack,
                };
                path_index += 1;
                decision_path.push(serde_json::to_value(chance_node).unwrap());
                
                continue;
            }
            
            // 如果是终端节点，跳出循环
            if *is_terminal {
                break;
            }
            
            // 处理玩家动作
            if action_index < available_actions.len() {
                let action = available_actions[action_index];
                
                // 根据动作类型更新底池和下注金额
                let (name, amount) = match action {
                    Action::Fold => ("Fold".to_string(), 0),
                    Action::Check => ("Check".to_string(), 0),
                    Action::Call => {
                        let call_amount = player_bet[1 - current_player] - player_bet[current_player];
                        player_bet[current_player] += call_amount;
                        pot += call_amount;
                        ("Call".to_string(), 0)
                    },
                    Action::Bet(amt) => {
                        player_bet[current_player] += amt;
                        pot += amt;
                        ("Bet".to_string(), amt)
                    },
                    Action::Raise(amt) => {
                        let raise_amount = amt - player_bet[current_player];
                        player_bet[current_player] = amt;
                        pot += raise_amount;
                        ("Raise".to_string(), amt)
                    },
                    Action::AllIn(amt) => {
                        let allin_amount = amt - player_bet[current_player];
                        player_bet[current_player] = amt;
                        pot += allin_amount;
                        ("Allin".to_string(), amt)
                    },
                    _ => ("Unknown".to_string(), 0),
                };
                
                // 获取当前节点的权益和EV
                let mut game = game_state.lock().unwrap();
                game.apply_history(&original_history[0..i]);
                game.cache_normalized_weights();
                
                let player_idx = if current_player == 0 { 0 } else { 1 };
                
                // 获取当前节点的权益和EV
                let equity_values = game.equity(player_idx);
                let ev_values = game.expected_values(player_idx);
                let weights = game.normalized_weights(player_idx);
                
                // 使用加权平均计算整体权益和EV
                let equity_val = if !equity_values.is_empty() && !weights.is_empty() {
                    let avg = weighted_average(&equity_values, &weights);
                    Some(avg as f32)
                } else {
                    None
                };
                
                let ev_val = if !ev_values.is_empty() && !weights.is_empty() {
                    let avg = weighted_average(&ev_values, &weights);
                    Some(avg as f32)
                } else {
                    None
                };
                
                // 获取手牌数据
                let mut hands_data = Vec::new();
                
                // 添加AA手牌数据
                let aa_indices = get_hand_indices("AA", &game);
                if !aa_indices.is_empty() {
                    let mut total_equity = 0.0;
                    let mut total_ev = 0.0;
                    let mut total_weight = 0.0;
                    let mut combo_count = 0;
                    
                    for &hand_idx in &aa_indices {
                        if hand_idx < weights.len() {
                            let weight = weights[hand_idx];
                            if weight > 0.0 {
                                combo_count += 1;
                                total_weight += weight;
                                
                                if hand_idx < equity_values.len() {
                                    total_equity += equity_values[hand_idx] * weight;
                                }
                                
                                if hand_idx < ev_values.len() {
                                    total_ev += ev_values[hand_idx] * weight;
                                }
                            }
                        }
                    }
                    
                    // 只有当有实际权重时才添加手牌数据
                    if total_weight > 0.0 {
                        hands_data.push(HandData {
                            hand: "AA".to_string(),
                            equity: (total_equity / total_weight) as f32,
                            ev: (total_ev / total_weight) as f32,
                            weight: total_weight,
                            combos: combo_count as u8,
                        });
                    }
                }
                
                // 添加所有AA花色组合
                let aa_combos = [
                    ("AsAh", "A♠A♥"), 
                    ("AsAd", "A♠A♦"), 
                    ("AsAc", "A♠A♣"),
                    ("AhAd", "A♥A♦"), 
                    ("AhAc", "A♥A♣"), 
                    ("AdAc", "A♦A♣")
                ];
                
                for (code, display) in aa_combos.iter() {
                    let combo_indices = get_hand_indices(code, &game);
                    if !combo_indices.is_empty() {
                        let mut combo_equity = 0.0;
                        let mut combo_ev = 0.0;
                        let mut combo_weight = 0.0;
                        let mut combo_count = 0;
                        
                        for &hand_idx in &combo_indices {
                            if hand_idx < weights.len() {
                                let weight = weights[hand_idx];
                                if weight > 0.0 {
                                    combo_count += 1;
                                    combo_weight += weight;
                                    
                                    if hand_idx < equity_values.len() {
                                        combo_equity += equity_values[hand_idx] * weight;
                                    }
                                    
                                    if hand_idx < ev_values.len() {
                                        combo_ev += ev_values[hand_idx] * weight;
                                    }
                                }
                            }
                        }
                        
                        // 只有当有实际权重时才添加手牌数据
                        if combo_weight > 0.0 {
                            hands_data.push(HandData {
                                hand: display.to_string(),
                                equity: (combo_equity / combo_weight) as f32,
                                ev: (combo_ev / combo_weight) as f32,
                                weight: combo_weight,
                                combos: combo_count as u8,
                            });
                        }
                    }
                }
                
                let hands_opt = if !hands_data.is_empty() { Some(hands_data) } else { None };
                
                // 创建玩家动作节点
                let player_node = PlayerNode {
                    r#type: "player".to_string(),
                    index: path_index,
                    player: if current_player == 0 { "oop" } else { "ip" }.to_string(),
                    action: ActionInfo {
                        name,
                        amount: amount.to_string(),
                        index: action_index,
                    },
                    equity: equity_val,
                    ev: ev_val,
                    hands: hands_opt,
                };
                
                path_index += 1;
                decision_path.push(serde_json::to_value(player_node).unwrap());
                
                // 切换玩家
                current_player = 1 - current_player;
            }
        }
        
        // 7. 如果到达终端节点，添加终端节点
        if game_states.last().unwrap().1 || !original_history.is_empty() {
            // 获取权益数据
            let (equity_oop, equity_ip) = {
                let mut game = game_state.lock().unwrap();
                game.apply_history(&original_history);
                game.cache_normalized_weights();
                
                let mut equity_oop = 0.5;
                let mut equity_ip = 0.5;
                
                if !game.is_chance_node() {
                    let equity_oop_values = game.equity(0);
                    let equity_ip_values = game.equity(1);
                    let weights_oop = game.normalized_weights(0);
                    let weights_ip = game.normalized_weights(1);
                    
                    if !equity_oop_values.is_empty() && !weights_oop.is_empty() {
                        equity_oop = weighted_average(&equity_oop_values, &weights_oop);
                    }
                    
                    if !equity_ip_values.is_empty() && !weights_ip.is_empty() {
                        equity_ip = weighted_average(&equity_ip_values, &weights_ip);
                    }
                }
                
                (equity_oop, equity_ip)
            };
            
            // 创建终端节点
            let terminal_node = TerminalNode {
                r#type: "terminal".to_string(),
                index: path_index,
                player: "equity".to_string(),
                pot: pot,
                equity_oop,
                equity_ip,
                equity_percentage: {
                    let mut map = HashMap::new();
                    map.insert("oop".to_string(), format!("{:.2}%", equity_oop * 100.0));
                    map.insert("ip".to_string(), format!("{:.2}%", equity_ip * 100.0));
                    map
                },
            };
            
            path_index += 1;
            decision_path.push(serde_json::to_value(terminal_node).unwrap());
        }
    }
    
    // 8. 生成路径摘要
    let path_summary = if decision_path.len() <= 1 {
        // 只有根节点的情况
        format!("Starting with pot {}, effective stack {} | Board: {} {} {} | Flop: ",
            starting_pot, effective_stack, board_cards[0], board_cards[1], board_cards[2])
    } else {
        generate_summary_from_path(&decision_path, &board_cards, starting_pot, effective_stack)
    };
    
    // 9. 构建完整的决策路径数据结构
    let complete_path = CompleteDecisionPath {
        decision_path,
        board: board_cards,
        current_position: PathPosition {
            selected_spot_index: path_index - 1,
            selected_chance_index: -1,
        },
        betting_info: BettingInfo {
            total_bet,
            pot_size: current_pot,
            starting_pot,
            stack: remaining_stack,
        },
        path_summary,
    };
    
    Ok(complete_path)
}

// 辅助函数: 将卡牌ID转换为字符串表示
fn card_to_string(card: u8) -> String {
    if card == NOT_DEALT {
        return "".to_string();
    }
    
    let rank = card / 4;
    let suit = card % 4;
    
    let rank_char = match rank {
        0 => '2',
        1 => '3',
        2 => '4',
        3 => '5',
        4 => '6',
        5 => '7',
        6 => '8',
        7 => '9',
        8 => 'T',
        9 => 'J',
        10 => 'Q',
        11 => 'K',
        12 => 'A',
        _ => '?',
    };
    
    let suit_char = match suit {
        0 => 'c', // 梅花
        1 => 'd', // 方块
        2 => 'h', // 红桃
        3 => 's', // 黑桃
        _ => '?',
    };
    
    format!("{}{}", rank_char, suit_char)
}

// 辅助函数: 根据决策路径生成摘要
fn generate_summary_from_path(
    path: &Vec<serde_json::Value>,
    board: &Vec<String>,
    starting_pot: i32,
    effective_stack: i32
) -> String {
    let mut summary = format!("Starting with pot {}, effective stack {} | Board: ", 
        starting_pot, effective_stack);
    
    // 添加翻牌信息
    if board.len() >= 3 {
        summary.push_str(&format!("{} {} {} | ", board[0], board[1], board[2]));
    }
    
    // 添加翻牌圈动作
    summary.push_str("Flop: ");
    let mut current_street = "flop";
    
    for node in path.iter().skip(1) { // 跳过根节点
        if let Some(node_type) = node.get("type").and_then(|t| t.as_str()) {
            if node_type == "chance" {
                // 街道变化
                if let Some(player) = node.get("player").and_then(|p| p.as_str()) {
                    current_street = player;
                    if player == "turn" {
                        if let Some(card) = node.get("cardDisplay").and_then(|c| c.as_str()) {
                            summary.push_str(&format!("Turn: {} | Turn actions: ", card));
                        }
                    } else if player == "river" {
                        if let Some(card) = node.get("cardDisplay").and_then(|c| c.as_str()) {
                            summary.push_str(&format!("River: {} | River actions: ", card));
                        }
                    }
                }
            } else if node_type == "player" {
                // 玩家动作
                let player = node.get("player").and_then(|p| p.as_str()).unwrap_or("unknown");
                let action_name = node.get("action").and_then(|a| a.get("name")).and_then(|n| n.as_str()).unwrap_or("unknown");
                let action_amount = node.get("action").and_then(|a| a.get("amount")).and_then(|n| n.as_str()).unwrap_or("0");
                
                let action_str = if action_name == "Check" || action_name == "Fold" || action_name == "Call" {
                    action_name.to_string()
                } else {
                    format!("{} {}", action_name, action_amount)
                };
                
                summary.push_str(&format!("{} {} | ", player.to_uppercase(), action_str));
            } else if node_type == "terminal" {
                // 终端节点
                let pot = node.get("pot").and_then(|p| p.as_i64()).unwrap_or(0) as i32;
                
                let eq_oop = node.get("equityOop").and_then(|e| e.as_f64()).unwrap_or(0.5);
                let eq_ip = node.get("equityIp").and_then(|e| e.as_f64()).unwrap_or(0.5);
                
                summary.push_str(&format!("Final pot: {} | Equity: OOP {:.2}%, IP {:.2}%", 
                    pot, eq_oop * 100.0, eq_ip * 100.0));
            }
        }
    }
    
    summary
}

// 辅助函数: 获取特定手牌的索引
fn get_hand_indices(hand: &str, game: &PostFlopGame) -> Vec<usize> {
    // 获取游戏中的私有牌数据
    let private_cards = game.private_cards(0); // 使用OOP玩家的牌，因为目前只处理OOP的手牌
    
    // 匹配逻辑 - 这里需要根据具体的手牌表示方法调整
    let indices: Vec<usize> = match hand {
        // AA - 任何A对子
        "AA" => private_cards.iter()
            .enumerate()
            .filter(|(_, &(c1, c2))| {
                let r1 = c1 / 4;  // 牌的等级 (0-12)
                let r2 = c2 / 4;
                r1 == 12 && r2 == 12  // A的等级是12
            })
            .map(|(i, _)| i)
            .collect(),
            
        // 特定花色的AA
        "A♠A♥" | "AsAh" => private_cards.iter()
            .enumerate()
            .filter(|(_, &(c1, c2))| {
                let r1 = c1 / 4;
                let s1 = c1 % 4;
                let r2 = c2 / 4;
                let s2 = c2 % 4;
                (r1 == 12 && r2 == 12) && ((s1 == 3 && s2 == 2) || (s1 == 2 && s2 == 3))  // A♠(3)A♥(2)
            })
            .map(|(i, _)| i)
            .collect(),
            
        "A♠A♦" | "AsAd" => private_cards.iter()
            .enumerate()
            .filter(|(_, &(c1, c2))| {
                let r1 = c1 / 4;
                let s1 = c1 % 4;
                let r2 = c2 / 4;
                let s2 = c2 % 4;
                (r1 == 12 && r2 == 12) && ((s1 == 3 && s2 == 1) || (s1 == 1 && s2 == 3))  // A♠(3)A♦(1)
            })
            .map(|(i, _)| i)
            .collect(),
            
        "A♠A♣" | "AsAc" => private_cards.iter()
            .enumerate()
            .filter(|(_, &(c1, c2))| {
                let r1 = c1 / 4;
                let s1 = c1 % 4;
                let r2 = c2 / 4;
                let s2 = c2 % 4;
                (r1 == 12 && r2 == 12) && ((s1 == 3 && s2 == 0) || (s1 == 0 && s2 == 3))  // A♠(3)A♣(0)
            })
            .map(|(i, _)| i)
            .collect(),
            
        "A♥A♦" | "AhAd" => private_cards.iter()
            .enumerate()
            .filter(|(_, &(c1, c2))| {
                let r1 = c1 / 4;
                let s1 = c1 % 4;
                let r2 = c2 / 4;
                let s2 = c2 % 4;
                (r1 == 12 && r2 == 12) && ((s1 == 2 && s2 == 1) || (s1 == 1 && s2 == 2))  // A♥(2)A♦(1)
            })
            .map(|(i, _)| i)
            .collect(),
            
        "A♥A♣" | "AhAc" => private_cards.iter()
            .enumerate()
            .filter(|(_, &(c1, c2))| {
                let r1 = c1 / 4;
                let s1 = c1 % 4;
                let r2 = c2 / 4;
                let s2 = c2 % 4;
                (r1 == 12 && r2 == 12) && ((s1 == 2 && s2 == 0) || (s1 == 0 && s2 == 2))  // A♥(2)A♣(0)
            })
            .map(|(i, _)| i)
            .collect(),
            
        "A♦A♣" | "AdAc" => private_cards.iter()
            .enumerate()
            .filter(|(_, &(c1, c2))| {
                let r1 = c1 / 4;
                let s1 = c1 % 4;
                let r2 = c2 / 4;
                let s2 = c2 % 4;
                (r1 == 12 && r2 == 12) && ((s1 == 1 && s2 == 0) || (s1 == 0 && s2 == 1))  // A♦(1)A♣(0)
            })
            .map(|(i, _)| i)
            .collect(),
            
        // 其他手牌可以根据需要添加
        _ => Vec::new(),
    };
    
    indices
}
