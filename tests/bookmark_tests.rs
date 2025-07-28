use wdr::*;
use tempfile::{TempDir, NamedTempFile};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::env;

/// Helper function to create a temp file with known bookmark data
fn create_temp_bookmarks_file(path: &Path, bookmarks: &[Bookmark]) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    for bookmark in bookmarks {
	writeln!(file, "{}{}", bookmark.name, DELIM)?;
	writeln!(file, "{}", bookmark.path)?;
    }
    Ok(())
}

/// Test bookmark_file_path
#[test]
fn test_bookmark_file_path() {
    // Set environment variable for testing.
    unsafe { env::set_var("WDC_BOOKMARK_FILE", "/test/path"); }
    let path = bookmark_file_path().unwrap();
    assert_eq!(path, Path::new("/test/path"));
}

#[test]
fn test_load_bookmarks_empty_file() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join(".bookmarks");
    // Create an empty file.
    let _file = File::create(&file_path).unwrap();
    let result = load_bookmarks(&file_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_add_bookmark_creates_file() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join(".bookmarks");
    // Create an empty file.
    let mut file = File::create(&file_path).unwrap();
    add_bookmark("test_bookmark", &file_path).unwrap();
    let contents = std::fs::read_to_string(file_path).unwrap();
    assert!(contents.contains("test_bookmark|"));
}
