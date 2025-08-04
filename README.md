# wdr

A simple command-line utility for managing directory bookmarks, written in Rust.

## Description

`wdr` (Working Directory Recorder) allows you to save, list, find, and remove directory bookmarks. It's useful for quickly navigating between frequently used directories in your terminal.

## Features

- Add bookmarks for current working directories
- List all saved bookmarks
- Find a specific bookmark and print its path
- Remove the most recently added bookmark (pop operation)
- Persistent storage using a simple text file
- Environment variable override for bookmark file location

## Installation

To install `wdr`, you need to have Rust and Cargo installed on your system. Then:

```bash
git clone https://github.com/jjwatt/wdr.git
cd wdr
cargo install --path .
```

## Usage

```bash
wdr <COMMAND>
```

### Commands

- `add <NAME>` - Add a bookmark for the current directory with the given name
- `list` - List all bookmarks (newest first)
- `find <NAME>` - Find and print the path for the given bookmark name
- `pop` - Remove and print the most recently added bookmark

### Examples

```bash
# Add a bookmark for the current directory
wdr add project1

# List all bookmarks
wdr list

# Find a bookmark
wdr find project1

# Remove the most recent bookmark
wdr pop
```

### Environment Variable

You can customize the location of the bookmarks file using the `WDC_BOOKMARK_FILE` environment variable:

```bash
export WDC_BOOKMARK_FILE="/path/to/your/bookmarks/file"
```

By default, bookmarks are stored in `~/.bookmarks`.

## File Format

The bookmarks file is a simple text file where each line represents a bookmark in the format:

```
name|/path/to/directory
```

## Dependencies

- [clap](https://crates.io/crates/clap) - For command-line argument parsing
- [tempfile](https://crates.io/crates/tempfile) - For testing with temporary files
- [serial_test](https://crates.io/crates/serial_test) - For serializing tests

## License

This project is licensed under the MIT License - see the LICENSE file for details.