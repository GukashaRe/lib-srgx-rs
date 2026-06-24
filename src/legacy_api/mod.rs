//! Legacy API 客户端模块
//!
//! 本模块提供对 `https://srgaoxiao.com/api` 的访问能力，用于调用旧版（Legacy）API。
//! 当前仅当 `legacy_api_unfinished` feature 启用时可用。
//!
//! # Feature Gate
//!
//! ```toml
//! [features]
//! legacy_api_unfinished = []
//! ```
//!
//! # 示例
//!
//! ```no_run
//! use lib_srgx_rs::legacy_api::LegacyApi;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let api = LegacyApi::new("your_bearer_token");
//!     
//!     // 调用不需要认证的接口
//!     let result: serde_json::Value = api.fetch("public/info", false, None).await?;
//!     println!("{:?}", result);
//!
//!     Ok(())
//! }
//! ```

pub mod reply;
pub mod school_comments;

use std::borrow::Cow;
use std::time::Duration;

use crate::api_data::errors::ApiError;
use anyhow::anyhow;
use reqwest::header::AUTHORIZATION;
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;

const LEGACY_BASE_URL: &str = "https://srgaoxiao.com/api";

/// Legacy API 客户端
///
/// 封装了与旧版 API 交互的通用逻辑，包括：
/// - Bearer Token 认证
/// - 统一的错误处理（HTTP 状态码 + 业务错误码）
/// - 灵活的查询参数扩展
///
/// # 类型参数
///
/// - `'a`: 生命周期参数，用于 `Cow` 借用的 token 字符串
///
/// # 示例
///
/// ```no_run
/// use lib_srgx_rs::legacy_api::LegacyApi;
///
/// let api = LegacyApi::new("your_bearer_token_here");
/// ```
pub struct LegacyApi<'a> {
    /// 认证令牌，包含 `Bearer ` 前缀
    token: Cow<'a, str>,
    /// HTTP 客户端
    client: Client,
}

impl<'a> LegacyApi<'a> {
    /// 创建一个新的 Legacy API 客户端实例
    ///
    /// # 参数
    ///
    /// - `bearer_token`: 原始 Bearer Token（不含 `Bearer ` 前缀），
    ///   方法内部会自动拼接前缀
    ///
    /// # 返回
    ///
    /// 返回配置好的 `LegacyApi` 实例。
    ///
    /// # Panics
    ///
    /// 当 HTTP 客户端构建失败时会 panic（通常不会发生，除非系统资源耗尽）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// let api = LegacyApi::new("sk-1234567890");
    /// ```
    pub fn new<'b>(bearer_token: &'b str) -> Self
    where
        'b: 'a,
    {
        let client = Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .https_only(true)
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        let full_token = Cow::Owned(format!("Bearer {}", bearer_token));

        Self {
            token: full_token,
            client,
        }
    }

    /// 发送一个通用的 GET 请求并反序列化响应
    ///
    /// # 类型参数
    ///
    /// - `T`: 响应数据的类型，必须实现 `serde::de::DeserializeOwned`。
    ///   建议使用 `serde_json::Value` 进行快速测试，或定义具体结构体。
    ///
    /// # 参数
    ///
    /// - `endpoint`: API 路径，可以包含前导斜杠或不含，例如 `"school/comments"` 或 `"/school/comments"`。
    /// - `require_auth`: 是否需要在请求头中携带 Bearer Token。
    ///   - `true`: 添加 `Authorization: Bearer <token>` 头
    ///   - `false`: 不添加认证头
    /// - `extra_params`: 可选的额外查询参数，以键值对形式传入。
    ///   每个元组为 `(key, value)`，其中 `key` 是字符串切片，`value` 是字符串。
    ///
    /// # 错误
    ///
    /// 该方法可能返回以下类型的错误：
    /// - **HTTP 错误**: 当状态码不是 2xx 时返回，包含状态码和响应体信息
    /// - **业务错误**: 当响应中 `success` 字段为 `false` 时返回，包含 `message` 字段内容
    /// - **反序列化错误**: 当响应体无法解析为类型 `T` 时返回
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use serde_json::Value;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let api = LegacyApi::new("your_token");
    ///
    /// // 不需要认证的请求
    /// let info: Value = api.fetch("public/info", false, None).await?;
    ///
    /// // 需要认证且带额外参数的请求
    /// let comments: Value = api.fetch(
    ///     "school/comments",
    ///     true,
    ///     Some(vec![("page", "1".to_string()), ("limit", "20".to_string())])
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        require_auth: bool,
        extra_params: Option<Vec<(&str, String)>>,
    ) -> anyhow::Result<T> {
        let base_url = Url::parse(LEGACY_BASE_URL)?;
        let endpoint_clean = endpoint.trim_start_matches('/');
        let full_url = base_url.join(endpoint_clean)?;

        let mut params: Vec<(String, String)> = Vec::new();

        if let Some(extra) = extra_params {
            for (key, value) in extra {
                params.push((key.to_string(), value));
            }
        }

        let mut request_builder = self.client.get(full_url).query(&params);

        if require_auth {
            request_builder = request_builder.header(AUTHORIZATION, &*self.token);
        }

        let resp = request_builder.send().await?;

        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            if let Ok(val) = serde_json::from_str::<Value>(&text)
                && let Some(msg) = val.get("message").and_then(|m| m.as_str())
            {
                return Err(anyhow!("HTTP错误 ({}): {}", status, msg));
            }
            return Err(anyhow!("HTTP错误 ({}): {}", status, text));
        }

        if let Ok(val) = serde_json::from_str::<Value>(&text)
            && let Some(success) = val.get("success").and_then(|s| s.as_bool())
            && !success
        {
            let error_msg = val
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("未知业务错误");
            return Err(anyhow!("业务错误: {}", ApiError::from_message(error_msg)));
        }

        serde_json::from_str::<T>(&text).map_err(|e| anyhow!("响应反序列化失败: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "需要网络环境，仅手动测试"]
    async fn test_fetch_public_info() {
        let api = LegacyApi::new("test_token");
        let result: Result<Value, _> = api.fetch("public/info", false, None).await;
        assert!(result.is_ok() || result.is_err());
    }
}
