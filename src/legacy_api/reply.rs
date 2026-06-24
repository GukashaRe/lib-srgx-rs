use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentReplys {
    pub id: i64,
    pub user_id: i64,
    pub nickname: String,
    pub is_anonymous: i64,
    pub is_verified: bool,
    pub is_official: bool,
    pub avatar_path: Option<serde_json::Value>,
    pub content: String,
    pub created_at: String,
    pub like_count: i64,
    pub user_liked: bool,
}

#[cfg(test)]
mod tests {
    use crate::legacy_api::reply::CommentReplys;

    #[test]
    fn test_reply_deserde() {
        let json = r#"[
    {
        "id": 324,
        "user_id": 4673,
        "nickname": "匿名用户1",
        "is_anonymous": 1,
        "is_verified": false,
        "is_official": false,
        "avatar_path": null,
        "content": "1111这个空气污染状况太糟糕了",
        "created_at": "2026-04-26 15:40:27",
        "like_count": 0,
        "user_liked": false
    },
    {
        "id": 325,
        "user_id": 4673,
        "nickname": "匿名用户1",
        "is_anonymous": 1,
        "is_verified": false,
        "is_official": false,
        "avatar_path": null,
        "content": "你北校还有早操？这也太惨了",
        "created_at": "2026-04-26 15:41:27",
        "like_count": 0,
        "user_liked": false
    }
]"#;
        let resp: Vec<CommentReplys> = serde_json::from_str(json).unwrap();
        assert_eq!(resp[0].content, "1111这个空气污染状况太糟糕了".to_string());
        assert_eq!(resp[1].avatar_path, None);
    }
}
