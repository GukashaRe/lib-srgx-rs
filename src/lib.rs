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

        // 检查HTTP状态码
        if !resp.status().is_success() {
            let ori_text = resp.text().await?;

            // 尝试解析错误信息
            if let Ok(val) = serde_json::from_str::<Value>(&ori_text) {
                if let Some(msg) = val.get("message").and_then(|m| m.as_str()) {
                    return Err(anyhow!("API请求失败: {}", msg));
                }
            }

            // 如果无法解析，返回原始文本
            return Err(anyhow!("API请求失败: {}", ori_text));
        }
        Ok(resp.json::<T>().await?)
    }
}
