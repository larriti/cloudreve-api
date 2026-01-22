//! V4 API 集成测试

use super::common::{TestConfig, TestCredentials, TestResults};
use cloudreve_api::api::v4::{ApiV4Client, models::*, uri::path_to_uri};
use std::time::Instant;

/// V4 API 测试套件
pub struct V4TestSuite {
    client: ApiV4Client,
    credentials: TestCredentials,
    #[allow(dead_code)]
    config: TestConfig,
}

impl V4TestSuite {
    /// 创建新的测试套件
    pub async fn new(
        config: TestConfig,
        credentials: TestCredentials,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let v4_config = config.v4_config().ok_or("V4 配置未找到")?;
        let mut client = ApiV4Client::new(&v4_config.base_url);

        // 执行登录获取 token
        let login_request = LoginRequest {
            email: &credentials.username,
            password: &credentials.password,
        };

        match client.login(&login_request).await {
            Ok(data) => {
                client.set_token(data.token.access_token);
                println!("│  │  ✓ V4 登录成功: {}", data.user.nickname);
            }
            Err(e) => {
                println!("│  │  ✗ V4 登录失败: {}", e);
                return Err(format!("V4 登录失败: {}", e).into());
            }
        }

        Ok(Self {
            client,
            credentials,
            config,
        })
    }

    /// 运行所有 V4 测试
    pub async fn run_all(&self) -> TestResults {
        let mut results = TestResults::new();
        let start = Instant::now();

        println!("\n┌─ V4 API 测试 ─────────────────────────────────────");

        // Session 测试
        results.merge(self.test_session().await);

        // File 测试
        results.merge(self.test_file().await);

        // User 测试
        results.merge(self.test_user().await);

        // Share 测试
        results.merge(self.test_share().await);

        // WebDAV 测试
        results.merge(self.test_webdav().await);

        // Workflow 测试
        results.merge(self.test_workflow().await);

        // Site 测试
        results.merge(self.test_site().await);

        results.duration_ms = start.elapsed().as_millis() as u64;
        results
    }

    /// Session 模块测试
    async fn test_session(&self) -> TestResults {
        let mut results = TestResults::new();
        println!("│  ├─ Session 测试...");

        // 准备登录
        match self.client.prepare_login(&self.credentials.username).await {
            Ok(prep) => {
                println!("│  │  ✓ 准备登录: password={}", prep.password_enabled);
                results.add_success();
            }
            Err(e) => {
                println!("│  │  ✗ 准备登录失败: {}", e);
                results.add_failure(
                    "v4_session_prepare".to_string(),
                    "v4".to_string(),
                    e.to_string(),
                );
            }
        }

        // 登出测试
        match self.client.logout().await {
            Ok(_) => {
                println!("│  │  ✓ 登出成功");
                results.add_success();
            }
            Err(e) => {
                println!("│  │  ✗ 登出失败: {}", e);
                results.add_failure(
                    "v4_session_logout".to_string(),
                    "v4".to_string(),
                    e.to_string(),
                );
            }
        }

        results
    }

    /// File 模块测试
    async fn test_file(&self) -> TestResults {
        let mut results = TestResults::new();
        println!("│  ├─ File 测试...");

        // 列出根目录
        match self
            .client
            .list_files(&ListFilesRequest {
                path: "/",
                page: Some(0),
                page_size: Some(50),
                order_by: None,
                order_direction: None,
                next_page_token: None,
            })
            .await
        {
            Ok(response) => {
                println!("│  │  ✓ 列出根目录: {} 个对象", response.files.len());
                results.add_success();
            }
            Err(e) => {
                println!("│  │  ✗ 列出根目录失败: {}", e);
                results.add_failure("v4_file_list".to_string(), "v4".to_string(), e.to_string());
            }
        }

        // 创建测试目录
        let test_dir_name = format!("test_v4_{}", chrono::Utc::now().timestamp());
        let test_path = format!("/{}", test_dir_name);

        match self.client.create_directory(&test_path).await {
            Ok(_) => {
                println!("│  │  ✓ 创建目录: {}", test_path);
                results.add_success();

                // 等待目录创建完成并同步
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

                // 获取文件信息 - 注意：这个测试可能会失败，因为某些 V4 实例的 404 响应格式问题
                match self.client.get_file_info(&test_path).await {
                    Ok(file) => {
                        println!("│  │  ✓ 获取文件信息: {} ({:?})", file.name, file.r#type);
                        results.add_success();

                        // 重命名目录
                        let new_name = format!("{}_renamed", test_dir_name);
                        let uri = path_to_uri(&test_path);
                        match self
                            .client
                            .rename_file(&RenameFileRequest {
                                uri: &uri,
                                new_name: &new_name,
                            })
                            .await
                        {
                            Ok(_) => {
                                println!("│  │  ✓ 重命名目录成功");
                                results.add_success();

                                // 删除目录
                                let new_path = format!("/{}", new_name);
                                match self.client.delete_file(&new_path).await {
                                    Ok(_) => {
                                        println!("│  │  ✓ 删除目录成功");
                                        results.add_success();
                                    }
                                    Err(e) => {
                                        println!("│  │  ✗ 删除目录失败: {} (可能需要手动清理)", e);
                                        results.add_failure(
                                            "v4_file_delete".to_string(),
                                            "v4".to_string(),
                                            e.to_string(),
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                println!("│  │  ✗ 重命名目录失败: {}", e);
                                results.add_failure(
                                    "v4_file_rename".to_string(),
                                    "v4".to_string(),
                                    e.to_string(),
                                );
                                // 清理
                                let _ = self.client.delete_file(&test_path).await;
                            }
                        }
                    }
                    Err(e) => {
                        println!("│  │  ✗ 获取文件信息失败: {} (检测到 API 响应格式问题)", e);
                        results.add_failure(
                            "v4_file_info".to_string(),
                            "v4".to_string(),
                            format!("{} - 这表明该端点在文件不存在时返回了非标准格式的响应", e),
                        );
                        // 清理
                        let _ = self.client.delete_file(&test_path).await;
                    }
                }
            }
            Err(e) => {
                println!("│  │  ✗ 创建目录失败: {}", e);
                results.add_failure(
                    "v4_file_create".to_string(),
                    "v4".to_string(),
                    e.to_string(),
                );
            }
        }

        results
    }

    /// User 模块测试
    async fn test_user(&self) -> TestResults {
        let mut results = TestResults::new();
        println!("│  ├─ User 测试...");

        // 获取用户设置 - 使用已有的方法
        match self.client.get_user_setting().await {
            Ok(_) => {
                println!("│  │  ✓ 获取用户设置成功");
                results.add_success();
            }
            Err(e) => {
                println!("│  │  ✗ 获取用户设置失败: {}", e);
                results.add_failure(
                    "v4_user_setting".to_string(),
                    "v4".to_string(),
                    e.to_string(),
                );
            }
        }

        // 获取存储策略 - 跳过此测试，端点可能不存在
        println!("│  │  ⊘ 跳过存储策略测试（端点可能不存在）");
        results.add_skip();

        // 获取用户容量 - 使用已有的方法
        match self.client.get_user_capacity().await {
            Ok(_) => {
                println!("│  │  ✓ 获取用户容量成功");
                results.add_success();
            }
            Err(e) => {
                println!("│  │  ✗ 获取用户容量失败: {}", e);
                results.add_failure(
                    "v4_user_capacity".to_string(),
                    "v4".to_string(),
                    e.to_string(),
                );
            }
        }

        results
    }

    /// Share 模块测试
    async fn test_share(&self) -> TestResults {
        let mut results = TestResults::new();
        println!("│  ├─ Share 测试...");

        // 列出分享
        match self.client.get::<serde_json::Value>("/share").await {
            Ok(_) => {
                println!("│  │  ✓ 列出分享成功");
                results.add_success();
            }
            Err(e) => {
                println!("│  │  ✗ 列出分享失败: {}", e);
                results.add_failure("v4_share_list".to_string(), "v4".to_string(), e.to_string());
            }
        }

        results
    }

    /// WebDAV 模块测试
    async fn test_webdav(&self) -> TestResults {
        let mut results = TestResults::new();
        println!("│  ├─ WebDAV 测试...");

        // 列出 WebDAV 账户
        match self.client.get::<serde_json::Value>("/devices/dav").await {
            Ok(_) => {
                println!("│  │  ✓ 列出 WebDAV 账户成功");
                results.add_success();
            }
            Err(e) => {
                println!("│  │  ✗ 列出 WebDAV 账户失败: {}", e);
                results.add_failure(
                    "v4_webdav_list".to_string(),
                    "v4".to_string(),
                    e.to_string(),
                );
            }
        }

        results
    }

    /// Workflow 模块测试
    async fn test_workflow(&self) -> TestResults {
        let mut results = TestResults::new();
        println!("│  ├─ Workflow 测试...");

        // 列出工作流任务
        match self.client.get::<serde_json::Value>("/workflow").await {
            Ok(_) => {
                println!("│  │  ✓ 列出工作流任务成功");
                results.add_success();
            }
            Err(e) => {
                println!("│  │  ✗ 列出工作流任务失败: {}", e);
                results.add_failure(
                    "v4_workflow_list".to_string(),
                    "v4".to_string(),
                    e.to_string(),
                );
            }
        }

        results
    }

    /// Site 模块测试
    async fn test_site(&self) -> TestResults {
        let mut results = TestResults::new();
        println!("│  ├─ Site 测试...");

        // Ping 测试 - 使用已有的方法
        match self.client.ping().await {
            Ok(_) => {
                println!("│  │  ✓ Site ping 成功");
                results.add_success();
            }
            Err(e) => {
                println!("│  │  ✗ Site ping 失败: {}", e);
                results.add_failure("v4_site_ping".to_string(), "v4".to_string(), e.to_string());
            }
        }

        // 获取站点版本 - 使用 get_version 方法
        match self.client.get_version().await {
            Ok(_) => {
                println!("│  │  ✓ 获取站点版本成功");
                results.add_success();
            }
            Err(e) => {
                println!("│  │  ✗ 获取站点版本失败: {}", e);
                results.add_failure(
                    "v4_site_version".to_string(),
                    "v4".to_string(),
                    e.to_string(),
                );
            }
        }

        // 获取站点配置 - 测试不同的 section
        use cloudreve_api::api::v4::models::SiteConfigSection;

        let sections = [
            SiteConfigSection::Basic,
            SiteConfigSection::Login,
            SiteConfigSection::Explorer,
            SiteConfigSection::Emojis,
            SiteConfigSection::Vas,
            SiteConfigSection::App,
            SiteConfigSection::Thumb,
        ];

        for section in sections {
            match self.client.get_site_config(section).await {
                Ok(config) => {
                    println!("│  │  ✓ 获取站点配置 [{}] 成功", section.as_str());
                    results.add_success();
                }
                Err(e) => {
                    println!("│  │  ✗ 获取站点配置 [{}] 失败: {}", section.as_str(), e);
                    results.add_failure(
                        format!("v4_site_config_{}", section.as_str()),
                        "v4".to_string(),
                        e.to_string(),
                    );
                }
            }
        }

        results
    }
}
