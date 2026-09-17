//#![feature(fs_set_times)]
use std::fs::{self};
use std::env;
use std::path::PathBuf;
use std::process;
use std::error::Error;

use chrono::{Local};

use add_timestamp_to_textfiles::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: '{e}'");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Current date-time
    let now = Local::now()
        .format("%Y-%m-%d, %H:%M");
    
    // Check if path is a file or a folder, for both are accepted
    let metadata = fs::metadata(&config.file_or_folder_path)?;

    if metadata.is_file() {
        process_file(PathBuf::from(&config.file_or_folder_path), &now)?;
    } else if metadata.is_symlink() {
        todo!();
    }
    else if metadata.is_dir() {
        process_dir(&PathBuf::from(&config.file_or_folder_path), &now)?;
    }

    Ok(())
}

struct Config {
    file_or_folder_path: String,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {     
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        
        let file_or_folder_path = args[1].clone();

        Ok(Config {
            file_or_folder_path,
        })
    }
}
