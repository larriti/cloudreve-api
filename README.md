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
cloudreve-api = "0.3"
tokio = { version = "1", features = ["full"] }
```

Or use cargo-edit:

```bash
cargo add cloudreve-api
```

## Quick Start

### Basic Setup

```rust
use cloudreve_api::{CloudreveAPI, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance (auto-detects API version)
    let mut api = CloudreveAPI::new("https://your-cloudreve-instance.com").await?;

    // Login
    api.login("user@example.com", "password").await?;

    println!("Successfully logged in!");
    Ok(())
}
```

### File Operations

```rust
use cloudreve_api::{CloudreveAPI, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut api = CloudreveAPI::new("https://your-cloudreve-instance.com").await?;
    api.login("user@example.com", "password").await?;

    // List files in a directory
    let files = api.list_files("/").await?;
    println!("Found {} items", files.total_count());

    for item in files.items() {
        if item.is_folder {
            println!("  📁 {}/", item.name);
        } else {
            println!("  📄 {} ({} bytes)", item.name, item.size);
        }
    }

    Ok(())
}
```

### Create Directory

```rust
use cloudreve_api::{CloudreveAPI, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut api = CloudreveAPI::new("https://your-cloudreve-instance.com").await?;
    api.login("user@example.com", "password").await?;

    // Create a new directory
    api.create_directory("/photos/vacation").await?;

    println!("Directory created successfully!");
    Ok(())
}
```

### Upload a File

```rust
use cloudreve_api::{CloudreveAPI, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut api = CloudreveAPI::new("https://your-cloudreve-instance.com").await?;
    api.login("user@example.com", "password").await?;

    // Upload a file
    let content = b"Hello, World!".to_vec();
    api.upload_file("/hello.txt", content, None).await?;

    println!("File uploaded successfully!");
    Ok(())
}
```

### Download a File

```rust
use cloudreve_api::{CloudreveAPI, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut api = CloudreveAPI::new("https://your-cloudreve-instance.com").await?;
    api.login("user@example.com", "password").await?;

    // Get download URL
    let url = api.download_file("/document.pdf").await?;
    println!("Download URL: {}", url);

    Ok(())
}
```

### Share Management

```rust
use cloudreve_api::{CloudreveAPI, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut api = CloudreveAPI::new("https://your-cloudreve-instance.com").await?;
    api.login("user@example.com", "password").await?;

    // List shares
    let shares = api.list_shares().await?;
    for share in shares {
        println!("Share key: {}", share.key);
    }

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
  - Upload (with chunked support)
  - Download
  - Create directory
  - Rename
  - Move
  - Copy
  - Delete
  - Get file info

- **Sharing**
  - Create share links
  - List share links
  - Update share links
  - Delete share links

- **Advanced Features**
  - File search
  - Thumbnail generation
  - Archive operations
  - WebDAV account management

## API Versions

This library supports both Cloudreve v3 and v4 APIs with automatic version detection:

```rust
use cloudreve_api::{CloudreveAPI, ApiVersion, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Auto-detect version
    let api = CloudreveAPI::new("https://instance.com").await?;

    // Or specify version explicitly
    let api = CloudreveAPI::with_version("https://instance.com", ApiVersion::V4)?;

    Ok(())
}
```

## Error Handling

All API calls return a `Result<T, Error>` type:

```rust
use cloudreve_api::{CloudreveAPI, Error};

#[tokio::main]
async fn main() {
    let mut api = CloudreveAPI::new("https://your-cloudreve-instance.com").await.unwrap();

    match api.login("user@example.com", "wrong_password").await {
        Ok(_) => println!("Login successful"),
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
- `Error::InvalidResponse` - Invalid API response format

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
