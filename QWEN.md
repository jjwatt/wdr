# WDR - Warp Directory in Rust

This project is a command-line tool for managing directory bookmarks.
It allows users to save and navigate to frequently visited directories
using named bookmarks.

** Features

- *Bookmark Management*: Add, list, find, and remove directory
   bookmarks
- *Flexible Storage*: Bookmarks stored in a file (default:
   ~~/.bookmarks~ or custom path via environment variable)
- *Environment Override*: Use ~WDC_BOOKMARK_FILE~ to specify a custom
    bookmarks file location
- *CLI Interface*: Simple command-line interface for bookmark
    operations

** Core Functions

*** ~bookmark_file_path()~
Determines the path to the bookmarks file:
- Uses ~WDC_BOOKMARK_FILE~ environment variable if set
- Otherwise, looks in home directory (~~/.bookmarks~)
- Falls back to current directory if no home directory is available

*** ~load_bookmarks()~
Reads and parses bookmarks from file:
- Ignores invalid lines silently
- Returns bookmarks sorted newest first

*** ~add_bookmark()~
Adds a new bookmark for current working directory:
- Creates file if it doesn't exist
- Appends new bookmark to end of file

*** ~list_bookmarks()~
Prints all bookmarks to stdout, newest first.

*** ~find_bookmark()~
Searches for bookmark by name and returns its path.

*** ~pop_bookmark()~
Removes and returns the path of the newest bookmark.

** File Format

Bookmarks are stored as text lines in the format:
#+END_SRC
bookmark_name|/path/to/directory
#+END_SRC

Invalid lines are ignored during loading.

** CLI Commands

- ~wdr add <name>~: Add current directory as a bookmark
- ~wdr list~: List all bookmarks (newest first)
- ~wdr find <name>~: Find and print path of bookmark
- ~wdr pop~: Remove and print newest bookmark

** Environment Variables

- ~WDC_BOOKMARK_FILE~: Override default bookmarks file location

** Example Usage
#+BEGIN_SRC bash
# Add a bookmark
wdr add projects

# List all bookmarks
wdr list

# Find a specific bookmark
wdr find projects

# Navigate to bookmarked directory
cd $(wdr find projects)
#+END_SRC
