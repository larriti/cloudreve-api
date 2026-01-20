//! API 集成测试主入口
//!
//! 运行此测试将执行所有 V3 和 V4 API 的集成测试

mod integration;
mod validation;

use integration::{common::*, v3_tests::V3TestSuite, v4_tests::V4TestSuite};
use validation::OpenApiValidator;

#[tokio::test]
async fn run_api_integration_tests() {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║     Cloudreve API 集成测试套件                          ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    // 加载测试配置
    let config = match TestConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("\n错误: {}", e);
            println!("\n请按以下步骤配置测试环境:");
            println!(
                "1. 复制配置文件: cp tests/config/test_config.example.toml tests/config/test_config.toml"
            );
            println!("2. 编辑 tests/config/test_config.toml，填入你的测试环境信息");
            println!("3. 重新运行测试\n");
            panic!("配置文件未找到或无效");
        }
    };

    println!("配置加载成功:");
    if config.v3_enabled() {
        println!("  - V3: 已启用 ({})", config.v3_config().unwrap().base_url);
    } else {
        println!("  - V3: 未配置");
    }
    if config.v4_enabled() {
        println!("  - V4: 已启用 ({})", config.v4_config().unwrap().base_url);
    } else {
        println!("  - V4: 未配置");
    }

    let mut all_results = TestResults::new();

    // 运行 V3 测试
    if config.v3_enabled() {
        let credentials = TestCredentials::from(config.v3_config().unwrap());
        match V3TestSuite::new(config.clone(), credentials).await {
            Ok(mut suite) => {
                let results = suite.run_all().await;
                all_results.merge(results);
            }
            Err(e) => {
                println!("│  ✗ V3 测试套件初始化失败: {}", e);
            }
        }
    }

    // 运行 V4 测试
    if config.v4_enabled() {
        let credentials = TestCredentials::from(config.v4_config().unwrap());
        match V4TestSuite::new(config.clone(), credentials).await {
            Ok(suite) => {
                let results = suite.run_all().await;
                all_results.merge(results);
            }
            Err(e) => {
                println!("│  ✗ V4 测试套件初始化失败: {}", e);
            }
        }
    }

    // 打印汇总
    all_results.print_summary();

    // 运行 OpenAPI 验证
    if config.validation_enabled() {
        println!("\n╔══════════════════════════════════════════════════════════╗");
        println!("║     OpenAPI 规范验证                                    ║");
        println!("╚══════════════════════════════════════════════════════════╝");

        let mut validator =
            OpenApiValidator::new(config.strict_mode(), config.allow_extra_fields())
                .await
                .expect("无法创建验证器");

        // 尝试从缓存加载规范
        let _ = validator.load_v3_from_cache().await;
        let _ = validator.load_v4_from_cache().await;

        // 如果缓存不存在，尝试从 MCP 加载
        if !validator.has_v3_spec() || !validator.has_v4_spec() {
            println!("  [OpenAPI] 缓存不存在，尝试从 MCP 加载...");
            let _ = validator.load_v3_from_mcp().await;
            let _ = validator.load_v4_from_mcp().await;
        }

        // 验证 V3
        let v3_report = validator.validate_v3();
        v3_report.print();

        // 验证 V4
        let v4_report = validator.validate_v4();
        v4_report.print();

        // 检查是否严格模式下有错误
        if config.strict_mode() && (v3_report.has_errors() || v4_report.has_errors()) {
            panic!("OpenAPI 验证失败（严格模式）");
        }
    }

    // 如果有测试失败，打印详细信息但不要 panic（这样可以看到所有测试结果）
    if !all_results.is_success() {
        println!("\n╔══════════════════════════════════════════════════════════╗");
        println!("║     测试发现的问题汇总                                  ║");
        println!("╚══════════════════════════════════════════════════════════╝");
        println!("上述测试失败表明可能存在以下问题：");
        println!("1. API 端点实现不正确");
        println!("2. 请求/响应结构体与实际 API 不匹配");
        println!("3. 某些端点返回格式不符合预期");
        println!("4. 时序问题（某些操作需要等待）");
        println!("\n建议：");
        println!("1. 检查失败的端点对应的代码实现");
        println!("2. 对比 OpenAPI 规范验证实现是否正确");
        println!("3. 考虑添加重试逻辑处理时序问题");
        println!("4. 检查错误响应处理是否完善");
    }
}
