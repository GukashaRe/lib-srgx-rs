// src/legacy_api/school_list.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;

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

/// 学校缓存管理器
#[derive(Debug, Clone)]
pub struct SchoolCache {
    id_map: HashMap<i64, School>,
    schools: Vec<School>,
}

impl SchoolCache {
    pub async fn load_or_fetch(
        api: &crate::legacy_api::LegacyApi<'_>,
        cache_path: Option<PathBuf>,
    ) -> Result<Self, anyhow::Error> {
        let cache_path = cache_path.unwrap_or_else(|| {
            let mut path = std::env::temp_dir();
            path.push("srgx_school_cache.json");
            path
        });

        if let Ok(cache) = Self::load_from_file(&cache_path) {
            eprintln!("📂 从缓存加载 {} 所学校", cache.len());
            return Ok(cache);
        }

        eprintln!("🌐 首次启动，从 API 拉取学校列表...");
        let cache = Self::fetch_all_schools(api).await?;
        eprintln!("✅ 成功拉取 {} 所学校", cache.len());

        if let Err(e) = cache.save_to_file(&cache_path) {
            eprintln!("⚠️ 保存缓存失败: {}", e);
        } else {
            eprintln!("💾 缓存已保存到: {:?}", cache_path);
        }

        Ok(cache)
    }

    pub fn load_from_file(path: &PathBuf) -> Result<Self, anyhow::Error> {
        let content = fs::read_to_string(path)?;
        let schools: Vec<School> = serde_json::from_str(&content)?;
        Ok(Self::from_schools(schools))
    }

    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), anyhow::Error> {
        let content = serde_json::to_string_pretty(&self.schools)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn from_schools(schools: Vec<School>) -> Self {
        let mut id_map = HashMap::new();
        for school in &schools {
            id_map.insert(school.id, school.clone());
        }
        Self { id_map, schools }
    }

    async fn fetch_all_schools(
        api: &crate::legacy_api::LegacyApi<'_>,
    ) -> Result<Self, anyhow::Error> {
        let mut all_schools = Vec::new();
        let mut page = 1;
        let page_size = 100_usize;
        let mut fetch_data_len = 0;

        loop {
            print!(
                "\r当前正在拉取第 {} 页，已拉取 {} 条数据\n",
                page, fetch_data_len
            );
            std::io::stdout().flush()?;
            let resp: SchoolSearchResponse = api.search_schools("", page, page_size as i64).await?;

            if resp.data.is_empty() {
                break;
            }

            let data_len = resp.data.len();
            all_schools.extend(resp.data);

            if data_len < page_size {
                break;
            }
            page += 1;
            fetch_data_len += data_len;
            sleep(Duration::from_millis(275)).await;
        }

        Ok(Self::from_schools(all_schools))
    }

    pub fn search_exact(&self, keyword: &str) -> Option<&School> {
        self.schools.iter().find(|s| s.name == keyword)
    }

    pub fn search_fuzzy(&self, keyword: &str, limit: usize) -> Vec<&School> {
        let keyword = keyword.trim();
        if keyword.is_empty() {
            return self.schools.iter().take(limit).collect();
        }

        let keyword_lower = keyword.to_lowercase();
        let mut results: Vec<(&School, i32)> = Vec::new();

        for school in &self.schools {
            let name_lower = school.name.to_lowercase();
            let mut score = 0;

            if name_lower == keyword_lower {
                score += 1000;
            } else if name_lower.starts_with(&keyword_lower) {
                score += 100;
            } else if name_lower.contains(&keyword_lower) {
                score += 10;
            } else {
                let initials: String = name_lower
                    .chars()
                    .filter(|c| c.is_ascii_alphabetic())
                    .map(|c| c.to_ascii_lowercase())
                    .collect();
                if initials.contains(&keyword_lower) {
                    score += 5;
                }
            }

            if let Some(tags) = &school.tags
                && tags.to_lowercase().contains(&keyword_lower)
            {
                score += 3;
            }

            if score > 0 {
                results.push((school, score));
            }
        }

        results.sort_by_key(|b| std::cmp::Reverse(b.1));
        results
            .iter()
            .take(limit)
            .map(|(school, _)| *school)
            .collect()
    }

    pub fn all_schools(&self) -> &[School] {
        &self.schools
    }

    pub fn len(&self) -> usize {
        self.schools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.schools.is_empty()
    }

    pub fn get_by_id(&self, id: i64) -> Option<&School> {
        self.id_map.get(&id)
    }

    pub fn search_multiple(&self, keywords: &[&str], limit: usize) -> Vec<&School> {
        let mut results = Vec::new();
        for keyword in keywords {
            let mut matches = self.search_fuzzy(keyword, limit);
            results.append(&mut matches);
        }
        let mut seen = std::collections::HashSet::new();
        results.retain(|s| seen.insert(s.id));
        results
    }

    pub fn search_and_display(&self, keyword: &str, limit: usize) {
        let results = self.search_fuzzy(keyword, limit);
        if results.is_empty() {
            println!("未找到匹配的学校");
            return;
        }

        println!("找到 {} 所匹配学校:", results.len());
        for (i, school) in results.iter().enumerate() {
            let rating_str = school
                .rating
                .map(|r| format!("{:.1}", r))
                .unwrap_or_else(|| "暂无".to_string());
            println!(
                "  {}. {} (ID: {}) 评分: {} 评价: {}",
                i + 1,
                school.name,
                school.id,
                rating_str,
                school.review_count.unwrap_or(0)
            );
        }
    }
}
