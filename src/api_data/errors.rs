use anyhow::anyhow;
use serde::Deserialize;

/// API特定的错误类型
///
/// 这个枚举覆盖了文档中所有可能返回的错误情况
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    /// 验证码格式不对或已使用
    InvalidCode,

    /// 报告已过期
    ReportExpired,

    /// 学信网未找到该验证码
    CodeNotFound,

    /// 请求频率过高，等待10秒自动恢复
    IpBlocked,

    /// 其他未知错误（兜底）
    Unknown(String),
}

impl ApiError {
    /// 从API返回的message字符串解析错误类型
    pub fn from_message(msg: &str) -> Self {
        match msg {
            "不合要求的在线验证码" => ApiError::InvalidCode,
            "报告已过期" => ApiError::ReportExpired,
            "验证码无效" => ApiError::CodeNotFound,
            "IP 被学信网拦截" => ApiError::IpBlocked,
            _ => ApiError::Unknown(msg.to_string()),
        }
    }

    /// 转换为用户友好的显示信息
    pub fn to_user_message(&self) -> String {
        match self {
            ApiError::InvalidCode => "验证码格式不正确或已使用".to_string(),
            ApiError::ReportExpired => "学历报告已过期".to_string(),
            ApiError::CodeNotFound => "学信网未找到该验证码".to_string(),
            ApiError::IpBlocked => "请求频率过高，IP被学信网拦截，请等待10秒后重试".to_string(),
            ApiError::Unknown(msg) => format!("未知错误: {}", msg),
        }
    }

    /// 判断是否需要重试
    pub fn is_retryable(&self) -> bool {
        matches!(self, ApiError::IpBlocked)
    }
    /// 转换为 anyhow::Error
    pub fn to_anyhow(&self) -> anyhow::Error {
        anyhow!(self.to_user_message())
    }
}

/// 扩展的错误响应结构体
#[derive(Debug, Clone, Deserialize)]
pub struct ApiErrorResponse {
    pub success: bool,
    pub message: String,
    pub elapsed: u64,
}

impl ApiErrorResponse {
    /// 转换为类型安全的错误枚举
    pub fn into_api_error(self) -> ApiError {
        ApiError::from_message(&self.message)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_user_message())
    }
}

impl std::error::Error for ApiError {}
