use wdr::*;
use tempfile::{TempDir, NamedTempFile};
use std::fs;
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
