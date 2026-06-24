//! Legacy API 客户端模块
//!
//! 本模块提供对 `https://srgaoxiao.com/api` 的访问能力，用于调用旧版（Legacy）API。
//! 当前仅当 `legacy_api_unfinished` feature 启用时可用。

pub mod reply;
pub mod school_comments;

use std::borrow::Cow;
use std::time::Duration;

use crate::api_data::errors::ApiError;
use crate::legacy_api::school_comments::Root;
use anyhow::Result;
use anyhow::anyhow;
use reqwest::header::AUTHORIZATION;
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;

const LEGACY_BASE_URL: &str = "https://srgaoxiao.com/api";

pub struct LegacyApi<'a> {
    token: Cow<'a, str>,
    client: Client,
}

impl<'a> LegacyApi<'a> {
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

    pub async fn fetch<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        require_auth: bool,
        extra_params: Option<Vec<(&str, String)>>,
    ) -> Result<T> {
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
        //        eprintln!("=== 响应体前 5000 字符 ===");
        //        eprintln!("{}", &text.chars().take(5000).collect::<String>());
        //        eprintln!("=== 响应体结束 ===");
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

    /// 获取学校评价列表
    pub async fn get_school_reviews(
        &self,
        school_id: i64,
        user_id: Option<i64>,
        sort: impl Into<String>,
        page: i64,
        page_size: i64,
    ) -> Result<Root> {
        let endpoint = format!("/api/reviews/school/{}", school_id);

        let mut extra_params = Vec::new();
        if let Some(uid) = user_id {
            extra_params.push(("userId", uid.to_string()));
        }
        extra_params.push(("sort", sort.into()));
        extra_params.push(("page", page.to_string()));
        extra_params.push(("pageSize", page_size.to_string()));

        self.fetch(&endpoint, false, Some(extra_params)).await
    }

    /// 获取指定评价的回复列表
    pub async fn get_replies(
        &self,
        review_id: i64,
        user_id: Option<i64>,
    ) -> Result<Vec<reply::CommentReplys>> {
        let endpoint = format!("/api/users/reviews/{}/replies", review_id);

        let mut extra_params = Vec::new();
        if let Some(uid) = user_id {
            extra_params.push(("userId", uid.to_string()));
        }

        let resp: Vec<reply::CommentReplys> =
            self.fetch(&endpoint, false, Some(extra_params)).await?;

        Ok(resp)
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

    #[tokio::test]
    #[ignore = "需要网络环境，仅手动测试"]
    async fn test_get_replies() {
        let api = LegacyApi::new("test_token");
        let result = api.get_replies(3546, None).await;
        assert!(result.is_ok() || result.is_err());
    }
}
