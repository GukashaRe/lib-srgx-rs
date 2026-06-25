// src/legacy_api/school_comments.rs

use serde::{Deserialize, Serialize};

// ... 已有的结构体 ...

/// 学校搜索结果响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchoolSearchResponse {
    pub data: Vec<School>,
    pub total: i64,
    pub page: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
}

/// 学校信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct School {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub province: Option<String>,
    pub city: Option<String>,
    pub logo_path: Option<String>,
    pub tags: Option<String>,
    pub ranking: Option<i64>,
    pub rating: Option<f64>,
    pub review_count: Option<i64>,
    pub verified_count: Option<i64>,
    pub motto: Option<String>,
    pub dormitory: Option<f64>,
    pub cafeteria: Option<f64>,
    pub faculty: Option<f64>,
    pub environment: Option<f64>,
    pub culture: Option<f64>,
    pub employment: Option<f64>,
    pub safety: Option<f64>,
}
