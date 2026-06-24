#![cfg(feature = "legacy_api_unfinished")]

pub mod reply;

use std::borrow::Cow;

use crate::api_data::errors::ApiError;
use anyhow::anyhow;
use reqwest::Client;
use reqwest::header::AUTHORIZATION;
use serde::de::DeserializeOwned;
use serde_json::Value;

const LEGACY_BASE_URL: &str = "";

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
            .user_agent("")
            .https_only(true)
            .build()
            .unwrap();

        // 在创建时拼接 Bearer 前缀，用 Cow 统一管理
        let full_token = Cow::Owned(format!("Bearer {}", bearer_token));

        Self {
            token: full_token,
            client,
        }
    }

    pub async fn fetch<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        no_auth: bool,
        extra_params: Option<Vec<(&str, String)>>,
    ) -> anyhow::Result<T> {
        let base_url = format!("{}{}", LEGACY_BASE_URL, endpoint);

        let mut params: Vec<(String, String)> = vec![];

        if let Some(extra) = extra_params {
            for (key, value) in extra {
                params.push((key.to_string(), value));
            }
        }

        let mut request_builder = self.client.get(&base_url).query(&params);

        if !no_auth {
            // 直接使用已经包含 Bearer 前缀的 token
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

        Ok(serde_json::from_str::<T>(&text)?)
    }
}
