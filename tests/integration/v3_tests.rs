//! V3 API 集成测试

use super::common::{TestConfig, TestCredentials, TestResults};
use cloudreve_api::api::v3::{ApiV3Client, models::*};
use std::time::Instant;

/// V3 API 测试套件
pub struct V3TestSuite {
    client: ApiV3Client,
    credentials: TestCredentials,
    config: TestConfig,
}

impl V3TestSuite {
    /// 创建新的测试套件
    pub async fn new(config: TestConfig, credentials: TestCredentials) -> Result<Self, Box<dyn std::error::Error>> {
        let v3_config = config.v3_config().ok_or("V3 配置未找到")?;
        let client = ApiV3Client::new(&v3_config.base_url);

        Ok(Self {
            client,
            credentials,
            config,
        })
    }

    /// 执行登录
    async fn login(&mut self) -> Result<User, Box<dyn std::error::Error>> {
        let request = LoginRequest {
            user_name: &self.credentials.username,
            password: &self.credentials.password,
            captcha_code: "",
        };
        Ok(self.client.login(&request).await?)
    }

    /// 运行所有 V3 测试
    pub async fn run_all(&mut self) -> TestResults {
        let mut results = TestResults::new();
        let start = Instant::now();

        println!("\n┌─ V3 API 测试 ─────────────────────────────────────");

        // Session 测试
        results.merge(self.test_session().await);

        // Directory 测试
        results.merge(self.test_directory().await);

        // File 测试
        results.merge(self.test_file().await);

        // Object 测试
        results.merge(self.test_object().await);

        // Share 测试
        results.merge(self.test_share().await);

        // Site 测试
        results.merge(self.test_site().await);

        // User 测试
        results.merge(self.test_user().await);

        results.duration_ms = start.elapsed().as_millis() as u64;
        results
    }

    /// Session 模块测试
    async fn test_session(&mut self) -> TestResults {
        let mut results = TestResults::new();
        println!("│  ├─ Session 测试...");

        // 登录测试
        match self.login().await {
            Ok(user) => {
                println!("│  │  ✓ 登录成功: {}", user.nickname);
                results.add_success();
            }
            Err(e) => {
                println!("│  │  ✗ 登录失败: {}", e);
                results.add_failure("v3_session_login".to_string(), "v3".to_string(), e.to_string());
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
                results.add_failure("v3_session_logout".to_string(), "v3".to_string(), e.to_string());
            }
        }

        // 重新登录以便后续测试
        let _ = self.login().await;

        results
    }

    /// Directory 模块测试
    async fn test_directory(&self) -> TestResults {
        let mut results = TestResults::new();
        println!("│  ├─ Directory 测试...");

        // 列出根目录
        match self.client.list_directory("/").await {
            Ok(list) => {
                println!("│  │  ✓ 列出根目录: {} 个对象", list.objects.len());
                results.add_success();
            }
            Err(e) => {
                println!("│  │  ✗ 列出根目录失败: {}", e);
                results.add_failure("v3_directory_list".to_string(), "v3".to_string(), e.to_string());
            }
        }

        // 创建测试目录
        let test_dir_name = format!("test_dir_{}", chrono::Utc::now().timestamp());
        let test_path = format!("/{}", test_dir_name);

        match self.client.create_directory(&CreateDirectoryRequest {
            path: &test_path,
        }).await {
            Ok(_) => {
                println!("│  │  ✓ 创建目录: {}", test_path);
                results.add_success();

                // 清理：删除测试目录
                let _ = self.client.delete_object(&DeleteObjectRequest {
                    dirs: vec![&test_path],
                    items: vec![],
                    force: true,
                    unlink: false,
                }).await;
            }
            Err(e) => {
                println!("│  │  ✗ 创建目录失败: {}", e);
                results.add_failure("v3_directory_create".to_string(), "v3".to_string(), e.to_string());
            }
        }

        results
    }

    /// File 模块测试
    async fn test_file(&self) -> TestResults {
        let mut results = TestResults::new();
        println!("│  ├─ File 测试...");

        // 首先获取存储策略
        let policy_id = match self.get_policy_id().await {
            Some(id) => id,
            None => {
                println!("│  │  ⊘ 跳过 File 测试: 无法获取存储策略");
                results.add_skip();
                results.add_skip();
                return results;
            }
        };

        // 创建上传会话
        let test_file_name = format!("test_{}.txt", chrono::Utc::now().timestamp());
        let upload_request = UploadFileRequest {
            path: "/",
            size: 1024,
            name: &test_file_name,
            policy_id: &policy_id,
            last_modified: chrono::Utc::now().timestamp(),
            mime_type: "text/plain",
        };

        match self.client.upload_file(&upload_request).await {
            Ok(session) => {
                println!("│  │  ✓ 创建上传会话: {}", session.session_id);
                results.add_success();

                // 上传分片（空数据用于测试）
                match self.client.upload_chunk(&session.session_id, 0, vec![b' '; 1024]).await {
                    Ok(_) => {
                        println!("│  │  ✓ 上传分片成功");
                        results.add_success();
                    }
                    Err(e) => {
                        println!("│  │  ✗ 上传分片失败: {}", e);
                        results.add_failure("v3_file_upload_chunk".to_string(), "v3".to_string(), e.to_string());
                    }
                }

                // 完成上传
                match self.client.complete_upload(&session.session_id).await {
                    Ok(_) => {
                        println!("│  │  ✓ 完成上传");
                        results.add_success();
                    }
                    Err(e) => {
                        println!("│  │  ✗ 完成上传失败: {}", e);
                        results.add_failure("v3_file_complete_upload".to_string(), "v3".to_string(), e.to_string());
                    }
                }
            }
            Err(e) => {
                println!("│  │  ✗ 创建上传会话失败: {}", e);
                results.add_failure("v3_file_upload".to_string(), "v3".to_string(), e.to_string());
                results.add_skip();
                results.add_skip();
            }
        }

        results
    }

    /// Object 模块测试
    async fn test_object(&self) -> TestResults {
        let mut results = TestResults::new();
        println!("│  ├─ Object 测试...");

        // 创建测试对象
        let test_dir_name = format!("test_obj_{}", chrono::Utc::now().timestamp());
        let test_path = format!("/{}", test_dir_name);

        // 先创建目录
        match self.client.create_directory(&CreateDirectoryRequest {
            path: &test_path,
        }).await {
            Ok(_) => {
                // 等待目录创建完成
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

                // 获取对象 ID 用于后续操作
                let obj_id = match self.client.list_directory("/").await {
                    Ok(list) => {
                        list.objects.iter().find(|o| o.name == test_dir_name).map(|o| o.id.clone())
                    }
                    Err(_) => None,
                };

                if let Some(id) = obj_id {
                    // 获取对象属性
                    match self.client.get_object_property(&id, Some(true), None).await {
                        Ok(prop) => {
                            println!("│  │  ✓ 获取对象属性: {}", prop.path);
                            results.add_success();
                        }
                        Err(e) => {
                            println!("│  │  ✗ 获取对象属性失败: {}", e);
                            results.add_failure("v3_object_get_property".to_string(), "v3".to_string(), e.to_string());
                        }
                    }

                    // 重命名测试 - 使用对象 ID 而不是路径
                    let new_name = format!("{}_renamed", test_dir_name);
                    match self.client.rename_object(&RenameObjectRequest {
                        action: "rename",
                        src: SourceItems { dirs: vec![id.as_str()], items: vec![] },
                        new_name: &new_name,
                    }).await {
                        Ok(_) => {
                            println!("│  │  ✓ 重命名对象成功");
                            results.add_success();

                            // 清理：删除重命名后的对象
                            let new_path = format!("/{}", new_name);
                            let _ = self.client.delete_object(&DeleteObjectRequest {
                                dirs: vec![new_path.as_str()],
                                items: vec![],
                                force: true,
                                unlink: false,
                            }).await;
                        }
                        Err(e) => {
                            println!("│  │  ✗ 重命名对象失败: {}", e);
                            results.add_failure("v3_object_rename".to_string(), "v3".to_string(), e.to_string());

                            // 清理
                            let _ = self.client.delete_object(&DeleteObjectRequest {
                                dirs: vec![test_path.as_str()],
                                items: vec![],
                                force: true,
                                unlink: false,
                            }).await;
                        }
                    }
                } else {
                    println!("│  │  ⊘ 跳过测试: 未找到创建的目录 (可能需要更多时间)");
                    results.add_skip();
                    results.add_skip();
                }
            }
            Err(e) => {
                println!("│  │  ✗ 创建目录失败: {}", e);
                results.add_failure("v3_object_create".to_string(), "v3".to_string(), e.to_string());
            }
        }

        results
    }

    /// Share 模块测试
    async fn test_share(&self) -> TestResults {
        let mut results = TestResults::new();
        println!("│  ├─ Share 测试...");

        // 创建测试目录用于分享
        let test_dir_name = format!("test_share_{}", chrono::Utc::now().timestamp());
        let test_path = format!("/{}", test_dir_name);

        let obj_id = if let Ok(_) = self.client.create_directory(&CreateDirectoryRequest {
            path: &test_path,
        }).await {
            // 获取目录 ID
            match self.client.list_directory("/").await {
                Ok(list) => {
                    list.objects.iter().find(|o| o.name == test_dir_name).map(|o| o.id.clone())
                }
                _ => None
            }
        } else {
            None
        };

        if let Some(id) = obj_id {
            match self.client.create_share(&ShareRequest {
                id: id.clone(),
                is_dir: true,
                password: "".to_string(),
                downloads: 0,
                expire: 0,
                preview: true,
            }).await {
                Ok(share) => {
                    println!("│  │  ✓ 创建分享: {}", share.key);
                    results.add_success();
                }
                Err(e) => {
                    println!("│  │  ✗ 创建分享失败: {}", e);
                    results.add_failure("v3_share_create".to_string(), "v3".to_string(), e.to_string());
                }
            }

            // 清理
            let _ = self.client.delete_object(&DeleteObjectRequest {
                dirs: vec![&test_path],
                items: vec![],
                force: true,
                unlink: false,
            }).await;
        } else {
            results.add_skip();
        }

        results
    }

    /// Site 模块测试
    async fn test_site(&self) -> TestResults {
        let mut results = TestResults::new();
        println!("│  ├─ Site 测试...");

        // Ping 测试
        match self.client.get::<serde_json::Value>("/site/ping").await {
            Ok(_) => {
                println!("│  │  ✓ Site ping 成功");
                results.add_success();
            }
            Err(e) => {
                println!("│  │  ✗ Site ping 失败: {}", e);
                results.add_failure("v3_site_ping".to_string(), "v3".to_string(), e.to_string());
            }
        }

        // 获取站点配置
        match self.client.get::<SiteConfig>("/site/config").await {
            Ok(config) => {
                println!("│  │  ✓ 获取站点配置: {}", config.title);
                results.add_success();
            }
            Err(e) => {
                println!("│  │  ✗ 获取站点配置失败: {}", e);
                results.add_failure("v3_site_config".to_string(), "v3".to_string(), e.to_string());
            }
        }

        results
    }

    /// User 模块测试
    async fn test_user(&self) -> TestResults {
        let mut results = TestResults::new();
        println!("│  ├─ User 测试...");

        // 获取用户设置
        match self.client.get::<serde_json::Value>("/user/setting").await {
            Ok(_) => {
                println!("│  │  ✓ 获取用户设置成功");
                results.add_success();
            }
            Err(e) => {
                println!("│  │  ✗ 获取用户设置失败: {}", e);
                results.add_failure("v3_user_setting".to_string(), "v3".to_string(), e.to_string());
            }
        }

        // 获取存储信息 - 使用 ApiResponse 包装
        match self.client.get::<ApiResponse<StorageInfo>>("/user/storage").await {
            Ok(response) => {
                if let Some(storage) = response.data {
                    println!("│  │  ✓ 获取存储信息: {} / {} bytes",
                        storage.used, storage.total);
                    results.add_success();
                } else {
                    println!("│  │  ✗ 获取存储信息失败: 无数据");
                    results.add_failure("v3_user_storage".to_string(), "v3".to_string(), "无数据".to_string());
                }
            }
            Err(e) => {
                println!("│  │  ✗ 获取存储信息失败: {}", e);
                results.add_failure("v3_user_storage".to_string(), "v3".to_string(), e.to_string());
            }
        }

        // 获取 WebDAV 账户 - 使用已有的方法
        match self.client.get_webdav_accounts().await {
            Ok(accounts) => {
                println!("│  │  ✓ 获取 WebDAV 账户: {} 个", accounts.len());
                results.add_success();
            }
            Err(e) => {
                println!("│  │  ✗ 获取 WebDAV 账户失败: {}", e);
                results.add_failure("v3_user_webdav".to_string(), "v3".to_string(), e.to_string());
            }
        }

        results
    }

    /// 辅助方法：获取存储策略 ID
    async fn get_policy_id(&self) -> Option<String> {
        match self.client.list_directory("/").await {
            Ok(list) => Some(list.policy.id),
            Err(_) => None,
        }
    }
}
