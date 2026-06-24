use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 评价列表响应结构体
///
/// 包含分页信息、校区列表以及评价数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewResponse {
    /// 总记录数
    pub total: i64,
    /// 当前页码
    pub page: i64,
    /// 每页大小
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    /// 总页数
    #[serde(rename = "totalPages")]
    pub total_pages: i64,
    /// 校区列表
    pub campuses: Vec<Campus>,
    /// 当前活跃校区（可能为 null）
    #[serde(rename = "activeCampus")]
    pub active_campus: Option<Value>,
    /// 评价数据列表
    pub data: Vec<ReviewItem>,
}

impl ReviewResponse {
    /// 过滤掉同一个 user_id 的重复评论，只保留每个用户最新的一条评论
    ///
    /// 按 `created_at` 字段判断新旧，保留时间最新的那条。
    ///
    /// # 示例
    /// ```
    /// use lib_srgx_rs::legacy_api::school_comments::ReviewResponse;
    ///
    /// let response: ReviewResponse = serde_json::from_str(json)?;
    /// let filtered = response.filter_unique_users();
    /// assert!(filtered.data.len() <= response.data.len());
    /// ```
    pub fn filter_unique_users(mut self) -> Self {
        let mut user_map: HashMap<i64, ReviewItem> = HashMap::new();
        
        for item in self.data {
            user_map
                .entry(item.user_id)
                .and_modify(|existing| {
                    // 如果当前条目比已存在的更新，则替换
                    if item.created_at > existing.created_at {
                        *existing = item.clone();
                    }
                })
                .or_insert(item);
        }
        
        // 将去重后的数据重新赋值，并保持原有的分页信息不变
        self.data = user_map.into_values().collect();
        self.total = self.data.len() as i64;
        // 更新总页数（向上取整）
        if self.page_size > 0 {
            self.total_pages = (self.total + self.page_size - 1) / self.page_size;
        }
        self
    }
    
    /// 获取某个用户的所有评论
    ///
    /// # 示例
    /// ```
    /// let response: ReviewResponse = serde_json::from_str(json)?;
    /// let user_comments = response.get_user_comments(3275);
    /// ```
    pub fn get_user_comments(&self, user_id: i64) -> Vec<&ReviewItem> {
        self.data
            .iter()
            .filter(|item| item.user_id == user_id)
            .collect()
    }
    
    /// 获取所有唯一的用户 ID 列表
    pub fn unique_user_ids(&self) -> Vec<i64> {
        let mut ids: Vec<i64> = self.data.iter().map(|item| item.user_id).collect();
        ids.sort();
        ids.dedup();
        ids
    }
}

/// 校区信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campus {
    /// 校区 ID
    pub id: i64,
    /// 校区名称
    pub name: String,
}

/// 评价条目
///
/// 包含评价的所有信息，包括用户、学校、内容、点赞等
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewItem {
    /// 评价 ID
    pub id: i64,
    /// 用户 ID
    pub user_id: i64,
    /// 学校 ID
    pub school_id: i64,
    /// 评价状态
    pub status: String,
    /// 拒绝原因（如果被拒绝）
    pub reject_reason: Option<Value>,
    /// 评价内容
    pub content: String,
    /// 图片列表（JSON 数组）
    pub images: Option<Value>,
    /// 显示名称
    pub display_name: String,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
    /// 浏览次数
    pub view_count: i64,
    /// 提交 ID（可能为 null）
    pub submission_id: Option<Value>,
    /// 审核任务 ID
    pub mod_task_id: String,
    /// 审核建议
    pub mod_suggestion: String,
    /// 校区 ID（可能为 null）
    pub campus_id: Option<Value>,
    /// 点赞数
    pub like_count: i64,
    /// 回复数
    pub reply_count: i64,
    /// 评价权重（用于排序）
    pub review_weight: f64,
    /// 用户昵称
    pub nickname: String,
    /// 认证学校 ID（可能为 null）
    pub verified_school_id: Option<Value>,
    /// 头像路径（可能为 null）
    pub avatar_path: Option<Value>,
    /// 用户等级（可能为 null）
    pub level: Option<Value>,
    /// 用户头衔（可能为 null）
    pub title: Option<Value>,
    /// 校区名称（可能为 null）
    pub campus_name: Option<String>,
    /// 是否匿名
    #[serde(rename = "isAnonymous")]
    pub is_anonymous: bool,
    /// 是否认证用户
    #[serde(rename = "isVerified")]
    pub is_verified: bool,
    /// 当前用户是否已点赞
    pub user_liked: bool,
    /// 评分详情（可能为 null）
    pub rating: Option<Rating>,
}

/// 评分详情
///
/// 包含评价的各方面评分
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rating {
    /// 评分 ID
    pub id: i64,
    /// 关联的评价 ID
    pub review_id: i64,
    /// 宿舍评分（1-5 分）
    pub dormitory: Option<i64>,
    /// 食堂评分（1-5 分）
    pub cafeteria: Option<i64>,
    /// 师资评分（1-5 分）
    pub faculty: Option<i64>,
    /// 环境评分（1-5 分）
    pub environment: Option<i64>,
    /// 文化氛围评分（1-5 分）
    pub culture: Option<i64>,
    /// 就业前景评分（1-5 分）
    pub employment: Option<i64>,
    /// 安全状况评分（1-5 分）
    pub safety: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reply_deserde() {
        let json = r#"{
    "total": 14,
    "page": 1,
    "pageSize": 10,
    "totalPages": 2,
    "campuses": [
        {"id": 1131, "name": "东校区"},
        {"id": 1132, "name": "师范分院校区"},
        {"id": 1133, "name": "南校区"},
        {"id": 1134, "name": "北校区"}
    ],
    "activeCampus": null,
    "data": [
        {
            "id": 2114,
            "user_id": 3275,
            "school_id": 457,
            "is_anonymous": 1,
            "status": "approved",
            "reject_reason": null,
            "content": "此城市一个月有半个月中度污染及以上，空气质量及其差劲...",
            "images": null,
            "display_name": "匿名用户",
            "created_at": "2026-04-22 09:20:50",
            "updated_at": "2026-04-22 09:20:50",
            "view_count": 0,
            "submission_id": null,
            "mod_task_id": "51ce1ab5-f3f0-4f65-b247-fcae3b335f22",
            "mod_suggestion": "pass",
            "campus_id": null,
            "like_count": 4,
            "reply_count": 2,
            "review_weight": 0.08,
            "is_question": 0,
            "nickname": "匿名用户3",
            "is_verified": 0,
            "verified_school_id": null,
            "avatar_path": null,
            "level": null,
            "title": null,
            "campus_name": null,
            "isAnonymous": true,
            "isVerified": false,
            "user_liked": false,
            "rating": null
        },
        {
            "id": 2113,
            "user_id": 3275,
            "school_id": 457,
            "is_anonymous": 1,
            "status": "approved",
            "reject_reason": null,
            "content": "此城市一个月有半个月中度污染及以上，空气质量及其差劲...",
            "images": null,
            "display_name": "匿名用户",
            "created_at": "2026-04-22 09:20:21",
            "updated_at": "2026-04-22 09:20:21",
            "view_count": 0,
            "submission_id": null,
            "mod_task_id": "3b60ca37-2a66-49a1-b35f-dc834dafea7a",
            "mod_suggestion": "pass",
            "campus_id": null,
            "like_count": 3,
            "reply_count": 0,
            "review_weight": 0.08,
            "is_question": 0,
            "nickname": "匿名用户3",
            "is_verified": 0,
            "verified_school_id": null,
            "avatar_path": null,
            "level": null,
            "title": null,
            "campus_name": null,
            "isAnonymous": true,
            "isVerified": false,
            "user_liked": false,
            "rating": null
        },
        {
            "id": 4367,
            "user_id": 8655,
            "school_id": 457,
            "is_anonymous": 1,
            "status": "approved",
            "reject_reason": null,
            "content": "我仅代表我个人体验观点...",
            "images": null,
            "display_name": "匿名用户",
            "created_at": "2026-05-03 07:58:29",
            "updated_at": "2026-05-03 07:58:29",
            "view_count": 0,
            "submission_id": null,
            "mod_task_id": "d46c4b1e-6799-424e-bc6e-e04b8c0ac9f7",
            "mod_suggestion": "pass",
            "campus_id": null,
            "like_count": 1,
            "reply_count": 1,
            "review_weight": 0.08,
            "is_question": 0,
            "nickname": "匿名用户5",
            "is_verified": 0,
            "verified_school_id": null,
            "avatar_path": null,
            "level": null,
            "title": null,
            "campus_name": null,
            "isAnonymous": true,
            "isVerified": false,
            "user_liked": false,
            "rating": null
        },
        {
            "id": 2112,
            "user_id": 3275,
            "school_id": 457,
            "is_anonymous": 1,
            "status": "approved",
            "reject_reason": null,
            "content": "此城市一个月有半个月中度污染及以上，空气质量及其差劲...",
            "images": null,
            "display_name": "匿名用户",
            "created_at": "2026-04-22 09:20:15",
            "updated_at": "2026-04-22 09:20:15",
            "view_count": 0,
            "submission_id": null,
            "mod_task_id": "70ac8427-daa3-4366-a876-749807b00e55",
            "mod_suggestion": "pass",
            "campus_id": null,
            "like_count": 1,
            "reply_count": 0,
            "review_weight": 0.08,
            "is_question": 0,
            "nickname": "匿名用户3",
            "is_verified": 0,
            "verified_school_id": null,
            "avatar_path": null,
            "level": null,
            "title": null,
            "campus_name": null,
            "isAnonymous": true,
            "isVerified": false,
            "user_liked": false,
            "rating": null
        },
        {
            "id": 6408,
            "user_id": 14318,
            "school_id": 457,
            "is_anonymous": 1,
            "status": "approved",
            "reject_reason": null,
            "content": "来这上学真是当m了...",
            "images": null,
            "display_name": "匿名用户",
            "created_at": "2026-06-05 22:44:36",
            "updated_at": "2026-06-13 11:14:27",
            "view_count": 0,
            "submission_id": null,
            "mod_task_id": "baca22f0-fd3b-4039-aeb1-6afa242972c5",
            "mod_suggestion": "block",
            "campus_id": 1131,
            "like_count": 0,
            "reply_count": 0,
            "review_weight": 0.08,
            "is_question": 0,
            "nickname": "匿名用户8",
            "is_verified": 0,
            "verified_school_id": null,
            "avatar_path": null,
            "level": null,
            "title": null,
            "campus_name": "东校区",
            "isAnonymous": true,
            "isVerified": false,
            "user_liked": false,
            "rating": null
        },
        {
            "id": 5544,
            "user_id": 11706,
            "school_id": 457,
            "is_anonymous": 1,
            "status": "approved",
            "reject_reason": null,
            "content": "纯纯的逆天学校...",
            "images": null,
            "display_name": "匿名用户",
            "created_at": "2026-05-14 08:40:48",
            "updated_at": "2026-05-14 08:40:48",
            "view_count": 0,
            "submission_id": null,
            "mod_task_id": "c26b8a60-c25b-44be-9383-78c8ee2fa1e3",
            "mod_suggestion": "pass",
            "campus_id": null,
            "like_count": 0,
            "reply_count": 0,
            "review_weight": 0.08,
            "is_question": 0,
            "nickname": "匿名用户7",
            "is_verified": 0,
            "verified_school_id": null,
            "avatar_path": null,
            "level": null,
            "title": null,
            "campus_name": null,
            "isAnonymous": true,
            "isVerified": false,
            "user_liked": false,
            "rating": {
                "id": 3685,
                "review_id": 5544,
                "dormitory": 1,
                "cafeteria": 3,
                "faculty": 2,
                "environment": 1,
                "culture": 3,
                "employment": 2,
                "safety": 4
            }
        },
        {
            "id": 5477,
            "user_id": 4057,
            "school_id": 457,
            "is_anonymous": 1,
            "status": "approved",
            "reject_reason": null,
            "content": "首先，要声明的是北华大学师范分院和北华大学只有大层面的关系...",
            "images": null,
            "display_name": "匿名用户",
            "created_at": "2026-05-13 05:52:08",
            "updated_at": "2026-05-13 05:52:08",
            "view_count": 0,
            "submission_id": null,
            "mod_task_id": "401eef83-bff4-40f9-8e67-e08ff2edcdf0",
            "mod_suggestion": "pass",
            "campus_id": null,
            "like_count": 0,
            "reply_count": 0,
            "review_weight": 5,
            "is_question": 0,
            "nickname": "匿名用户6",
            "is_verified": 1,
            "verified_school_id": 457,
            "avatar_path": null,
            "level": null,
            "title": null,
            "campus_name": null,
            "isAnonymous": true,
            "isVerified": true,
            "user_liked": false,
            "rating": {
                "id": 3627,
                "review_id": 5477,
                "dormitory": 1,
                "cafeteria": 4,
                "faculty": 4,
                "environment": 1,
                "culture": 2,
                "employment": 4,
                "safety": 3
            }
        },
        {
            "id": 2790,
            "user_id": 4673,
            "school_id": 457,
            "is_anonymous": 1,
            "status": "approved",
            "reject_reason": null,
            "content": "寝室的话 我这边可能是中奖了...",
            "images": "/uploads/f08bdeef-2f0a-40c2-a620-9a80836ce270.jpg",
            "display_name": "匿名用户",
            "created_at": "2026-04-26 15:45:54",
            "updated_at": "2026-04-26 15:45:54",
            "view_count": 0,
            "submission_id": null,
            "mod_task_id": "2cf6d0cf-7173-4c6c-b2e2-6d439f5323e1",
            "mod_suggestion": "pass",
            "campus_id": null,
            "like_count": 0,
            "reply_count": 0,
            "review_weight": 0.08,
            "is_question": 0,
            "nickname": "匿名用户4",
            "is_verified": 0,
            "verified_school_id": null,
            "avatar_path": null,
            "level": null,
            "title": null,
            "campus_name": null,
            "isAnonymous": true,
            "isVerified": false,
            "user_liked": false,
            "rating": null
        },
        {
            "id": 1738,
            "user_id": 2702,
            "school_id": 457,
            "is_anonymous": 1,
            "status": "approved",
            "reject_reason": null,
            "content": "早上六点整起床上早操...",
            "images": null,
            "display_name": "匿名用户",
            "created_at": "2026-04-21 11:28:58",
            "updated_at": "2026-04-21 11:28:58",
            "view_count": 0,
            "submission_id": null,
            "mod_task_id": "61178421-faf6-4b3b-8d53-c8410627daf6",
            "mod_suggestion": "pass",
            "campus_id": null,
            "like_count": 0,
            "reply_count": 0,
            "review_weight": 0.08,
            "is_question": 0,
            "nickname": "匿名用户2",
            "is_verified": 0,
            "verified_school_id": null,
            "avatar_path": null,
            "level": null,
            "title": null,
            "campus_name": null,
            "isAnonymous": true,
            "isVerified": false,
            "user_liked": false,
            "rating": {
                "id": 818,
                "review_id": 1738,
                "dormitory": 1,
                "cafeteria": 1,
                "faculty": 2,
                "environment": 3,
                "culture": 1,
                "employment": null,
                "safety": 4
            }
        },
        {
            "id": 598,
            "user_id": 1082,
            "school_id": 457,
            "is_anonymous": 1,
            "status": "approved",
            "reject_reason": null,
            "content": "南校，校区老破小...",
            "images": null,
            "display_name": "匿名用户",
            "created_at": "2026-04-19 19:45:49",
            "updated_at": "2026-04-19 19:45:49",
            "view_count": 0,
            "submission_id": null,
            "mod_task_id": "a90be8e5-e193-417a-a0cb-75a0fd51da41",
            "mod_suggestion": "pass",
            "campus_id": null,
            "like_count": 0,
            "reply_count": 0,
            "review_weight": 0.08,
            "is_question": 0,
            "nickname": "匿名用户1",
            "is_verified": 0,
            "verified_school_id": null,
            "avatar_path": null,
            "level": null,
            "title": null,
            "campus_name": null,
            "isAnonymous": true,
            "isVerified": false,
            "user_liked": false,
            "rating": null
        }
    ]
}"#;
        
        let resp: ReviewResponse = serde_json::from_str(json).unwrap();
        
        // 测试原始数据：10 条
        assert_eq!(resp.data.len(), 10);
        
        // 测试过滤重复用户
        let filtered = resp.filter_unique_users();
        
        // user_id: 3275 有 3 条，去重后保留 1 条，减少 2 条
        // 所以 10 - 2 = 8
        assert_eq!(filtered.data.len(), 8);  // ✅ 改为 8
        
        // 验证去重后 user_id 3275 只保留最新的一条
        let user_3275: Vec<&ReviewItem> = filtered
            .data
            .iter()
            .filter(|item| item.user_id == 3275)
            .collect();
        assert_eq!(user_3275.len(), 1);
        assert_eq!(user_3275[0].id, 2114); // 最新的评论 id 是 2114
        
        // 测试获取用户评论
        let comments = filtered.get_user_comments(3275);
        assert_eq!(comments.len(), 1);
        
        // 测试唯一用户 ID 列表
        let unique_ids = filtered.unique_user_ids();
        assert_eq!(unique_ids.len(), 8);
        assert!(unique_ids.contains(&3275));
        assert!(unique_ids.contains(&8655));
        assert!(unique_ids.contains(&14318));
        
        // 测试原始数据不受影响（因为 filter_unique_users 消耗了 self）
        let resp2: ReviewResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp2.data.len(), 10);
    }
}