use lib_srgx_rs::SrgxImpl;
use serde_json::Value;

#[tokio::test]
async fn test_reqwest() {
    // 使用测试专用的 token（建议从环境变量读取）
    let token = std::env::var("SRGX_TEST_TOKEN").unwrap_or_else(|_| "sk_chsi_abcdefg".to_string());
    let code = std::env::var("SRGX_TEST_CODE").unwrap_or_else(|_| "ABCDEFGHIJKL".to_string());

    let client = SrgxImpl::new(token, code);

    let result: Value = client.send_request("/api/query", None).await.unwrap();

    assert!(result.get("success").is_some());
}
