# add_timestamp_to_textfiles

## Use case
In my very peculicar use case, it's about a large number of personal Markdown diary-files that accumulated over the years where I want to permanently preserve the date and time, which I was usually too lazy to add to the files while initially writing them. 

File system timestamps are useful, but they tend to get lost when eg. transferring files, changing filesystems or restoring from backup. In addition to that, whenever you add something to a previous diary-note, you'll likely lose the information as of when the note was originally written.

## Description
Permanently write the *modified* timestamp of text-files into the files themselves, formatted with the "de_CH" locale (German Switzerland), while preserving the timestamp in the file metadata.

Works on individual files as well as folders and unlimited subfolders. 

## Usage
Download the binary in the release section, make it executable and then run `./add_timestamp_to_textfiles <file or folder path>`
If you want to compile from source you need Rust nightly. Set it up for this specific project by running the following command in the project directory: `rustup override set nightly`
