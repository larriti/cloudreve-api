//! URI handling utilities for Cloudreve v4 API
//!
//! This module provides utilities for converting file paths to the Cloudreve URI format
//! and validating URIs according to the Cloudreve API specification.

/// Cloudreve URI prefix for user files
pub const CLOUDREVE_URI_PREFIX: &str = "cloudreve://my/";

/// Converts a file path to Cloudreve URI format
///
/// # Arguments
/// * `path` - File path (can be absolute, relative, or already a URI)
///
/// # Returns
/// A properly formatted Cloudreve URI string
///
/// # Examples
/// ```
/// use cloudreve_api::api::v4::uri::path_to_uri;
///
/// assert_eq!(path_to_uri("/path/to/file.txt"), "cloudreve://my/path/to/file.txt");
/// assert_eq!(path_to_uri("path/to/file.txt"), "cloudreve://my/path/to/file.txt");
/// assert_eq!(path_to_uri("cloudreve://my/path/to/file.txt"), "cloudreve://my/path/to/file.txt");
/// ```
pub fn path_to_uri(path: &str) -> String {
    // If already a valid Cloudreve URI, return as-is
    if path.starts_with("cloudreve://") {
        return path.to_string();
    }

    // Remove leading slash if present
    let trimmed_path = path.strip_prefix('/').unwrap_or(path);

    // Build URI
    format!("{}{}", CLOUDREVE_URI_PREFIX, trimmed_path)
}

/// Validates if a string is a properly formatted Cloudreve URI
///
/// # Arguments
/// * `uri` - URI string to validate
///
/// # Returns
/// `true` if the URI is valid, `false` otherwise
pub fn is_valid_uri(uri: &str) -> bool {
    uri.starts_with(CLOUDREVE_URI_PREFIX) && uri.len() >= CLOUDREVE_URI_PREFIX.len()
}

/// Extracts the path component from a Cloudreve URI
///
/// # Arguments
/// * `uri` - Cloudreve URI string
///
/// # Returns
/// The path component, or an error if the URI is invalid
pub fn uri_to_path(uri: &str) -> Result<&str, String> {
    if !uri.starts_with(CLOUDREVE_URI_PREFIX) {
        return Err(format!(
            "Invalid Cloudreve URI: expected format 'cloudreve://my/...', got: {}",
            uri
        ));
    }

    let path = &uri[CLOUDREVE_URI_PREFIX.len() - 1..];
    Ok(path)
}

/// Converts multiple paths to URIs
///
/// # Arguments
/// * `paths` - Slice of path strings
///
/// # Returns
/// Vector of converted URIs
pub fn paths_to_uris(paths: &[&str]) -> Vec<String> {
    paths.iter().map(|p| path_to_uri(p)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_to_uri_absolute() {
        assert_eq!(
            path_to_uri("/path/to/file.txt"),
            "cloudreve://my/path/to/file.txt"
        );
    }

    #[test]
    fn test_path_to_uri_relative() {
        assert_eq!(
            path_to_uri("path/to/file.txt"),
            "cloudreve://my/path/to/file.txt"
        );
    }

    #[test]
    fn test_path_to_uri_already_uri() {
        assert_eq!(
            path_to_uri("cloudreve://my/path/to/file.txt"),
            "cloudreve://my/path/to/file.txt"
        );
    }

    #[test]
    fn test_path_to_uri_root() {
        assert_eq!(path_to_uri("/"), "cloudreve://my/");
    }

    #[test]
    fn test_is_valid_uri() {
        assert!(is_valid_uri("cloudreve://my/path/to/file.txt"));
        assert!(is_valid_uri("cloudreve://my/"));
        assert!(!is_valid_uri("/path/to/file.txt"));
        assert!(!is_valid_uri("path/to/file.txt"));
        assert!(!is_valid_uri("cloudreve://"));
    }

    #[test]
    fn test_uri_to_path() {
        assert_eq!(
            uri_to_path("cloudreve://my/path/to/file.txt").unwrap(),
            "/path/to/file.txt"
        );
        assert_eq!(uri_to_path("cloudreve://my/").unwrap(), "/");
    }

    #[test]
    fn test_uri_to_path_invalid() {
        assert!(uri_to_path("/path/to/file.txt").is_err());
        assert!(uri_to_path("cloudreve://").is_err());
    }

    #[test]
    fn test_paths_to_uris() {
        let paths = vec!["/file1.txt", "file2.txt", "cloudreve://my/file3.txt"];
        let uris = paths_to_uris(&paths);
        assert_eq!(
            uris,
            vec![
                "cloudreve://my/file1.txt",
                "cloudreve://my/file2.txt",
                "cloudreve://my/file3.txt"
            ]
        );
    }
}
