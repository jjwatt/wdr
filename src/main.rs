use clap::Parser;
use wdr::{Cli, Commands, add_bookmark, bookmark_file_path, find_bookmark, list_bookmarks, pop_bookmark};

fn main() {
    let path = match bookmark_file_path() {
	Ok(path) => path,
	Err(e)=> {
	    eprintln!("Failed to determine bookmark file path: {}", e);
	    return;
	}
    };

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { name } => {
            if let Err(e) = add_bookmark(name, &path) {
                eprintln!("Failed to add bookmark: {}", e);
            }
        }
        Commands::List => {
            if let Err(e) = list_bookmarks(&path) {
                eprintln!("Error listing bookmarks: {}", e);
            }
        }
        Commands::Find { name } => {
            match find_bookmark(name, &path) {
                Ok(Some(found_path)) => println!("{}", found_path),
                Ok(None) => println!("Bookmark not found."),
                Err(e) => eprintln!("Error finding bookmark: {}", e),
            }
        }
        Commands::Pop => {
            match pop_bookmark(&path) {
                Ok(Some(popped_path)) => println!("{}", popped_path),
                Ok(None) => println!("No bookmarks to pop."),
                Err(e) => eprintln!("Error popping bookmark: {}", e),
            }
        }
    }
}
