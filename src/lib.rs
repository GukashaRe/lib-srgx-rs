pub mod api_data;

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
    pub async fn get_query<T: DeserializeOwned>(&self) -> Result<T> {
        let url = format!(
            "{}/api/query?code={}&api_key={}",
            BASE_URL, self.code, self.api_token
        );

        let resp = self.client.get(&url).send().await?;
        let status = resp.status();
        let text = resp.text().await?;

        // 1. 处理 HTTP 错误
        if !status.is_success() {
            // 尝试提取错误信息
            if let Ok(val) = serde_json::from_str::<Value>(&text) {
                if let Some(msg) = val.get("message").and_then(|m| m.as_str()) {
                    return Err(anyhow!("HTTP错误 ({}): {}", status, msg));
                }
            }
            return Err(anyhow!("HTTP错误 ({}): {}", status, text));
        }

        // 2. ⭐ 关键：处理 HTTP 200 但业务失败的情况
        //    这里不直接返回错误，而是让调用者知道"业务失败"的具体原因
        if let Ok(val) = serde_json::from_str::<Value>(&text) {
            if let Some(success) = val.get("success").and_then(|s| s.as_bool()) {
                if !success {
                    // 提取业务错误信息
                    let error_msg = val
                        .get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("未知业务错误");

                    // ⭐ 向上传递业务错误
                    return Err(anyhow!("业务错误: {}", error_msg));
                }
            }
        }

        // 3. 真正的成功：反序列化为目标类型
        Ok(serde_json::from_str::<T>(&text)?)
    }
}
