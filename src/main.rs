use clap::Parser;
use wdr::{Cli, Commands, add_bookmark, list_bookmarks, find_bookmark, pop_bookmark};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { name } => {
            if let Err(e) = add_bookmark(name) {
                eprintln!("Failed to add bookmark: {}", e);
            }
        }
        Commands::List => {
            if let Err(e) = list_bookmarks() {
                eprintln!("Error listing bookmarks: {}", e);
            }
        }
        Commands::Find { name } => {
            match find_bookmark(name) {
                Ok(Some(path)) => println!("{}", path),
                Ok(None) => println!("Bookmark not found."),
                Err(e) => eprintln!("Error finding bookmark: {}", e),
            }
        }
        Commands::Pop => {
            match pop_bookmark() {
                Ok(Some(path)) => println!("{}", path),
                Ok(None) => println!("No bookmarks to pop."),
                Err(e) => eprintln!("Error popping bookmark: {}", e),
            }
        }
    }
}
