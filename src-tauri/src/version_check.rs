use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionControlResponse {
    pub free: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionCheckResult {
    pub is_shutdown: bool,
    pub error: Option<String>,
    pub response: Option<VersionControlResponse>,
}

/// 检查版本控制信息
pub async fn check_version_control() -> VersionCheckResult {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap_or_else(|e| {
            eprintln!("创建HTTP客户端失败: {}", e);
            reqwest::Client::new()
        });

    match client
        .get("https://gtosim.com/version_control.json")
        .header("Accept", "application/json")
        .header("Cache-Control", "no-cache")
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            println!("版本检查HTTP状态码: {}", status);
            
            if !status.is_success() {
                let error_msg = format!("HTTP错误: {}", status);
                println!("版本检查失败: {}", error_msg);
                return VersionCheckResult {
                    is_shutdown: false,
                    error: Some(error_msg),
                    response: None,
                };
            }

            match response.json::<VersionControlResponse>().await {
                Ok(data) => {
                    println!("版本检查响应: {:?}", data);
                    let is_shutdown = data.free == "shutdown";
                    println!("免费版本是否停用: {}", is_shutdown);
                    
                    VersionCheckResult {
                        is_shutdown,
                        error: None,
                        response: Some(data),
                    }
                }
                Err(e) => {
                    let error_msg = format!("解析JSON失败: {}", e);
                    println!("版本检查失败: {}", error_msg);
                    VersionCheckResult {
                        is_shutdown: false,
                        error: Some(error_msg),
                        response: None,
                    }
                }
            }
        }
        Err(e) => {
            let error_msg = format!("网络请求失败: {}", e);
            println!("版本检查失败: {}", error_msg);
            VersionCheckResult {
                is_shutdown: false,
                error: Some(error_msg),
                response: None,
            }
        }
    }
}
