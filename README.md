# Cloudreve API

A Rust library for interacting with the Cloudreve API. This library provides asynchronous access to all major Cloudreve API endpoints with proper error handling and type safety.

## Features

- Full async/await support
- Comprehensive API coverage for Cloudreve v4
- Type-safe request and response structures
- Built-in error handling
- Easy authentication management
- Support for all major API categories:
  - Session and authentication
  - User management
  - File operations
  - Sharing
  - Workflow management

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cloudreve-api = "0.1.0"
```

Or use cargo-edit:

```bash
cargo add cloudreve-api
```

## Usage

### Basic Setup

```rust
use cloudreve_api::{CloudreveClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = CloudreveClient::new("https://your-cloudreve-instance.com");
    Ok(())
}
```

### Authentication

```rust
use cloudreve_api::{CloudreveClient, LoginRequest, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = CloudreveClient::new("https://your-cloudreve-instance.com");

    // Login
    let login_request = LoginRequest {
        email: "user@example.com",
        password: "password123",
        captcha: None,
        ticket: None,
    };

    let token = client.login(&login_request).await?;
    client.set_token(token.access_token);

    Ok(())
}
```

### File Operations

```rust
use cloudreve_api::{CloudreveClient, ListFilesRequest, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = CloudreveClient::new("https://your-cloudreve-instance.com");

    // List files
    let list_request = ListFilesRequest {
        path: "/",
        page: Some(1),
        per_page: Some(20),
        sort_by: Some("name"),
        order: Some("asc"),
    };

    let files = client.list_files(&list_request).await?;
    println!("Found {} files", files.len());

    Ok(())
}
```

## API Coverage

This library covers the following API categories:

- **Session Management**: Login, logout, token refresh, 2FA
- **User Management**: Signup, profile management, preferences, storage capacity
- **File Operations**: List, create, rename, move, delete files
- **Sharing**: Create, list, edit, delete share links
- **Workflow**: Remote downloads, task management, archive creation

## Error Handling

All API calls return a `Result<T, Error>` type. Common error types include:

- `Error::Http` - Network/http related errors
- `Error::Json` - Serialization/deserialization errors
- `Error::Api` - API error responses with code and message
- `Error::Auth` - Authentication failures
- `Error::InvalidResponse` - Malformed responses

## License

This project is licensed under the MIT License.
