//! 共享测试工具和配置

use serde::Deserialize;
use std::fs;
use std::path::Path;

/// 测试配置
#[derive(Debug, Clone, Deserialize)]
pub struct TestConfig {
    pub general: GeneralConfig,
    pub environments: EnvironmentsConfig,
    pub validation: ValidationConfig,
    pub reporting: ReportingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_timeout")]
    pub timeout: u64,
    #[serde(default = "default_parallel")]
    pub parallel: bool,
    #[serde(default = "default_verbose")]
    pub verbose: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvironmentsConfig {
    pub v3: Option<EnvironmentConfig>,
    pub v4: Option<EnvironmentConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvironmentConfig {
    pub base_url: String,
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub otp_secret: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ValidationConfig {
    #[serde(default = "default_check_openapi")]
    pub check_openapi_compliance: bool,
    #[serde(default)]
    pub strict_mode: bool,
    #[serde(default = "default_allow_extra_fields")]
    pub allow_extra_fields: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReportingConfig {
    #[serde(default)]
    pub output_format: Vec<String>,
    #[serde(default = "default_json_report_path")]
    pub json_report_path: String,
}

fn default_timeout() -> u64 { 300 }
fn default_parallel() -> bool { true }
fn default_verbose() -> bool { true }
fn default_check_openapi() -> bool { true }
fn default_allow_extra_fields() -> bool { true }
fn default_json_report_path() -> String { "test-results/results.json".to_string() }

impl TestConfig {
    /// 从文件加载配置
    pub fn load() -> Result<Self, String> {
        let config_path = "tests/config/test_config.toml";

        // 如果配置文件不存在，尝试使用示例配置
        let path = if Path::new(config_path).exists() {
            config_path
        } else {
            return Err(format!(
                "配置文件不存在: {}\n请复制 test_config.example.toml 为 test_config.toml 并填入测试环境信息",
                config_path
            ));
        };

        let content = fs::read_to_string(path)
            .map_err(|e| format!("无法读取配置文件: {}", e))?;

        let config: TestConfig = toml::from_str(&content)
            .map_err(|e| format!("解析配置文件失败: {}", e))?;

        Ok(config)
    }

    /// 检查 V3 环境是否配置
    pub fn v3_enabled(&self) -> bool {
        self.environments.v3.is_some()
    }

    /// 检查 V4 环境是否配置
    pub fn v4_enabled(&self) -> bool {
        self.environments.v4.is_some()
    }

    /// 获取 V3 配置
    pub fn v3_config(&self) -> Option<&EnvironmentConfig> {
        self.environments.v3.as_ref()
    }

    /// 获取 V4 配置
    pub fn v4_config(&self) -> Option<&EnvironmentConfig> {
        self.environments.v4.as_ref()
    }

    /// 是否启用 OpenAPI 验证
    pub fn validation_enabled(&self) -> bool {
        self.validation.check_openapi_compliance
    }

    /// 是否严格模式
    pub fn strict_mode(&self) -> bool {
        self.validation.strict_mode
    }

    /// 是否允许额外字段
    pub fn allow_extra_fields(&self) -> bool {
        self.validation.allow_extra_fields
    }
}

/// 测试结果
#[derive(Debug, Default)]
pub struct TestResults {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub failures: Vec<TestFailure>,
    pub duration_ms: u64,
}

#[derive(Debug)]
pub struct TestFailure {
    pub test_name: String,
    pub version: String,
    pub error: String,
}

impl TestResults {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_success(&mut self) {
        self.total += 1;
        self.passed += 1;
    }

    pub fn add_failure(&mut self, test_name: String, version: String, error: String) {
        self.total += 1;
        self.failed += 1;
        self.failures.push(TestFailure {
            test_name,
            version,
            error,
        });
    }

    pub fn add_skip(&mut self) {
        self.total += 1;
        self.skipped += 1;
    }

    pub fn merge(&mut self, other: TestResults) {
        self.total += other.total;
        self.passed += other.passed;
        self.failed += other.failed;
        self.skipped += other.skipped;
        self.failures.extend(other.failures);
        self.duration_ms += other.duration_ms;
    }

    pub fn is_success(&self) -> bool {
        self.failed == 0
    }

    pub fn print_summary(&self) {
        println!("\n╔══════════════════════════════════════════════════════════╗");
        println!("║                   测试结果汇总                           ║");
        println!("╚══════════════════════════════════════════════════════════╝");
        println!("总计: {} | 通过: {} | 失败: {} | 跳过: {}",
            self.total, self.passed, self.failed, self.skipped);
        println!("耗时: {}ms\n", self.duration_ms);

        if !self.failures.is_empty() {
            println!("失败的测试:");
            for failure in &self.failures {
                println!("  [{}] {}: {}", failure.version, failure.test_name, failure.error);
            }
            println!();
        }
    }
}

/// 测试凭证
#[derive(Debug, Clone)]
pub struct TestCredentials {
    pub username: String,
    pub password: String,
    pub otp_secret: Option<String>,
}

impl From<&EnvironmentConfig> for TestCredentials {
    fn from(config: &EnvironmentConfig) -> Self {
        Self {
            username: config.username.clone(),
            password: config.password.clone(),
            otp_secret: config.otp_secret.clone(),
        }
    }
}
