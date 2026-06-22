use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::Value;

const BASE_URL: &str = "https://srgaoxiao.online";

#[derive(Debug)]
pub struct SrgxImpl {
    pub api_token: String,
    pub code: String,
    pub client: Client,
}

impl SrgxImpl {
    pub fn new(api_token: String, code: String) -> Self {
        let client = Client::builder()
            .user_agent("Gukasha-lib-srgx-rs/beta")
            .https_only(true)
            .build()
            .unwrap();
        Self {
            api_token,
            code,
            client,
        }
    }

    /// 统一请求方法：处理所有公共逻辑
    ///
    /// # 参数
    /// - `endpoint`: API路径，如 "/api/query"
    /// - `params`: 额外的URL参数（可选）
    ///
    /// # 返回
    /// - `Ok(T)`: 成功反序列化的数据
    /// - `Err(anyhow::Error)`: HTTP错误、业务错误、解析错误
    pub async fn send_request<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: Option<Vec<(&str, String)>>,
    ) -> Result<T> {
        // 构建基础URL
        let mut url = format!(
            "{}{}?code={}&api_key={}",
            BASE_URL, endpoint, self.code, self.api_token
        );

        // 添加额外参数
        if let Some(params) = params {
            for (key, value) in params {
                url.push_str(&format!("&{}={}", key, value));
            }
        }

        // 发送请求
        let resp = self.client.get(&url).send().await?;
        let status = resp.status();
        let text = resp.text().await?;

        // 1. 处理 HTTP 错误（4xx, 5xx）
        if !status.is_success() {
            if let Ok(val) = serde_json::from_str::<Value>(&text) {
                if let Some(msg) = val.get("message").and_then(|m| m.as_str()) {
                    return Err(anyhow!("HTTP错误 ({}): {}", status, msg));
                }
            }
            return Err(anyhow!("HTTP错误 ({}): {}", status, text));
        }

        // 2. 处理业务错误（HTTP 200 但 success=false）
        if let Ok(val) = serde_json::from_str::<Value>(&text) {
            if let Some(success) = val.get("success").and_then(|s| s.as_bool()) {
                if !success {
                    let error_msg = val
                        .get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("未知业务错误");
                    return Err(anyhow!("业务错误: {}", error_msg));
                }
            }
        }

        // 3. 真正的成功：反序列化为目标类型
        Ok(serde_json::from_str::<T>(&text)?)
    }
}
