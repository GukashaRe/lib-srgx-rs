pub mod errors;

use serde::{Deserialize, Serialize};

/// 代表从 `/api/query` 接口返回的成功查询响应。
///
/// 仅当响应中 `success` 字段为 `true` 时，此结构体才完整填充。
///
/// # 示例
/// ```
/// use serde_json;
/// use lib_srgx_rs::api_data::QuerySuccessResponse;
///
/// let json = r#"{"success":true,"name":"张某某","schoolName":"某大学","major":"园艺","degreeLevel":"本科","elapsed":602,"nodeId":"node_1"}"#;
/// let resp: QuerySuccessResponse = serde_json::from_str(json).unwrap();
/// assert_eq!(resp.name, "张某某");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuerySuccessResponse {
    /// 是否查询成功。对于此结构体，该值始终为 `true`。
    pub success: bool,
    /// 被查询者的姓名。
    pub name: String,
    /// 毕业或就读的学校名称。
    pub school_name: String,
    /// 所学专业。
    pub major: String,
    /// 学历层次，例如 "本科"、"硕士"。
    pub degree_level: String,
    /// 查询耗时，单位毫秒。
    pub elapsed: u64,
    /// 处理此请求的节点标识符。
    pub node_id: String,
}

/// 代表从 `/api/query` 接口返回的失败查询响应。
///
/// 当响应中 `success` 字段为 `false` 时，使用此结构体。
///
/// # 示例
/// ```
/// use serde_json;
/// use lib_srgx_rs::api_data::QueryErrorResponse;
///
/// let json = r#"{"success":false,"message":"报告已过期","elapsed":300}"#;
/// let resp: QueryErrorResponse = serde_json::from_str(json).unwrap();
/// assert_eq!(resp.message, "报告已过期");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct QueryErrorResponse {
    /// 是否查询成功。对于此结构体，该值始终为 `false`。
    pub success: bool,
    /// 错误信息，例如 "报告已过期"。
    pub message: String,
    /// 查询耗时，单位毫秒。
    pub elapsed: u64,
}

/// 代表从 `/api/query` 接口返回的完整响应，它是成功和失败响应的枚举。
///
/// 这个枚举更全面地覆盖了API的所有可能返回情况，建议在解析时优先使用。
///
/// # 示例
/// ```
/// use serde_json;
/// use lib_srgx_rs::api_data::{QueryResponse, QuerySuccessResponse};
///
/// let json = r#"{"success":true,"name":"张某某","schoolName":"某大学","major":"园艺","degreeLevel":"本科","elapsed":602,"nodeId":"node_1"}"#;
/// let resp: QueryResponse = serde_json::from_str(json).unwrap();
///
/// assert!(resp.is_success());
/// if let Some(data) = resp.success_data() {
///     println!("查询成功: {}", data.name);
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum QueryResponse {
    /// 查询成功时收到的数据变体。
    Success(QuerySuccessResponse),
    /// 查询失败时收到的数据变体。
    Error(QueryErrorResponse),
}

impl QueryResponse {
    /// 检查查询是否成功。
    ///
    /// # 返回值
    /// - `true`: 查询成功，包含学历信息
    /// - `false`: 查询失败，包含错误信息
    #[must_use]
    pub fn is_success(&self) -> bool {
        matches!(self, QueryResponse::Success(_))
    }

    /// 如果查询成功，则返回成功数据的引用。
    ///
    /// # 返回值
    /// - `Some(&QuerySuccessResponse)`: 查询成功，包含数据
    /// - `None`: 查询失败
    #[must_use]
    pub fn success_data(&self) -> Option<&QuerySuccessResponse> {
        match self {
            QueryResponse::Success(data) => Some(data),
            QueryResponse::Error(_) => None,
        }
    }

    /// 如果查询成功，则返回成功数据的可变引用。
    #[must_use]
    pub fn success_data_mut(&mut self) -> Option<&mut QuerySuccessResponse> {
        match self {
            QueryResponse::Success(data) => Some(data),
            QueryResponse::Error(_) => None,
        }
    }

    /// 如果查询失败，则返回错误信息的引用。
    ///
    /// # 返回值
    /// - `Some(&QueryErrorResponse)`: 查询失败，包含错误信息
    /// - `None`: 查询成功
    #[must_use]
    pub fn error_data(&self) -> Option<&QueryErrorResponse> {
        match self {
            QueryResponse::Error(data) => Some(data),
            QueryResponse::Success(_) => None,
        }
    }

    /// 如果查询失败，则返回错误信息的可变引用。
    #[must_use]
    pub fn error_data_mut(&mut self) -> Option<&mut QueryErrorResponse> {
        match self {
            QueryResponse::Error(data) => Some(data),
            QueryResponse::Success(_) => None,
        }
    }

    /// 消耗枚举，返回成功数据的所有权。
    ///
    /// # 返回值
    /// - `Some(QuerySuccessResponse)`: 查询成功，包含数据
    /// - `None`: 查询失败
    #[must_use]
    pub fn into_success_data(self) -> Option<QuerySuccessResponse> {
        match self {
            QueryResponse::Success(data) => Some(data),
            QueryResponse::Error(_) => None,
        }
    }

    /// 消耗枚举，返回错误数据的所有权。
    ///
    /// # 返回值
    /// - `Some(QueryErrorResponse)`: 查询失败，包含错误信息
    /// - `None`: 查询成功
    #[must_use]
    pub fn into_error_data(self) -> Option<QueryErrorResponse> {
        match self {
            QueryResponse::Error(data) => Some(data),
            QueryResponse::Success(_) => None,
        }
    }
}

/// 用于构建API请求的参数结构体。
///
/// # 示例
/// ```
/// use lib_srgx_rs::api_data::QueryRequest;
///
/// let request = QueryRequest {
///     code: "ABCD1234EFGH5678".to_string(),
///     api_key: "sk_chsi_xxx".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct QueryRequest {
    /// 12-16 位字母数字验证码。
    pub code: String,
    /// 在控制台创建的API密钥。
    pub api_key: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_success_response() {
        let json = r#"{"success":true,"name":"张某某","schoolName":"某大学","major":"园艺","degreeLevel":"本科","elapsed":602,"nodeId":"node_1"}"#;
        let resp: QuerySuccessResponse = serde_json::from_str(json).unwrap();

        assert!(resp.success);
        assert_eq!(resp.name, "张某某");
        assert_eq!(resp.school_name, "某大学");
        assert_eq!(resp.major, "园艺");
        assert_eq!(resp.degree_level, "本科");
        assert_eq!(resp.elapsed, 602);
        assert_eq!(resp.node_id, "node_1");
    }

    #[test]
    fn test_deserialize_error_response() {
        let json = r#"{"success":false,"message":"报告已过期","elapsed":300}"#;
        let resp: QueryErrorResponse = serde_json::from_str(json).unwrap();

        assert!(!resp.success);
        assert_eq!(resp.message, "报告已过期");
        assert_eq!(resp.elapsed, 300);
    }

    #[test]
    fn test_deserialize_enum_success() {
        let json = r#"{"success":true,"name":"张某某","schoolName":"某大学","major":"园艺","degreeLevel":"本科","elapsed":602,"nodeId":"node_1"}"#;
        let resp: QueryResponse = serde_json::from_str(json).unwrap();

        assert!(resp.is_success());
        assert!(resp.success_data().is_some());
        assert!(resp.error_data().is_none());

        let data = resp.success_data().unwrap();
        assert_eq!(data.school_name, "某大学");
    }

    #[test]
    fn test_deserialize_enum_error() {
        let json = r#"{"success":false,"message":"报告已过期","elapsed":300}"#;
        let resp: QueryResponse = serde_json::from_str(json).unwrap();

        assert!(!resp.is_success());
        assert!(resp.success_data().is_none());
        assert!(resp.error_data().is_some());

        let error = resp.error_data().unwrap();
        assert_eq!(error.message, "报告已过期");
    }
}
