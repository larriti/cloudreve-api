# cloudreve-api

[![Crates.io](https://img.shields.io/crates/v/cloudreve-api)](https://crates.io/crates/cloudreve-api)
[![Documentation](https://docs.rs/cloudreve-api/badge.svg)](https://docs.rs/cloudreve-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust library for interacting with the Cloudreve API. This library provides asynchronous access to all major Cloudreve API endpoints with proper error handling and type safety.

## Features

- ✨ Full async/await support with Tokio
- 🔐 Complete authentication flow (login, token refresh, 2FA)
- 📁 Comprehensive file operations (upload, download, list, delete, move, copy)
- 🔗 Share link management
- 👤 User and storage management
- 🎯 Type-safe request and response structures
- 🛡️ Built-in error handling
- 📦 Support for both Cloudreve v3 and v4 APIs

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cloudreve-api = "0.2"
tokio = { version = "1", features = ["full"] }
```

Or use cargo-edit:

```bash
cargo add cloudreve-api
```

## Quick Start

### Basic Setup

```rust
use cloudreve_api::CloudreveClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client instance
    let client = CloudreveClient::new("https://your-cloudreve-instance.com");

    // Login
    let token = client.login("user@example.com", "password123").await?;

    println!("Access token: {}", token.access_token);
    Ok(())
}
```

### File Operations

```rust
use cloudreve_api::CloudreveClient;
use cloudreve_api::api::v4::models::ListFilesRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = CloudreveClient::new("https://your-cloudreve-instance.com");
    let token = client.login("user@example.com", "password123").await?;
    client.set_token(&token.access_token);

    // List files in a directory
    let request = ListFilesRequest {
        path: "/",
        page: Some(0),
        page_size: Some(50),
        ..Default::default()
    };

    let response = client.list_files(&request).await?;
    println!("Found {} files", response.files.len());

    for file in response.files {
        println!("  - {} ({} bytes)", file.name, file.size);
    }

    Ok(())
}
```

### Upload a File

```rust
use cloudreve_api::CloudreveClient;
use cloudreve_api::api::v4::models::CreateUploadSessionRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = CloudreveClient::new("https://your-cloudreve-instance.com");
    let token = client.login("user@example.com", "password123").await?;
    client.set_token(&token.access_token);

    // Create an upload session
    let upload_request = CreateUploadSessionRequest {
        uri: "cloudreve://my/uploads/file.txt",
        size: 1024,
        policy_id: "1",
        ..Default::default()
    };

    let session = client.create_upload_session(&upload_request).await?;
    println!("Upload session ID: {}", session.session_id);

    // Upload file chunks
    let chunk_data = vec![0u8; 1024];
    client.upload_file_chunk(&session.session_id, 0, &chunk_data).await?;

    Ok(())
}
```

## API Coverage

### Supported API Categories

- **Authentication & Session**
  - Login with email/password
  - Token management
  - 2FA support

- **User Management**
  - User profile
  - Storage capacity
  - Storage policies
  - Settings management

- **File Operations**
  - List files/directories
  - Upload (chunked upload support)
  - Download
  - Create directory
  - Rename
  - Move
  - Copy
  - Delete
  - Get file info
  - File metadata

- **Sharing**
  - Create share links
  - List share links
  - Update share links
  - Delete share links

- **Advanced Features**
  - File search
  - Thumbnail generation
  - Archive operations

## Error Handling

All API calls return a `Result<T, Error>` type:

```rust
use cloudreve_api::{CloudreveClient, Error};

#[tokio::main]
async fn main() {
    let client = CloudreveClient::new("https://your-cloudreve-instance.com");

    match client.login("user@example.com", "wrong_password").await {
        Ok(token) => println!("Token: {}", token.access_token),
        Err(Error::Api { code, message }) => {
            eprintln!("API Error {}: {}", code, message);
        }
        Err(Error::Http(err)) => {
            eprintln!("HTTP Error: {}", err);
        }
        Err(err) => {
            eprintln!("Other Error: {}", err);
        }
    }
}
```

## Error Types

- `Error::Http` - Network/HTTP related errors
- `Error::Json` - Serialization/deserialization errors
- `Error::Api` - API error responses with code and message
- `Error::Reqwest` - Underlying reqwest errors
- `Error::Url` - URL parsing errors

## API Versions

This library supports both Cloudreve v3 and v4 APIs:

```rust
use cloudreve_api::api::{v3, v4};

// Use v3 API
let v3_client = CloudreveClient::new("https://instance.com");
v3_client.v3_get_site_policy().await?;

// Use v4 API
let v4_client = CloudreveClient::new("https://instance.com");
v4_client.v4_get_user_capacity().await?;
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

If you encounter any issues or have questions, please [file an issue](https://github.com/larriti/cloudreve-api/issues).

## Acknowledgments

- Built with [Tokio](https://tokio.rs/) for async runtime
- Uses [Reqwest](https://docs.rs/reqwest/) for HTTP requests
- Part of the [Cloudreve](https://github.com/cloudreve/Cloudreve) ecosystem
