# add_timestamp_to_textfiles

## Description
Permanently writing the "time and date created" of text-files into the files themselves, beautifully formatted with the "de_CH" locale, while also preserving the file timestamps. The "modified" timestamp is used for this purpose.

Works on individual files as well as folders and unlimited subfolders. 

## Limitation

Currently only works on folders with simple text-files.

## Use case
In my very peculicar use case, it's about a large number of personal Markdown diary-files that accumulated over the years where I want to permanently preserve the date and time, which I was usually too lazy to add to the files while initially writing them. 

File system timestamps are useful, but they tend to get lost when eg. transferring files or restoring from backup. Secondly, whenever you add something to a previous diary-note, you'll lose the information as of when the note was originally written.
