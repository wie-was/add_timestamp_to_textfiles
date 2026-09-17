//#![feature(fs_set_times)]
use std::fs::{self, FileTimes};

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
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let modified = fs::metadata(&config.file_path)?
        .modified()?;

    let modified_datetime: DateTime<Local> = modified.into();

    let datetime = modified_datetime
        .format_localized("%A, %d. %B %Y, %H:%M", Locale::de_CH);

    let now = Local::now()
        .format("%Y-%m-%d, %H:%M");


    let contents = format!("{} (*)\n\n{}\n\n\
    (*) The date and time on the first line of this text file have been automatically injected based on the \
    \"modified\" timestamp value as it was encountered on {}. The timestamp of the file was then reset \
    to this value after this text has been injected. \
    ", datetime, contents, now);

    // Overwrite file
    fs::write(&config.file_path, contents)?;

    // Reset the modified timestamp to before the date-time-injection into the file
    let reset = FileTimes::new()
        .set_modified(modified);
    fs::set_times(&config.file_path, reset)?;

    Ok(())
}

struct Config {
    file_path: String,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {     
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        
        let file_path = args[1].clone();

        Ok(Config {
            file_path,
        })
    }
}
