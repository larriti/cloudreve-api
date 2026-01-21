//! File operations for CloudreveAPI

use crate::Error;
use crate::api::v3::models as v3_models;
use crate::api::v4::models as v4_models;
use crate::api::v4::uri::path_to_uri;
use crate::client::UnifiedClient;
use log::debug;

/// Result of batch delete operation
#[derive(Debug, Default)]
pub struct DeleteResult {
    pub deleted: usize,
    pub failed: usize,
    pub errors: Vec<(String, String)>,
}

/// File operation methods for CloudreveAPI
impl super::CloudreveAPI {
    /// List files in a directory
    ///
    /// Returns a unified file list regardless of API version.
    pub async fn list_files(
        &self,
        path: &str,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<FileList, Error> {
        debug!("Listing files in: {}", path);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3 doesn't support pagination in list_directory
                let dir_list = client.list_directory(path).await?;
                Ok(FileList::V3(dir_list))
            }
            UnifiedClient::V4(client) => {
                let page_size = page_size.unwrap_or(100);

                // First, fetch the first page to check pagination mode (cursor vs offset)
                let first_request = v4_models::ListFilesRequest {
                    path,
                    page: Some(0),
                    page_size: Some(page_size),
                    order_by: None,
                    order_direction: None,
                    next_page_token: None,
                };
                let first_response = client.list_files(&first_request).await?;

                // If no page specified or requesting page 0, return the first page
                if page.is_none() || page == Some(0) {
                    return Ok(FileList::V4(Box::new(first_response)));
                }

                let target_page = page.unwrap();
                let is_cursor = first_response.pagination.is_cursor;

                if is_cursor {
                    // Cursor pagination: need to fetch pages sequentially to get next_page_token
                    let mut next_token = first_response.pagination.next_token.clone();
                    let mut current_response = first_response;

                    for current_page in 1..=target_page {
                        // Check if we have more pages
                        if next_token.is_none()
                            || next_token.as_ref().map(|t| t.is_empty()).unwrap_or(true)
                        {
                            return Err(Error::InvalidResponse(format!(
                                "Page {} does not exist (only {} pages available)",
                                target_page, current_page
                            )));
                        }

                        // Fetch next page using the token
                        let request = v4_models::ListFilesRequest {
                            path,
                            page: Some(current_page),
                            page_size: Some(page_size),
                            order_by: None,
                            order_direction: None,
                            next_page_token: next_token.as_deref(),
                        };
                        current_response = client.list_files(&request).await?;

                        // If this is the target page, return it
                        if current_page == target_page {
                            return Ok(FileList::V4(Box::new(current_response)));
                        }

                        // Get next token for next iteration
                        next_token = current_response.pagination.next_token.clone();
                    }

                    // Should not reach here, but handle the case
                    Ok(FileList::V4(Box::new(current_response)))
                } else {
                    // Offset pagination: can directly request the target page
                    let request = v4_models::ListFilesRequest {
                        path,
                        page: Some(target_page),
                        page_size: Some(page_size),
                        order_by: None,
                        order_direction: None,
                        next_page_token: None,
                    };
                    let list_response = client.list_files(&request).await?;
                    Ok(FileList::V4(Box::new(list_response)))
                }
            }
        }
    }

    /// List all files in a directory with automatic pagination
    ///
    /// This method automatically fetches all pages for V4 API and combines them.
    /// For V3 API, it returns the single page result (no pagination support).
    pub async fn list_files_all(
        &self,
        path: &str,
        page_size: Option<u32>,
    ) -> Result<FileListAll, Error> {
        debug!("Listing all files in: {} (with pagination)", path);

        let page_size = page_size.unwrap_or(500); // Default to 500 items per page

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3 doesn't support pagination
                let dir_list = client.list_directory(path).await?;
                Ok(FileListAll::V3(dir_list))
            }
            UnifiedClient::V4(client) => {
                let mut all_files = Vec::new();
                let mut parent: Option<v4_models::File> = None;
                let mut storage_policy: Option<v4_models::StoragePolicy> = None;
                #[allow(unused_assignments)]
                let mut pagination: Option<v4_models::PaginationResults> = None;
                let mut next_token: Option<String> = None;
                let mut page_num = 1;

                loop {
                    let request = v4_models::ListFilesRequest {
                        path,
                        page: Some(page_num),
                        page_size: Some(page_size),
                        order_by: None,
                        order_direction: None,
                        next_page_token: next_token.as_deref(),
                    };
                    let list_response = client.list_files(&request).await?;

                    // Store parent and storage_policy from first response
                    if parent.is_none() {
                        parent = Some(list_response.parent.clone());
                        storage_policy = list_response.storage_policy.clone();
                    }

                    // Collect files
                    all_files.extend(list_response.files);

                    // Check if there are more pages (before moving pagination)
                    next_token = list_response.pagination.next_token.clone();
                    let has_more = next_token.is_some();

                    // Store pagination info from last response
                    pagination = Some(list_response.pagination);

                    if !has_more {
                        break;
                    }

                    page_num += 1;
                    debug!(
                        "Fetching page {} (next_token: {})",
                        page_num,
                        next_token.as_ref().unwrap()
                    );
                }

                let parent = parent.expect("parent should always be set after first API call");
                let pagination = pagination.expect("should have at least one response");
                let combined_response = v4_models::ListResponse {
                    files: all_files,
                    parent,
                    pagination,
                    props: v4_models::NavigatorProps {
                        capability: String::new(),
                        max_page_size: page_size as i32,
                        order_by_options: Vec::new(),
                        order_direction_options: Vec::new(),
                    },
                    context_hint: String::new(),
                    mixed_type: false,
                    storage_policy,
                    view: None,
                };

                Ok(FileListAll::V4(Box::new(combined_response)))
            }
        }
    }

    /// Create a directory
    ///
    /// Creates a new directory at the specified path.
    pub async fn create_directory(&self, path: &str) -> Result<(), Error> {
        debug!("Creating directory: {}", path);

        match &self.inner {
            UnifiedClient::V3(client) => {
                let request = v3_models::CreateDirectoryRequest { path };
                client.create_directory(&request).await?;
                Ok(())
            }
            UnifiedClient::V4(client) => {
                client.create_directory(path).await?;
                Ok(())
            }
        }
    }

    /// Delete a file or directory
    ///
    /// Accepts either a path or URI for deletion.
    pub async fn delete(&self, target: DeleteTarget) -> Result<(), Error> {
        debug!("Deleting target: {:?}", target);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3 requires IDs, not paths. Need to get the ID from the parent directory listing.
                let path = match &target {
                    DeleteTarget::Path(p) => p.as_str(),
                    DeleteTarget::Uri(u) => u.as_str(),
                };

                // Get the parent directory to find the object's ID
                let normalized_path = if path.ends_with('/') && path != "/" {
                    &path[..path.len() - 1]
                } else {
                    path
                };

                let parent_path = if normalized_path == "/" {
                    return Err(Error::InvalidResponse(
                        "Cannot delete root directory".to_string(),
                    ));
                } else {
                    let pos = normalized_path.rfind('/');
                    match pos {
                        Some(0) => "/",
                        Some(p) => &normalized_path[..p],
                        None => "/",
                    }
                };

                let file_name = normalized_path.rsplit('/').next().unwrap_or("");

                // List parent directory to find the object
                let dir_list = client.list_directory(parent_path).await?;

                // Find the object by name to get its ID and type
                let obj = dir_list
                    .objects
                    .iter()
                    .find(|obj| obj.name == file_name)
                    .ok_or_else(|| Error::InvalidResponse(format!("File not found: {}", path)))?;

                // Separate into files and folders based on object type
                let (folders, files) = if obj.object_type == "dir" {
                    (vec![obj.id.as_str()], Vec::<&str>::new())
                } else {
                    (Vec::<&str>::new(), vec![obj.id.as_str()])
                };

                let request = v3_models::DeleteObjectRequest {
                    items: files,
                    dirs: folders,
                    force: true,
                    unlink: false,
                };
                client.delete_object(&request).await?;
                Ok(())
            }
            UnifiedClient::V4(client) => {
                let path = match &target {
                    DeleteTarget::Path(p) => p.as_str(),
                    DeleteTarget::Uri(u) => u.as_str(),
                };
                client.delete_file(path).await?;
                Ok(())
            }
        }
    }

    /// Batch delete multiple files and/or folders
    ///
    /// This method accepts multiple paths and deletes them all in a single API call.
    /// Files and folders can be mixed in the same request. The server handles
    /// recursive deletion of folder contents automatically.
    ///
    /// # Arguments
    /// * `paths` - Slice of paths to delete (can mix files and folders)
    ///
    /// # Example
    /// ```no_run
    /// # use cloudreve_api::CloudreveAPI;
    /// # async fn example(api: &CloudreveAPI) -> cloudreve_api::Result<()> {
    /// // Delete multiple items at once
    /// api.batch_delete(&[
    ///     "/folder/file1.txt",
    ///     "/folder/file2.txt",
    ///     "/another_folder",  // folder will be deleted recursively
    /// ]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn batch_delete(&self, paths: &[&str]) -> Result<DeleteResult, Error> {
        debug!("Batch deleting {} paths", paths.len());

        if paths.is_empty() {
            return Ok(DeleteResult::default());
        }

        match &self.inner {
            UnifiedClient::V3(client) => self.batch_delete_v3(client, paths).await,
            UnifiedClient::V4(client) => self.batch_delete_v4(client, paths).await,
        }
    }

    /// Get file information by path or URI
    ///
    /// Returns unified file information regardless of API version.
    pub async fn get_file_info(&self, path: &str) -> Result<FileInfo, Error> {
        debug!("Getting file info for: {}", path);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3: Use object property (requires ID) or get from directory listing
                // For simplicity, list the parent directory and find the object

                // Normalize path: remove trailing slash unless it's the root directory
                let normalized_path = if path.ends_with('/') && path != "/" {
                    &path[..path.len() - 1]
                } else {
                    path
                };

                let parent_path = if normalized_path == "/" {
                    "/"
                } else {
                    let pos = normalized_path.rfind('/');
                    match pos {
                        Some(0) => "/",
                        Some(p) => &normalized_path[..p],
                        None => "/",
                    }
                };

                let dir_list = client.list_directory(parent_path).await?;

                // Find the object by name
                let file_name = if normalized_path == "/" {
                    ""
                } else {
                    normalized_path.rsplit('/').next().unwrap_or("")
                };

                for obj in &dir_list.objects {
                    if obj.name == file_name {
                        return Ok(FileInfo::V3(obj.clone()));
                    }
                }

                Err(Error::InvalidResponse(format!("File not found: {}", path)))
            }
            UnifiedClient::V4(client) => {
                let request = v4_models::GetFileInfoRequest {
                    uri: path,
                    include_extended_info: Some(false),
                };
                let file = client.get_file_info_extended(&request).await?;
                Ok(FileInfo::V4(file))
            }
        }
    }

    /// Rename a file or directory
    ///
    /// Renames a file or directory at the given path to a new name.
    pub async fn rename(&self, path: &str, new_name: &str) -> Result<(), Error> {
        debug!("Renaming {} to {}", path, new_name);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3 needs object ID, not path. Get the ID from parent directory listing.
                let normalized_path = if path.ends_with('/') && path != "/" {
                    &path[..path.len() - 1]
                } else {
                    path
                };

                let parent_path = if normalized_path == "/" {
                    return Err(Error::InvalidResponse(
                        "Cannot rename root directory".to_string(),
                    ));
                } else {
                    let pos = normalized_path.rfind('/');
                    match pos {
                        Some(0) => "/",
                        Some(p) => &normalized_path[..p],
                        None => "/",
                    }
                };

                let file_name = normalized_path.rsplit('/').next().unwrap_or("");

                debug!(
                    "V3 rename: parent_path={}, file_name={}, new_name={}",
                    parent_path, file_name, new_name
                );

                // List parent directory to find the object ID
                let dir_list = client.list_directory(parent_path).await?;

                debug!(
                    "V3 rename: found {} objects in parent directory",
                    dir_list.objects.len()
                );

                // Find the object by name to get its ID
                let obj = dir_list
                    .objects
                    .iter()
                    .find(|obj| obj.name == file_name)
                    .ok_or_else(|| {
                        // Provide helpful error message showing available files
                        let available_files: Vec<String> = dir_list.objects
                            .iter()
                            .filter(|obj| obj.object_type == "file")
                            .map(|obj| obj.name.clone())
                            .take(10)
                            .collect();
                        Error::InvalidResponse(format!(
                            "File not found: '{}'. Did you mean:\n  - {}\nAvailable files in {}: {}",
                            path,
                            available_files.join("\n  - "),
                            parent_path,
                            available_files.len()
                        ))
                    })?;

                debug!(
                    "V3 rename: found object id={}, type={}",
                    obj.id, obj.object_type
                );

                // Use object ID for rename
                let request = v3_models::RenameObjectRequest {
                    action: "rename",
                    src: v3_models::SourceItems {
                        dirs: if obj.object_type == "dir" {
                            vec![obj.id.as_str()]
                        } else {
                            vec![]
                        },
                        items: if obj.object_type != "dir" {
                            vec![obj.id.as_str()]
                        } else {
                            vec![]
                        },
                    },
                    new_name,
                };
                client.rename_object(&request).await?;
                Ok(())
            }
            UnifiedClient::V4(client) => {
                let uri = path_to_uri(path);
                let request = v4_models::RenameFileRequest {
                    uri: uri.as_str(),
                    new_name,
                };
                let _ = client.rename_file(&request).await?;
                Ok(())
            }
        }
    }

    /// Move a file or directory
    ///
    /// Moves a file or directory from source path to destination path.
    pub async fn move_file(&self, src: &str, dest: &str) -> Result<(), Error> {
        debug!("Moving {} to {}", src, dest);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3 needs object ID, not path. Get the ID from parent directory listing.
                let normalized_path = if src.ends_with('/') && src != "/" {
                    &src[..src.len() - 1]
                } else {
                    src
                };

                // Normalize destination path - remove trailing slash unless it's root
                let normalized_dest = if dest.ends_with('/') && dest != "/" {
                    &dest[..dest.len() - 1]
                } else {
                    dest
                };

                let src_dir = if let Some(pos) = normalized_path.rfind('/') {
                    if pos == 0 {
                        "/"
                    } else {
                        &normalized_path[..pos]
                    }
                } else {
                    "/"
                };

                let file_name = normalized_path.rsplit('/').next().unwrap_or("");

                debug!(
                    "V3 move: src_dir={}, file_name={}, dest={}",
                    src_dir, file_name, normalized_dest
                );

                // List parent directory to find the object ID
                let dir_list = client.list_directory(src_dir).await?;

                // Find the object by name to get its ID
                let obj = dir_list
                    .objects
                    .iter()
                    .find(|obj| obj.name == file_name)
                    .ok_or_else(|| Error::InvalidResponse(format!("File not found: {}", src)))?;

                debug!(
                    "V3 move: found object id={}, type={}",
                    obj.id, obj.object_type
                );

                // Verify destination directory exists
                match client.list_directory(normalized_dest).await {
                    Ok(_) => {
                        debug!("V3 move: destination directory exists");
                    }
                    Err(e) => {
                        return Err(Error::InvalidResponse(format!(
                            "Destination directory '{}' does not exist or is not accessible: {}",
                            normalized_dest, e
                        )));
                    }
                }

                let request = v3_models::MoveObjectRequest {
                    action: "move",
                    src_dir,
                    src: v3_models::SourceItems {
                        dirs: if obj.object_type == "dir" {
                            vec![obj.id.as_str()]
                        } else {
                            vec![]
                        },
                        items: if obj.object_type != "dir" {
                            vec![obj.id.as_str()]
                        } else {
                            vec![]
                        },
                    },
                    dst: normalized_dest,
                };
                client.move_object(&request).await?;
                Ok(())
            }
            UnifiedClient::V4(client) => {
                // V4 API: Check if this is a rename operation (same directory)
                // Extract source directory and filename
                let src_normalized = if src.ends_with('/') && src != "/" {
                    &src[..src.len() - 1]
                } else {
                    src
                };

                let dest_normalized = if dest.ends_with('/') && dest != "/" {
                    &dest[..dest.len() - 1]
                } else {
                    dest
                };

                let src_dir = if let Some(pos) = src_normalized.rfind('/') {
                    if pos == 0 {
                        "/"
                    } else {
                        &src_normalized[..pos]
                    }
                } else {
                    "/"
                };

                let dest_dir = if let Some(pos) = dest_normalized.rfind('/') {
                    if pos == 0 {
                        "/"
                    } else {
                        &dest_normalized[..pos]
                    }
                } else {
                    "/"
                };

                let src_name = src_normalized.rsplit('/').next().unwrap_or("");
                let dest_name = dest_normalized.rsplit('/').next().unwrap_or("");

                // If same directory, use rename operation
                if src_dir == dest_dir && src_name != dest_name {
                    debug!(
                        "Detected rename operation within same directory: {} -> {}",
                        src, dest
                    );
                    let src_uri = path_to_uri(src);
                    let request = v4_models::RenameFileRequest {
                        uri: src_uri.as_str(),
                        new_name: dest_name,
                    };
                    let _ = client.rename_file(&request).await?;
                    Ok(())
                } else {
                    // Different directory, use move operation (dest is treated as directory)
                    let src_uri = path_to_uri(src);
                    let dest_uri = path_to_uri(dest);
                    let request = v4_models::MoveFileRequest {
                        uris: vec![src_uri.as_str()],
                        dst: dest_uri.as_str(),
                        copy: None,
                    };
                    client.move_file(&request).await?;
                    Ok(())
                }
            }
        }
    }

    /// Copy a file or directory
    ///
    /// Copies a file or directory from source path to destination path.
    pub async fn copy_file(&self, src: &str, dest: &str) -> Result<(), Error> {
        debug!("Copying {} to {}", src, dest);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3 needs object ID, not path. Get the ID from parent directory listing.
                let normalized_path = if src.ends_with('/') && src != "/" {
                    &src[..src.len() - 1]
                } else {
                    src
                };

                let src_dir = if let Some(pos) = normalized_path.rfind('/') {
                    if pos == 0 {
                        "/"
                    } else {
                        &normalized_path[..pos]
                    }
                } else {
                    "/"
                };

                let file_name = normalized_path.rsplit('/').next().unwrap_or("");

                // List parent directory to find the object ID
                let dir_list = client.list_directory(src_dir).await?;

                // Find the object by name to get its ID
                let obj = dir_list
                    .objects
                    .iter()
                    .find(|obj| obj.name == file_name)
                    .ok_or_else(|| Error::InvalidResponse(format!("File not found: {}", src)))?;

                let request = v3_models::CopyObjectRequest {
                    src_dir,
                    src: v3_models::SourceItems {
                        dirs: if obj.object_type == "dir" {
                            vec![obj.id.as_str()]
                        } else {
                            vec![]
                        },
                        items: if obj.object_type != "dir" {
                            vec![obj.id.as_str()]
                        } else {
                            vec![]
                        },
                    },
                    dst: dest,
                };
                client.copy_object(&request).await?;
                Ok(())
            }
            UnifiedClient::V4(client) => {
                // Parse source and destination paths
                let src_normalized = if src.ends_with('/') && src != "/" {
                    &src[..src.len() - 1]
                } else {
                    src
                };
                let dest_normalized = if dest.ends_with('/') && dest != "/" {
                    &dest[..dest.len() - 1]
                } else {
                    dest
                };

                // Extract source directory and file name
                let src_dir = if let Some(pos) = src_normalized.rfind('/') {
                    if pos == 0 {
                        "/"
                    } else {
                        &src_normalized[..pos]
                    }
                } else {
                    "/"
                };
                let src_name = src_normalized.rsplit('/').next().unwrap_or("");

                // Extract destination directory and file name
                // For V4 API, the dst parameter should be the target directory URI
                // If dest ends with a filename (different from src), we need to handle it specially
                let dest_dir = if let Some(pos) = dest_normalized.rfind('/') {
                    if pos == 0 {
                        "/"
                    } else {
                        &dest_normalized[..pos]
                    }
                } else {
                    "/"
                };
                let dest_name = dest_normalized.rsplit('/').next().unwrap_or("");

                let src_uri = path_to_uri(src);
                let dest_dir_uri = path_to_uri(dest_dir);

                // Check if this is a "copy and rename" operation (same directory, different name)
                if src_dir == dest_dir && src_name != dest_name && !dest_name.is_empty() {
                    // V4 API doesn't support copy+rename in one operation
                    // Strategy: Use a temporary directory as an intermediate step
                    debug!(
                        "Detected copy+rename operation in same directory: {} -> {}",
                        src, dest
                    );

                    // Step 0: If destination file exists, delete it first
                    let dest_path = format!("{}/{}", dest_dir.trim_end_matches('/'), dest_name);
                    let dest_uri = path_to_uri(&dest_path);
                    if client.get_file_info(dest_uri.as_str()).await.is_err() {
                        // File doesn't exist, continue
                    } else {
                        // File exists, delete it
                        debug!("Destination file exists, deleting: {}", dest_path);
                        let delete_request = v4_models::DeleteFileRequest {
                            uris: vec![dest_uri.as_str()],
                            unlink: None,
                            skip_soft_delete: None,
                        };
                        let _: Result<v4_models::ApiResponse<()>, _> =
                            client.delete_with_body("/file", &delete_request).await;
                    }

                    // Step 1: Create a temporary directory
                    let temp_dir_name = format!(".temp_copy_{}", std::process::id());
                    let temp_dir_path =
                        format!("{}/{}", dest_dir.trim_end_matches('/'), temp_dir_name);
                    let _ = client.create_directory(&temp_dir_path).await;

                    // Step 2: Copy to the temporary directory
                    let temp_dir_uri = path_to_uri(&temp_dir_path);
                    let copy_request = v4_models::CopyFileRequest {
                        uris: vec![src_uri.as_str()],
                        dst: temp_dir_uri.as_str(),
                    };
                    client.copy_file(&copy_request).await?;

                    // Step 3: Rename the file in temporary directory to a unique name
                    let temp_file_old_uri = path_to_uri(&format!(
                        "{}/{}",
                        temp_dir_path.trim_end_matches('/'),
                        src_name
                    ));
                    let temp_file_new_name = format!("{}_copy", src_name);
                    let rename_request = v4_models::RenameFileRequest {
                        uri: temp_file_old_uri.as_str(),
                        new_name: temp_file_new_name.as_str(),
                    };
                    let _ = client.rename_file(&rename_request).await?;

                    // Step 4: Move from temp directory to destination directory
                    let temp_file_new_uri = path_to_uri(&format!(
                        "{}/{}",
                        temp_dir_path.trim_end_matches('/'),
                        temp_file_new_name
                    ));
                    let move_request = v4_models::MoveFileRequest {
                        uris: vec![temp_file_new_uri.as_str()],
                        dst: dest_dir_uri.as_str(),
                        copy: None,
                    };
                    client.move_file(&move_request).await?;

                    // Step 5: Rename to the final destination name
                    let moved_uri = path_to_uri(&format!(
                        "{}/{}",
                        dest_dir.trim_end_matches('/'),
                        temp_file_new_name
                    ));
                    let final_rename_request = v4_models::RenameFileRequest {
                        uri: moved_uri.as_str(),
                        new_name: dest_name,
                    };
                    let _ = client.rename_file(&final_rename_request).await?;

                    // Step 6: Clean up temporary directory
                    let temp_dir_uri_for_delete = path_to_uri(&temp_dir_path);
                    let delete_request = v4_models::DeleteFileRequest {
                        uris: vec![temp_dir_uri_for_delete.as_str()],
                        unlink: None,
                        skip_soft_delete: None,
                    };
                    let _: Result<v4_models::ApiResponse<()>, _> =
                        client.delete_with_body("/file", &delete_request).await;

                    Ok(())
                } else {
                    // Standard copy operation to different directory
                    let request = v4_models::CopyFileRequest {
                        uris: vec![src_uri.as_str()],
                        dst: dest_dir_uri.as_str(),
                    };
                    client.copy_file(&request).await?;
                    Ok(())
                }
            }
        }
    }

    /// Upload a file
    ///
    /// Uploads a file to the specified path. Returns the uploaded file info.
    pub async fn upload_file(
        &self,
        path: &str,
        content: Vec<u8>,
        policy_id: Option<&str>,
    ) -> Result<(), Error> {
        debug!("Uploading file to: {}", path);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3: Need to get policy_id if not provided
                let final_policy_id = if let Some(pid) = policy_id {
                    pid.to_string()
                } else {
                    // Get policy_id from parent directory listing
                    // For V3, path should be parent directory only
                    let parent_dir = if let Some(pos) = path.rfind('/') {
                        if pos == 0 { "/" } else { &path[..pos] }
                    } else {
                        "/"
                    };
                    debug!("Getting policy_id from directory: {}", parent_dir);
                    let dir_list = client.list_directory(parent_dir).await?;
                    dir_list.policy.id
                };

                // V3 uses parent directory as path, not full file path
                let upload_dir = if let Some(pos) = path.rfind('/') {
                    if pos == 0 { "/" } else { &path[..pos] }
                } else {
                    "/"
                };
                let file_name = path.rsplit('/').next().unwrap_or("file");
                debug!("V3 upload - dir: {}, file: {}", upload_dir, file_name);
                let request = v3_models::UploadFileRequest {
                    path: upload_dir,
                    name: file_name,
                    policy_id: &final_policy_id,
                    size: content.len() as i64,
                    last_modified: 0,
                    mime_type: "",
                };
                let session = client.upload_file(&request).await?;

                // Upload single chunk (for simplicity)
                client.upload_chunk(&session.session_id, 0, content).await?;

                // Note: complete_upload is only needed for certain storage policies (like OneDrive)
                // For other policies, the upload is complete after the chunk is uploaded
                // We attempt to complete but ignore errors if it's not supported
                match client.complete_upload(&session.session_id).await {
                    Ok(_) => {}
                    Err(Error::Api { code: 40011, .. }) => {
                        // "上传会话不存在或已过期" - might mean upload already completed
                        debug!("complete_upload not needed or already completed");
                    }
                    Err(_) => {
                        // Other errors, also ignore for now
                        debug!("complete_upload returned error, ignoring");
                    }
                }

                Ok(())
            }
            UnifiedClient::V4(client) => {
                // V4: Need to get policy_id if not provided
                let final_policy_id = if let Some(pid) = policy_id {
                    pid.to_string()
                } else {
                    // Get policy_id from parent directory listing
                    let parent_dir = if let Some(pos) = path.rfind('/') {
                        if pos == 0 { "/" } else { &path[..pos] }
                    } else {
                        "/"
                    };
                    debug!("V4: Getting policy_id from directory: {}", parent_dir);
                    let list_request = v4_models::ListFilesRequest {
                        path: parent_dir,
                        page: Some(0),
                        page_size: Some(1),
                        ..Default::default()
                    };
                    match client.list_files(&list_request).await {
                        Ok(response) => response
                            .storage_policy
                            .map(|p| p.id)
                            .unwrap_or_else(|| "default".to_string()),
                        Err(_) => "default".to_string(),
                    }
                };

                // V4: Use upload session
                let request = v4_models::CreateUploadSessionRequest {
                    uri: &path_to_uri(path),
                    size: content.len() as u64,
                    policy_id: &final_policy_id,
                    last_modified: None,
                    mime_type: None,
                    metadata: None,
                    entity_type: None,
                };
                let session = client.create_upload_session(&request).await?;

                // Upload content
                client
                    .upload_file_chunk(&session.session_id, 0, &content)
                    .await?;

                Ok(())
            }
        }
    }

    /// Download a file
    ///
    /// Returns the download URL for the file.
    pub async fn download_file(&self, path: &str) -> Result<String, Error> {
        debug!("Downloading file: {}", path);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3: Need file ID, not path
                // Parse path to get parent directory and filename
                let normalized_path = if path.ends_with('/') && path != "/" {
                    &path[..path.len() - 1]
                } else {
                    path
                };

                let parent_path = if normalized_path == "/" {
                    "/"
                } else {
                    let pos = normalized_path.rfind('/');
                    match pos {
                        Some(0) => "/",
                        Some(p) => &normalized_path[..p],
                        None => "/",
                    }
                };

                let file_name = normalized_path.rsplit('/').next().unwrap_or("");

                debug!(
                    "V3: Looking for file '{}' in parent directory '{}'",
                    file_name, parent_path
                );

                // List directory to find file ID
                let dir_list = client.list_directory(parent_path).await?;
                let file_id = dir_list
                    .objects
                    .iter()
                    .find(|obj| obj.name == file_name)
                    .ok_or_else(|| Error::InvalidResponse(format!("File not found: {}", path)))?
                    .id
                    .clone();

                debug!("V3: Found file ID: {}", file_id);

                // Download using file ID
                let url_info = client.download_file(&file_id).await?;
                // Construct full URL from base_url and relative path
                let full_url = format!("{}{}", self.base_url.trim_end_matches('/'), url_info.url);
                Ok(full_url)
            }
            UnifiedClient::V4(client) => {
                let request = v4_models::CreateDownloadUrlRequest {
                    uris: vec![path],
                    download: Some(true),
                    redirect: Some(false), // 不自动重定向，返回 JSON 响应
                    entity: None,
                    use_primary_site_url: None,
                    skip_error: None,
                    archive: None,
                    no_cache: None,
                };
                let response = client.create_download_url(&request).await?;
                if let Some(first_url) = response.urls.first() {
                    Ok(first_url.url.clone())
                } else {
                    Err(Error::InvalidResponse(
                        "No download URL returned".to_string(),
                    ))
                }
            }
        }
    }

    /// Restore a file from trash
    ///
    /// Restores a file or directory from the trash. Only available in V4.
    pub async fn restore_file(&self, path: &str) -> Result<(), Error> {
        debug!("Restoring file: {}", path);

        match &self.inner {
            UnifiedClient::V3(_) => Err(Error::UnsupportedFeature(
                "restore from trash".to_string(),
                "v3".to_string(),
            )),
            UnifiedClient::V4(client) => {
                let request = v4_models::RestoreFileRequest { uris: vec![path] };
                client.restore_from_trash(&request).await?;
                Ok(())
            }
        }
    }

    /// Preview a file
    ///
    /// Returns preview information for the file. For V3, requires file ID.
    pub async fn preview_file(&self, file_id: &str) -> Result<String, Error> {
        debug!("Previewing file: {}", file_id);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3: Get preview info
                let _preview = client.preview_file(file_id).await?;
                // Return preview URL or info
                Ok(format!("Preview available for file: {}", file_id))
            }
            UnifiedClient::V4(_client) => {
                // V4 preview implementation would go here
                Err(Error::UnsupportedFeature(
                    "preview".to_string(),
                    "v4".to_string(),
                ))
            }
        }
    }

    /// Get thumbnail for a file
    ///
    /// Returns thumbnail information for the file. For V3, requires file ID.
    pub async fn get_thumbnail(&self, file_id: &str) -> Result<String, Error> {
        debug!("Getting thumbnail for file: {}", file_id);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3: Get thumbnail info
                let _thumbnail = client.get_thumbnail(file_id).await?;
                Ok(format!("Thumbnail available for file: {}", file_id))
            }
            UnifiedClient::V4(_client) => {
                // V4 thumbnail implementation would go here
                Err(Error::UnsupportedFeature(
                    "thumbnail".to_string(),
                    "v4".to_string(),
                ))
            }
        }
    }
}

/// Unified file list response
///
/// Wraps both V3 and V4 directory listing responses.
#[derive(Debug)]
pub enum FileList {
    V3(v3_models::DirectoryList),
    V4(Box<v4_models::ListResponse>),
}

impl FileList {
    /// Get parent directory name
    pub fn parent_name(&self) -> String {
        match self {
            FileList::V3(d) => d.parent.clone(), // V3: parent field is the parent ID
            FileList::V4(r) => r.parent.name.clone(),
        }
    }

    /// Get parent directory ID
    pub fn parent_id(&self) -> String {
        match self {
            FileList::V3(d) => d.parent.clone(), // V3: parent field is the parent ID
            FileList::V4(r) => r.parent.id.clone(),
        }
    }

    /// Get parent directory path
    pub fn parent_path(&self) -> String {
        match self {
            FileList::V3(_) => String::new(), // V3 doesn't provide parent path
            FileList::V4(r) => r.parent.path.clone(),
        }
    }

    /// Get storage policy ID (V4 only)
    pub fn storage_policy_id(&self) -> Option<String> {
        match self {
            FileList::V3(_) => None,
            FileList::V4(r) => r.storage_policy.as_ref().map(|p| p.id.clone()),
        }
    }

    /// Get storage policy name (V4 only)
    pub fn storage_policy_name(&self) -> Option<String> {
        match self {
            FileList::V3(_) => None,
            FileList::V4(r) => r.storage_policy.as_ref().map(|p| p.name.clone()),
        }
    }

    /// Get files and folders
    pub fn items(&self) -> Vec<FileItem> {
        match self {
            FileList::V3(d) => d
                .objects
                .iter()
                .map(|obj| FileItem {
                    name: obj.name.clone(),
                    is_folder: obj.object_type == "dir",
                    size: obj.size,
                })
                .collect(),
            FileList::V4(r) => r
                .files
                .iter()
                .map(|file| FileItem {
                    name: file.name.clone(),
                    is_folder: matches!(file.r#type, v4_models::FileType::Folder),
                    size: file.size,
                })
                .collect(),
        }
    }

    /// Get total count
    pub fn total_count(&self) -> usize {
        self.items().len()
    }

    /// Get next page token (V4 only)
    pub fn next_token(&self) -> Option<String> {
        match self {
            FileList::V3(_) => None,
            FileList::V4(r) => r.pagination.next_token.clone(),
        }
    }

    /// Get total items count from pagination (V4 only)
    pub fn total_items(&self) -> Option<i64> {
        match self {
            FileList::V3(_) => None,
            FileList::V4(r) => r.pagination.total_items,
        }
    }

    /// Check if there are more pages (V4 only)
    pub fn has_more_pages(&self) -> bool {
        self.next_token().is_some()
    }
}

/// Unified file list with automatic pagination support
///
/// This variant contains all pages combined for V4 API.
pub enum FileListAll {
    V3(v3_models::DirectoryList),
    V4(Box<v4_models::ListResponse>),
}

impl FileListAll {
    /// Get parent directory name
    pub fn parent_name(&self) -> String {
        match self {
            FileListAll::V3(d) => d.parent.clone(),
            FileListAll::V4(r) => r.parent.name.clone(),
        }
    }

    /// Get parent directory ID
    pub fn parent_id(&self) -> String {
        match self {
            FileListAll::V3(d) => d.parent.clone(),
            FileListAll::V4(r) => r.parent.id.clone(),
        }
    }

    /// Get parent directory path
    pub fn parent_path(&self) -> String {
        match self {
            FileListAll::V3(_) => String::new(),
            FileListAll::V4(r) => r.parent.path.clone(),
        }
    }

    /// Get storage policy ID (V4 only)
    pub fn storage_policy_id(&self) -> Option<String> {
        match self {
            FileListAll::V3(_) => None,
            FileListAll::V4(r) => r.storage_policy.as_ref().map(|p| p.id.clone()),
        }
    }

    /// Get storage policy name (V4 only)
    pub fn storage_policy_name(&self) -> Option<String> {
        match self {
            FileListAll::V3(_) => None,
            FileListAll::V4(r) => r.storage_policy.as_ref().map(|p| p.name.clone()),
        }
    }

    /// Get files and folders (all pages combined)
    pub fn items(&self) -> Vec<FileItem> {
        match self {
            FileListAll::V3(d) => d
                .objects
                .iter()
                .map(|obj| FileItem {
                    name: obj.name.clone(),
                    is_folder: obj.object_type == "dir",
                    size: obj.size,
                })
                .collect(),
            FileListAll::V4(r) => r
                .files
                .iter()
                .map(|file| FileItem {
                    name: file.name.clone(),
                    is_folder: matches!(file.r#type, v4_models::FileType::Folder),
                    size: file.size,
                })
                .collect(),
        }
    }

    /// Get total count (all items)
    pub fn total_count(&self) -> usize {
        self.items().len()
    }

    /// Get total items count from pagination (V4 only)
    pub fn total_items(&self) -> Option<i64> {
        match self {
            FileListAll::V3(_) => None,
            FileListAll::V4(r) => r.pagination.total_items,
        }
    }
}

/// Unified file/folder item
#[derive(Debug, Clone)]
pub struct FileItem {
    pub name: String,
    pub is_folder: bool,
    pub size: i64,
}

/// Target for delete operation
///
/// Accepts either a path or URI to provide flexibility.
#[derive(Debug, Clone)]
pub enum DeleteTarget {
    Path(String),
    Uri(String),
}

impl From<&str> for DeleteTarget {
    fn from(s: &str) -> Self {
        if s.starts_with("cloudreve://") {
            DeleteTarget::Uri(s.to_string())
        } else {
            DeleteTarget::Path(s.to_string())
        }
    }
}

impl From<String> for DeleteTarget {
    fn from(s: String) -> Self {
        if s.starts_with("cloudreve://") {
            DeleteTarget::Uri(s)
        } else {
            DeleteTarget::Path(s)
        }
    }
}

/// Unified file information response
///
/// Wraps both V3 and V4 file information responses.
#[derive(Debug, Clone)]
pub enum FileInfo {
    V3(v3_models::Object),
    V4(v4_models::File),
}

impl FileInfo {
    /// Get file name
    pub fn name(&self) -> String {
        match self {
            FileInfo::V3(obj) => obj.name.clone(),
            FileInfo::V4(file) => file.name.clone(),
        }
    }

    /// Get file size
    pub fn size(&self) -> i64 {
        match self {
            FileInfo::V3(obj) => obj.size,
            FileInfo::V4(file) => file.size,
        }
    }

    /// Check if it's a folder
    pub fn is_folder(&self) -> bool {
        match self {
            FileInfo::V3(obj) => obj.object_type == "dir",
            FileInfo::V4(file) => matches!(file.r#type, v4_models::FileType::Folder),
        }
    }

    /// Get file path
    pub fn path(&self) -> String {
        match self {
            FileInfo::V3(obj) => obj.path.clone(),
            FileInfo::V4(file) => file.path.clone(),
        }
    }

    /// Get created date
    pub fn created_at(&self) -> String {
        match self {
            FileInfo::V3(obj) => obj.create_date.clone(),
            FileInfo::V4(file) => file.created_at.clone(),
        }
    }

    /// Get updated date
    pub fn updated_at(&self) -> String {
        match self {
            FileInfo::V3(obj) => obj.date.clone(),
            FileInfo::V4(file) => file.updated_at.clone(),
        }
    }
}

// Private methods for batch_delete
impl super::CloudreveAPI {
    async fn batch_delete_v3(
        &self,
        client: &crate::api::v3::ApiV3Client,
        paths: &[&str],
    ) -> Result<DeleteResult, Error> {
        let mut result = DeleteResult::default();

        // Group paths by parent directory to minimize API calls
        use std::collections::HashMap;
        let mut parent_groups: HashMap<&str, Vec<&str>> = HashMap::new();

        for path in paths {
            // Normalize path
            let normalized = if path.ends_with('/') && *path != "/" {
                &path[..path.len() - 1]
            } else {
                *path
            };

            // Get parent directory
            let parent = if normalized == "/" {
                return Err(Error::InvalidResponse(
                    "Cannot delete root directory".to_string(),
                ));
            } else {
                let pos = normalized.rfind('/');
                match pos {
                    Some(0) => "/",
                    Some(p) => &normalized[..p],
                    None => "/",
                }
            };

            parent_groups.entry(parent).or_default().push(normalized);
        }

        // For each parent directory, list once and delete all items
        for (parent_dir, items) in parent_groups {
            let dir_list = match client.list_directory(parent_dir).await {
                Ok(list) => list,
                Err(e) => {
                    // All items in this group failed
                    result.failed += items.len();
                    for item in &items {
                        result.errors.push((item.to_string(), e.to_string()));
                    }
                    continue;
                }
            };

            // Find IDs for all items and separate into files and folders
            let mut file_ids = Vec::new();
            let mut folder_ids = Vec::new();

            for item_path in &items {
                let file_name = item_path.rsplit('/').next().unwrap_or("");

                match dir_list.objects.iter().find(|obj| obj.name == file_name) {
                    Some(obj) => {
                        if obj.object_type == "dir" {
                            folder_ids.push(obj.id.as_str());
                        } else {
                            file_ids.push(obj.id.as_str());
                        }
                    }
                    None => {
                        result.failed += 1;
                        result
                            .errors
                            .push((item_path.to_string(), "File not found".to_string()));
                    }
                }
            }

            // Delete all files and folders in one API call
            if !file_ids.is_empty() || !folder_ids.is_empty() {
                let item_count = file_ids.len() + folder_ids.len();
                let request = v3_models::DeleteObjectRequest {
                    items: file_ids,
                    dirs: folder_ids,
                    force: true,
                    unlink: false,
                };

                match client.delete_object(&request).await {
                    Ok(_) => {
                        result.deleted += item_count;
                    }
                    Err(e) => {
                        result.failed += item_count;
                        for item_path in &items {
                            result.errors.push((item_path.to_string(), e.to_string()));
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    async fn batch_delete_v4(
        &self,
        client: &crate::api::v4::ApiV4Client,
        paths: &[&str],
    ) -> Result<DeleteResult, Error> {
        let mut result = DeleteResult::default();

        // Convert all paths to URIs
        let uris: Vec<String> = paths
            .iter()
            .map(|p| crate::api::v4::uri::path_to_uri(p))
            .collect();

        let uri_refs: Vec<&str> = uris.iter().map(|s| s.as_str()).collect();

        let request = v4_models::DeleteFileRequest {
            uris: uri_refs,
            unlink: None,
            skip_soft_delete: None,
        };

        let response: v4_models::ApiResponse<()> =
            client.delete_with_body("/file", &request).await?;
        match response.code {
            0 => {
                result.deleted = paths.len();
            }
            code => {
                // On error, fall back to individual deletion
                debug!(
                    "Batch delete failed (code {}), falling back to individual deletion",
                    code
                );
                for (path, uri) in paths.iter().zip(uris.iter()) {
                    let single_request = v4_models::DeleteFileRequest {
                        uris: vec![uri.as_str()],
                        unlink: None,
                        skip_soft_delete: None,
                    };
                    let result_: Result<v4_models::ApiResponse<()>, Error> =
                        client.delete_with_body("/file", &single_request).await;
                    match result_ {
                        Ok(resp) if resp.code == 0 => result.deleted += 1,
                        Ok(resp) => {
                            result.failed += 1;
                            result
                                .errors
                                .push((path.to_string(), format!("API error code: {}", resp.code)));
                        }
                        Err(e) => {
                            result.failed += 1;
                            result.errors.push((path.to_string(), e.to_string()));
                        }
                    }
                }
            }
        }

        Ok(result)
    }
}
