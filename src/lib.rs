use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use clap::{Parser, Subcommand};


/// Environment variable that can overwrite the bookmarks file path.
pub const BM_ENV: &str = "WDC_BOOKMARK_FILE";
/// The name of the file where bookmarks are stored.
pub const BM_FILENAME: &str = ".bookmarks";
/// The delimiter used to separate the bookmark name from the path.
pub const DELIM: &str = "|";

/// A bookmark is just a name and the directory it points to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bookmark {
    pub name: String,
    pub path: String,
}

impl Bookmark {
    /// Create a new Bookmark from name and path.
    fn new(name: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
        }
    }

    /// Parse a single line from the bookmarks file.
    fn from_line(line: &str) -> Option<Self> {
        let (name, path) = line.split_once(DELIM)?;
        Some(Self::new(name, path))
    }

    /// Serialise back to the line format used in the file.
    fn to_line(&self) -> String {
        format!("{}{}{}", self.name, DELIM, self.path)
    }
}

/// Gets the path to the bookmarks file.
/// Use the environment variable BM_ENV if it's defined.
/// Otherwise, look in the user's home directory for `BM_FILENAME`.
/// If there is no `HOME` directory, then look in the current directory.
pub fn bookmark_file_path() -> io::Result<PathBuf> {
    Ok(env::var(BM_ENV)
       .map(PathBuf::from)
       .unwrap_or_else(|_| {
	   env::home_dir().or(env::current_dir().ok())
	       .unwrap_or_else(||
			       ".".into()).join(BM_FILENAME)
       }))
}

/// Load every bookmark in the file, newest → oldest.
/// Invalid lines are silently ignored.
pub fn load_bookmarks<P: AsRef<Path>>(path: P) -> io::Result<Vec<Bookmark>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut bookmarks: Vec<_> = reader
        .lines()
        .filter_map(|line| line.ok().and_then(|l| Bookmark::from_line(&l)))
        .collect();

    bookmarks.reverse();
    Ok(bookmarks)
}

/// Add a bookmark for the current working directory.
pub fn add_bookmark(name: &str) -> io::Result<()> {
    let path = env::current_dir()?.display().to_string();
    let bookmark = Bookmark::new(name, path);

    let file_path = bookmark_file_path()?;
    let mut file = File::options()
        .create(true)
        .append(true)
        .open(&file_path)?;
    writeln!(file, "{}", bookmark.to_line())?;
    Ok(())
}


/// List all bookmarks to stdout (newest first).
pub fn list_bookmarks() -> io::Result<()> {
    let bookmarks = load_bookmarks(bookmark_file_path()?)?;
    for bm in bookmarks {
        println!("{}{}{}", bm.name, DELIM, bm.path);
    }
    Ok(())
}

/// Persist the given bookmarks to disk, oldest → newest.
pub fn save_bookmarks<P: AsRef<Path>>(path: P, bookmarks: &[Bookmark]) -> io::Result<()> {
    let mut file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    for bm in bookmarks.iter().rev() {
        writeln!(file, "{}", bm.to_line())?;
    }
    Ok(())
}

/// Print the path for the given bookmark name.
pub fn find_bookmark(name: &str) -> io::Result<Option<String>> {
    let bookmarks = load_bookmarks(bookmark_file_path()?)?;
    Ok(bookmarks
        .into_iter()
        .find(|bm| bm.name == name)
        .map(|bm| bm.path))
}


/// Remove the newest bookmark and return its path.
pub fn pop_bookmark() -> io::Result<Option<String>> {
    let file_path = bookmark_file_path()?;
    let mut bookmarks = load_bookmarks(&file_path)?;

    let popped = bookmarks.first().cloned();
    bookmarks.remove(0);

    save_bookmarks(file_path, &bookmarks)?;
    Ok(popped.map(|bm| bm.path))
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a bookmark.
    Add { name: String },
    /// List all bookmarks.
    List,
    /// Find a bookmark.
    Find { name: String },
    /// Pop the last added bookmark.
    Pop,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::path::PathBuf;

    #[test]
    fn test_bookmark_file_path_defaults() {
	// Clear the env variable.
	unsafe { std::env::remove_var("WDC_BOOKMARK_FILE"); }
	let path = bookmark_file_path().unwrap();
	let home_dir = env::home_dir().expect("Home directory not found");
	let expected = home_dir.join(BM_FILENAME);
	assert_eq!(path, expected);
    }
    #[test]
    fn test_load_bookmarks_empty_file() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join(".bookmarks");
	// Create an empty file.
	let _file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file_path);
	unsafe { std::env::set_var("WDC_BOOKMARK_FILE", &file_path); }
        let result = load_bookmarks(&file_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_add_bookmark_creates_file() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join(".bookmarks");
	// Create an empty file.
	let _file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file_path);
	unsafe { std::env::set_var("WDC_BOOKMARK_FILE", &file_path); }
        add_bookmark("test_bookmark").unwrap();
        let contents = std::fs::read_to_string(file_path).unwrap();
        assert!(contents.contains("test_bookmark|"));
    }
}
