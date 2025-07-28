use wdr::*;
use std::path::{Path, PathBuf};
use tempfile::{TempDir, NamedTempFile};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::env;
use serial_test::serial;

fn with_test_env<F>(test_body: F)
where
    // The test body is a closure that gets the path to the temp file.
    F: FnOnce(PathBuf),
{
    let original_val = env::var(BM_ENV);
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join(".test_bookmarks");
    // Set the env var just for this test.
    unsafe { std::env::set_var(BM_ENV, &file_path); }
    test_body(file_path);
    match original_val {
	Ok(val) => unsafe { std::env::set_var(BM_ENV, val) },
	Err(_) => unsafe  { std::env::remove_var(BM_ENV) },
    }
}

#[serial]
#[test]
fn test_bookmark_file_path_env_override() {
    with_test_env(|custom_path| {
	let path = bookmark_file_path().unwrap();
	assert_eq!(path, custom_path);
    });
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
