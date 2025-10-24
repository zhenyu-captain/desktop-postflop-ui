#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod bunching;
mod range;
mod solver;
mod test;
mod tree;
mod version_check;
mod network_check;
use crate::bunching::*;
use crate::range::*;
use crate::solver::*;
use crate::test::*;
use crate::tree::*;
use crate::version_check::*;
use crate::network_check::*;

use postflop_solver::*;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::sync::Mutex;
use sysinfo::{System, SystemExt};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            // 不自动打开DevTools，只通过快捷键调用
            println!("应用启动成功，按 Cmd+Shift+I 或 F12 打开DevTools");
            Ok(())
        })
        .manage(Mutex::new(RangeManager::default()))
        .manage(Mutex::new(default_action_tree()))
        .manage(Mutex::new(None as Option<BunchingData>))
        .manage(Mutex::new(PostFlopGame::default()))
        .manage(Mutex::new(ThreadPoolBuilder::new().build().unwrap()))
        .invoke_handler(tauri::generate_handler![
            os_name,
            memory,
            set_num_threads,
            range_num_combos,
            range_clear,
            range_invert,
            range_update,
            range_from_string,
            range_to_string,
            range_get_weights,
            range_raw_data,
            tree_new,
            tree_added_lines,
            tree_removed_lines,
            tree_invalid_terminals,
            tree_actions,
            tree_is_terminal_node,
            tree_is_chance_node,
            tree_back_to_root,
            tree_apply_history,
            tree_play,
            tree_total_bet_amount,
            tree_add_bet_action,
            tree_remove_current_node,
            tree_delete_added_line,
            tree_delete_removed_line,
            bunching_init,
            bunching_clear,
            bunching_progress,
            game_init,
            game_private_cards,
            game_memory_usage,
            game_memory_usage_bunching,
            game_allocate_memory,
            game_set_bunching,
            game_solve_step,
            game_exploitability,
            game_finalize,
            game_apply_history,
            game_total_bet_amount,
            game_actions_after,
            game_possible_cards,
            game_get_results,
            game_get_chance_reports,
            game_save_to_bin,
            game_load_from_bin,
            game_get_config_info,
            get_decision_tree,
            get_current_node_with_children,
            get_complete_decision_path,
            log_to_console,
            save_config_to_file,
            load_config_from_file,
            write_log_to_file,
            get_log_file_path,
            toggle_devtools,
            check_network_connectivity,
            check_version_control_status,
            exit_application
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(target_os = "windows")]
#[tauri::command]
fn os_name() -> String {
    "windows".to_string()
}

#[cfg(target_os = "macos")]
#[tauri::command]
fn os_name() -> String {
    "macos".to_string()
}

#[cfg(target_os = "linux")]
#[tauri::command]
fn os_name() -> String {
    "linux".to_string()
}

#[tauri::command]
fn memory() -> (u64, u64) {
    let mut system = System::new_all();
    system.refresh_memory();
    (system.available_memory(), system.total_memory())
}

#[tauri::command]
fn set_num_threads(pool_state: tauri::State<Mutex<ThreadPool>>, num_threads: usize) {
    *pool_state.lock().unwrap() = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();
}

/// 从前端接收数据并打印到Rust控制台
#[tauri::command]
fn log_to_console(message: String) {
    println!("\n====== 前端数据打印 ======");
    println!("{}", message);
    println!("====== 前端数据结束 ======\n");
}

#[tauri::command]
fn save_config_to_file(file_path: String, config_data: String) -> Result<(), String> {
    std::fs::write(&file_path, config_data)
        .map_err(|e| format!("Failed to save config file: {}", e))
}

#[tauri::command]
fn load_config_from_file(file_path: String) -> Result<String, String> {
    std::fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to load config file: {}", e))
}

/// 获取日志文件路径
#[tauri::command]
fn get_log_file_path() -> Result<String, String> {
    // 获取应用数据目录
    let app_data_dir = dirs::data_dir()
        .ok_or("无法获取应用数据目录")?
        .join("desktop-postflop-ui")
        .join("logs");
    
    // 确保目录存在
    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("无法创建日志目录: {}", e))?;
    
    // 生成日志文件名（包含日期）
    let now = chrono::Local::now();
    let log_filename = format!("solver-{}.log", now.format("%Y-%m-%d"));
    let log_path = app_data_dir.join(log_filename);
    
    Ok(log_path.to_string_lossy().to_string())
}

/// 写入日志到文件
#[tauri::command]
fn write_log_to_file(log_message: String) -> Result<(), String> {
    use std::fs::OpenOptions;
    use std::io::Write;
    
    let log_path = get_log_file_path()?;
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .map_err(|e| format!("无法打开日志文件: {}", e))?;
    
    writeln!(file, "{}", log_message)
        .map_err(|e| format!("无法写入日志文件: {}", e))?;
    
    Ok(())
}

/// 切换开发者工具
#[tauri::command]
fn toggle_devtools(app: tauri::AppHandle) {
    if let Some(window) = app.get_window("main") {
        if window.is_devtools_open() {
            window.close_devtools();
            println!("DevTools已关闭");
        } else {
            window.open_devtools();
            println!("DevTools已打开");
        }
    }
}

/// 检查网络连接状态
#[tauri::command]
async fn check_network_connectivity() -> Result<NetworkCheckResult, String> {
    println!("开始检查网络连接状态...");
    let result = crate::network_check::check_network_connectivity().await;
    println!("网络检查完成: {:?}", result);
    Ok(result)
}

/// 检查版本控制状态
#[tauri::command]
async fn check_version_control_status() -> Result<VersionCheckResult, String> {
    println!("开始检查版本控制状态...");
    let result = check_version_control().await;
    println!("版本检查完成: {:?}", result);
    Ok(result)
}

/// 退出应用程序
#[tauri::command]
fn exit_application(app: tauri::AppHandle) {
    println!("免费版本已停用，正在退出应用程序...");
    app.exit(0);
}

