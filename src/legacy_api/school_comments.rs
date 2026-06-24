use serde::{Deserialize, Deserializer, Serialize};

/// 将 0/1 反序列化为 bool
fn deserialize_bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value = i64::deserialize(deserializer)?;
    Ok(value == 1)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Root {
    pub total: i64,
    pub page: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    #[serde(rename = "totalPages")]
    pub total_pages: i64,
    pub campuses: Vec<CampusesItem>,
    #[serde(rename = "activeCampus")]
    pub active_campus: Option<serde_json::Value>,
    pub data: Vec<DataItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampusesItem {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataItem {
    pub id: i64,
    pub user_id: i64,
    pub school_id: i64,
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub is_anonymous: bool,
    pub status: String,
    pub reject_reason: Option<serde_json::Value>,
    pub content: String,
    pub images: Option<serde_json::Value>,
    pub display_name: String,
    pub created_at: String,
    pub updated_at: String,
    pub view_count: i64,
    pub submission_id: Option<serde_json::Value>,
    pub mod_task_id: Option<String>,
    pub mod_suggestion: String,
    pub campus_id: Option<i64>,
    pub like_count: i64,
    pub reply_count: i64,
    pub review_weight: f64,
    pub is_question: i64,
    pub nickname: String,
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub is_verified: bool,
    pub verified_school_id: Option<serde_json::Value>,
    pub avatar_path: Option<serde_json::Value>,
    pub level: Option<serde_json::Value>,
    pub title: Option<serde_json::Value>,
    pub campus_name: Option<String>,
    pub user_liked: bool,
    pub rating: Option<Rating>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rating {
    pub id: i64,
    pub review_id: i64,
    pub dormitory: Option<f64>, // ✅ 全部改为 Option<f64>
    pub cafeteria: Option<f64>,
    pub faculty: Option<f64>,
    pub environment: Option<f64>,
    pub culture: Option<f64>,
    pub employment: Option<f64>,
    pub safety: Option<f64>,
}
