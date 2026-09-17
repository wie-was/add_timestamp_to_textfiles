//#![feature(fs_set_times)]
use std::fs::{self, FileTimes};
//use std::path::{self, Path};

use std::env;
use std::process;
use std::error::Error;

use chrono::{DateTime, Local, Locale};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: '{e}");
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
        //let modified = metadata.modified()?;
        todo!();
    } else if metadata.is_symlink() {
        todo!();
    }
    else if metadata.is_dir() {
        for entry in fs::read_dir(&config.file_or_folder_path)? {
            
            // TODO
            // Exclude hidden files and other unwanted filetypes

            let entry = entry?;
            let file_path = entry.path();

            if file_path.is_dir() {
                todo!();
            }

            let contents = fs::read_to_string(&file_path)?;
            
            // Skip the process on files where it already has been done 
            // Criterion: Last line begins with "(*)"
            if contents.lines().last().unwrap_or("something went wrong").starts_with("(*) ") {
                continue;
            }
            
            let modified = fs::metadata(&file_path)?.modified()?;
            let modified_datetime: DateTime<Local> = modified.into();
            let datetime = modified_datetime
                .format_localized("%A, %d. %B %Y", Locale::de_CH);

            let contents_new = format!("{} (*)\n\n{}\n\n\n\
            (*) The date and time on the first line of this text-file have been automatically injected based on the \
            \"modified\" timestamp value as it was encountered on {}; without altering the timestamp. \
            Magic.", datetime, contents, now);

            // Overwrite file
            fs::write(&file_path, contents_new)?;

            // Reset the modified timestamp to before the date-time-injection into the file
            let reset = FileTimes::new()
                .set_modified(modified);
            fs::set_times(file_path, reset)?;
        }
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
