#![cfg(feature = "legacy_api_unfinished")]

use crate::api_data::errors::ApiError;
use anyhow::anyhow;
use reqwest::Client;
use reqwest::header::AUTHORIZATION;
use serde::de::DeserializeOwned;
use serde_json::Value;

const LEGACY_BASE_URL: &str = "";

pub struct LegacyApi<'a> {
    token: &'a str,
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
        Self {
            token: bearer_token,
            client,
        }
    }

    pub async fn fetch<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        extra_params: Option<Vec<(&str, String)>>,
    ) -> anyhow::Result<T> {
        let base_url = format!("{}{}", LEGACY_BASE_URL, endpoint);

        let mut params = vec![];

        if let Some(extra) = extra_params {
            for (key, value) in extra {
                params.push((key.to_string(), value));
            }
        }

        let resp = self
            .client
            .get(&base_url)
            .header(AUTHORIZATION, self.token)
            .query(&params)
            .send()
            .await?;

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
