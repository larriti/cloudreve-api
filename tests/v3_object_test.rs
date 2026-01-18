use cloudreve_api::Result;
use cloudreve_api::api::v3::models::*;

#[cfg(test)]
mod v3_object_tests {
    use super::*;

    #[tokio::test]
    async fn test_rename_object_request_struct() -> Result<()> {
        let _rename_request = RenameObjectRequest {
            action: "rename",
            src: SourceItems {
                dirs: vec![],
                items: vec!["dmLxcN"],
            },
            new_name: "new_name.txt",
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_move_object_request_struct() -> Result<()> {
        let _move_request = MoveObjectRequest {
            action: "move",
            src_dir: "/",
            src: SourceItems {
                dirs: vec![],
                items: vec!["JLaoI9"],
            },
            dst: "/demo02",
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_copy_object_request_struct() -> Result<()> {
        let _copy_request = CopyObjectRequest {
            src_dir: "/a",
            src: SourceItems {
                dirs: vec![],
                items: vec!["xxxxxx"],
            },
            dst: "/b",
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_object_request_struct() -> Result<()> {
        let _delete_request = DeleteObjectRequest {
            items: vec!["EWlPFJ"],
            dirs: vec![],
            force: false,
            unlink: false,
        };
        Ok(())
    }

    #[test]
    fn test_source_items_struct() {
        let items = SourceItems {
            dirs: vec!["folder1", "folder2"],
            items: vec!["file1", "file2", "file3"],
        };

        assert_eq!(items.dirs.len(), 2);
        assert_eq!(items.items.len(), 3);
        assert_eq!(items.dirs[0], "folder1");
    }

    #[test]
    fn test_property_struct() {
        let property = Property {
            created_at: "2024-05-01T11:04:25.491493+08:00".to_string(),
            updated_at: "2024-05-01T11:04:25.491493+08:00".to_string(),
            policy: "".to_string(),
            size: 1597,
            child_folder_num: 1,
            child_file_num: 1,
            path: "".to_string(),
            query_date: "2024-05-01T11:20:50.7077079+08:00".to_string(),
        };

        assert_eq!(property.size, 1597);
        assert_eq!(property.child_folder_num, 1);
        assert_eq!(property.child_file_num, 1);
    }
}
