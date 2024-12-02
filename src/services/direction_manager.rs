use std::fs;

pub fn find_all_files_inside_direction(path: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.file_type().unwrap().is_file() {
                        files.push(entry.path().to_string_lossy().into_owned());
                        /*
                        to_string_lossy():
                            Converts a Path or OsStr to a Cow<str>.
                            Replaces any non-Unicode sequences with U+FFFD REPLACEMENT CHARACTER.
                            Returns a Cow::Borrowed if the input is valid UTF-8, or Cow::Owned otherwise.

                        into_owned():
                            Converts a Cow<T> into an owned T.
                            Used to obtain an owned String from the result of to_string_lossy().
                         */
                    } else if entry.file_type().unwrap().is_dir() {
                        files.extend(find_all_files_inside_direction(
                            entry.path().to_str().unwrap(),
                        ));
                        /*
                        to_str():
                            Attempts to convert a Path or CStr to a &str slice.
                            Returns None if the input contains invalid Unicode.

                        unwrap():
                            Extracts the value from an Option or Result5.
                            Panics if the value is None or Err5.
                         */
                    }
                }
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }

    files
}
