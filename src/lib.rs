use std::fs::{self, FileTimes};
use std::error::Error;
use std::path::PathBuf;

use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{DateTime, Local, Locale};

pub fn process_dir(
    file_or_folder_path: &PathBuf,
    now: &DelayedFormat<StrftimeItems>,
    mut counter: &mut u32,
    mut err_counter: &mut u32,
    mut dir_counter: &mut u32,
) -> Result<(), Box<dyn Error>> {
    *dir_counter += 1;

    for entry in fs::read_dir(file_or_folder_path)? {
        // Progress output
        println!("Processing file nr {}\x1B[1A\x1B[K", *counter + *err_counter + 1);
        
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_dir() {
            process_dir(
                &file_path,
                &now,
                &mut counter,
                &mut err_counter,
                &mut dir_counter
            )?;
        } else if file_path.is_file() {
            match process_file(file_path, &now, &mut counter, &mut err_counter) {
                Ok(_) => (),
                Err(_) => continue,
            };
        } else {
            // Is Symlink
            todo!();
        } 
    }

    Ok(())
}

pub fn process_file(
    file_path: PathBuf, 
    now: &DelayedFormat<StrftimeItems>,
    counter: &mut u32,
    err_counter: &mut u32
) -> Result<(), Box<dyn Error>> {
    let contents_result = fs::read_to_string(&file_path);

    let contents = match contents_result {
        Ok(contents) => contents,
        Err(error) => { 
                *err_counter += 1;
                return Err(Box::new(error));
        }
    };

    // Skip the process on files where it already has been done 
    // Criterion: Last line begins with "(*)"
    if contents.lines().last().unwrap_or("something went wrong").starts_with("(*) ") {
        // IDEA: Use separate counter for this "skipping reason".
        *err_counter += 1;
        return Ok(())
    }
    
    let modified = fs::metadata(&file_path)?.modified()?;
    let modified_datetime: DateTime<Local> = modified.into();
    let date = modified_datetime
        .format_localized("%A, %d. %B %Y", Locale::de_CH);
    let time = modified_datetime
        .format_localized("%H:%M", Locale::de_CH);

    let contents_new = format!("##### {} (*)\n\n {} \n\n\n\
    (*) The date on the first line of this text-file has been automatically injected based on the \
    *modified* timestamp value as it was encountered on the file on {} — without altering the timestamp. \
    Magic. The exact time of day of the timestamp was {}.", date, contents, now, time);

    // Overwrite file
    fs::write(&file_path, contents_new)?;

    // Reset the modified timestamp to before the date-time-injection into the file
    let reset = FileTimes::new()
        .set_modified(modified);
    fs::set_times(file_path, reset)?;

    // File successfully written
    *counter += 1;
    Ok(())
}