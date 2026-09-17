use std::fs::{self, FileTimes};
use std::error::Error;
use std::path::PathBuf;

use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{DateTime, Local, Locale};

pub fn process_dir(file_or_folder_path: &PathBuf, now: &DelayedFormat<StrftimeItems>) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(file_or_folder_path)? {
        // TODO
        // Exclude hidden files and other unwanted filetypes

        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_dir() {
            process_dir(&file_path, &now)?;
        } else if file_path.is_file() {
            process_file(file_path, &now)?;
        } else {
            // Is Symlink
            todo!();
        } 
    }

    Ok(())
}

pub fn process_file(file_path: PathBuf, now: &DelayedFormat<StrftimeItems>) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&file_path)?;
            
    // Skip the process on files where it already has been done 
    // Criterion: Last line begins with "(*)"
    if contents.lines().last().unwrap_or("something went wrong").starts_with("(*) ") {
        return Ok(())
    }
    
    let modified = fs::metadata(&file_path)?.modified()?;
    let modified_datetime: DateTime<Local> = modified.into();
    let datetime = modified_datetime
        .format_localized("%A, %d. %B %Y", Locale::de_CH);

    let contents_new = format!("{} (*)\n\n{}\n\n\n\
    (*) Date and time on the first line of this text-file have been automatically injected based on the \
    \"modified\" timestamp value as it was encountered on {}; without altering the timestamp. \
    Magic.", datetime, contents, now);

    // Overwrite file
    fs::write(&file_path, contents_new)?;

    // Reset the modified timestamp to before the date-time-injection into the file
    let reset = FileTimes::new()
        .set_modified(modified);
    fs::set_times(file_path, reset)?;

    Ok(())
}