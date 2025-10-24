use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkCheckResult {
    pub is_online: bool,
    pub error: Option<String>,
    pub gtosim_com_status: Option<bool>,
    pub www_gtosim_com_status: Option<bool>,
}

/// 检查网络连接状态
pub async fn check_network_connectivity() -> NetworkCheckResult {
    println!("开始网络连接检查...");
    
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap_or_else(|e| {
            eprintln!("创建HTTP客户端失败: {}", e);
            reqwest::Client::new()
        });

    let mut gtosim_com_status = false;
    let mut www_gtosim_com_status = false;
    let mut error_messages = Vec::new();

    // 检查 https://gtosim.com
    println!("正在检查 gtosim.com...");
    match client
        .get("https://gtosim.com")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Cache-Control", "no-cache")
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            gtosim_com_status = status.is_success();
            println!("gtosim.com 响应: 状态码={}, 成功={}", status, gtosim_com_status);
            if !gtosim_com_status {
                error_messages.push(format!("gtosim.com returned status: {}", status));
            }
        }
        Err(e) => {
            println!("gtosim.com 连接失败: {:?}", e);
            error_messages.push(format!("gtosim.com error: {}", e));
        }
    }

    // 检查 https://www.gtosim.com
    println!("正在检查 www.gtosim.com...");
    match client
        .get("https://www.gtosim.com")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Cache-Control", "no-cache")
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            www_gtosim_com_status = status.is_success();
            println!("www.gtosim.com 响应: 状态码={}, 成功={}", status, www_gtosim_com_status);
            if !www_gtosim_com_status {
                error_messages.push(format!("www.gtosim.com returned status: {}", status));
            }
        }
        Err(e) => {
            println!("www.gtosim.com 连接失败: {:?}", e);
            error_messages.push(format!("www.gtosim.com error: {}", e));
        }
    }

    let is_online = gtosim_com_status || www_gtosim_com_status;
    let error_message = if error_messages.is_empty() {
        None
    } else {
        Some(error_messages.join("; "))
    };
    
    println!("网络检测最终结果: 在线={}, gtosim.com={}, www.gtosim.com={}, 错误={:?}", 
             is_online, gtosim_com_status, www_gtosim_com_status, error_message);

    NetworkCheckResult {
        is_online,
        error: error_message,
        gtosim_com_status: Some(gtosim_com_status),
        www_gtosim_com_status: Some(www_gtosim_com_status),
    }
}
