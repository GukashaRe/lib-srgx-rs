# lib-srgx-rs

神人高校 · 第三方 Rust SDK

## 这是什么

lib-srgx-rs 是 [神人高校](https://srgaoxiao.com) 的第三方 Rust SDK。

神人高校是一个面向中国大学生的真实高校评价社区，汇集了全国 2900+ 所高校的多维度真实评价数据。

这个库让 Rust 开发者能够程序化地访问这些数据，用于 CLI 工具、数据分析、学术研究或第三方应用开发。

---

## 快速开始

### 安装

```toml
[dependencies]
lib-srgx-rs = "0.1.0"
```

### 使用示例

```rust
use lib_srgx_rs::legacy_api::LegacyApi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api = LegacyApi::new("your_token");

    // 搜索学校
    let schools = api.search_schools("武汉大学", 1, 5).await?;
    let school = &schools.data[0];

    // 拉取评价
    let reviews = api
        .get_school_reviews(school.id, None, "comprehensive", 1, 20)
        .await?;

    for item in reviews.data {
        println!("{}: {}", item.nickname, item.content);
    }

    Ok(())
}
```

---

## 功能列表

- 搜索学校（按名称、省份、城市、标签）
- 获取学校评价列表（支持翻页、排序、过滤）
- 获取评价回复
- 自动翻页拉取全部数据
- 导出 JSON 文件
- 自动生成统计摘要

---

## 技术设计

核心架构：

```
LegacyApi
  ├── fetch()              统一请求基础设施
  ├── search_schools()     搜索学校
  ├── get_school_reviews() 获取评价列表
  └── get_replies()        获取评价回复
```

设计特点：

- 类型安全：所有 API 响应都有对应的 Rust 结构体
- 统一错误处理：HTTP 错误和业务错误统一转换为 anyhow::Error
- 异步支持：基于 tokio 和 reqwest
- 零拷贝设计：使用 Cow 处理 Token 生命周期
- 自动处理 null：所有可能为 null 的字段都用 Option<T> 包装

---

## 运行测试

```bash
cargo test --features legacy_api_unfinished
```

---

## License

MIT

---

## 免责声明

本 SDK 是第三方社区项目，与神人高校官方无关。仅供学习和研究使用。

---

用 Rust 让真实评价触手可及。🦀
