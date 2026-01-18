//! Basic usage example for the Cloudreve API Rust library

use cloudreve_api::{CloudreveClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a new client
    let _client = CloudreveClient::new("https://your-cloudreve-instance.com");

    // Example 1: Get site configuration
    // Note: This method doesn't exist in the current API, so it's commented out
    // match client.get_site_config().await {
    //     Ok(config) => println!("Site config: {:?}", config),
    //     Err(e) => eprintln!("Failed to get site config: {}", e),
    // }

    // Example 2: User registration (uncomment to test)
    /*
    let register_request = RegisterRequest {
        username: "testuser",
        password: "password123",
        email: Some("test@example.com"),
    };

    match client.register(&register_request).await {
        Ok(user) => println!("User registered: {:?}", user),
        Err(e) => eprintln!("Registration failed: {}", e),
    }
    */

    // Example 3: User login (uncomment to test)
    /*
    let login_request = LoginRequest {
        username: "testuser",
        password: "password123",
    };

    match client.login(&login_request).await {
        Ok(user) => {
            println!("Login successful!");
            // Set the token for subsequent authenticated requests
            client.set_token(user.id.to_string()); // In real usage, you'd use the actual token
        }
        Err(e) => eprintln!("Login failed: {}", e),
    }
    */

    // Example 4: Get current user info (requires authentication)
    /*
    match client.get_current_user().await {
        Ok(user) => println!("Current user: {:?}", user),
        Err(e) => eprintln!("Failed to get user info: {}", e),
    }
    */

    // Example 5: List files (requires authentication)
    /*
    let list_request = ListFilesRequest {
        path: "/",
        page: Some(1),
        per_page: Some(20),
        sort_by: Some("name"),
        order: Some("asc"),
    };

    match client.list_files(&list_request).await {
        Ok(files) => println!("Found {} files", files.len()),
        Err(e) => eprintln!("Failed to list files: {}", e),
    }
    */

    println!("Basic usage example completed");
    Ok(())
}
