pub mod api_data;

use anyhow::{Result, anyhow};
use api_data::errors::ApiError;
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
    /// # 示例
    /// ```
    /// use lib_srgx_rs::SrgxImpl;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = SrgxImpl::new("token".to_string(), "code".to_string());
    /// let response = client.send_request::<serde_json::Value>("/api/query", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_request<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        extra_params: Option<Vec<(&str, String)>>,
    ) -> Result<T> {
        let base_url = format!("{}{}", BASE_URL, endpoint);

        // 构建查询参数，使用 (String, String) 避免生命周期问题
        let mut params: Vec<(String, String)> = vec![
            ("code".to_string(), self.code.clone()),
            ("api_key".to_string(), self.api_token.clone()),
        ];

        if let Some(extra) = extra_params {
            for (key, value) in extra {
                params.push((key.to_string(), value));
            }
        }

        let resp = self.client.get(&base_url).query(&params).send().await?;

        let status = resp.status();
        let text = resp.text().await?;

        // 处理 HTTP 错误（4xx, 5xx）
        if !status.is_success() {
            if let Ok(val) = serde_json::from_str::<Value>(&text) {
                if let Some(msg) = val.get("message").and_then(|m| m.as_str()) {
                    return Err(anyhow!("HTTP错误 ({}): {}", status, msg));
                }
            }
            return Err(anyhow!("HTTP错误 ({}): {}", status, text));
        }

        // 处理业务错误（HTTP 200 但 success=false）
        if let Ok(val) = serde_json::from_str::<Value>(&text) {
            if let Some(success) = val.get("success").and_then(|s| s.as_bool()) {
                if !success {
                    let error_msg = val
                        .get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("未知业务错误");
                    return Err(anyhow!("业务错误: {}", ApiError::from_message(error_msg)));
                }
            }
        }

        // 成功：反序列化为目标类型
        Ok(serde_json::from_str::<T>(&text)?)
    }

    /// 查询学历信息
    ///
    /// # 示例
    /// ```
    /// use lib_srgx_rs::SrgxImpl;
    /// use lib_srgx_rs::api_data::QueryResponse;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = SrgxImpl::new("your_token".to_string(), "your_code".to_string());
    /// let response = client.get_query().await?;
    ///
    /// if let Some(data) = response.success_data() {
    ///     println!("姓名: {}, 学校: {}", data.name, data.school_name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_query(&self) -> Result<api_data::QueryResponse> {
        self.send_request("/api/query", None).await
    }
}
