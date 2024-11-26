use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::collections::HashMap;

fn main() {
    let path = "./src";
    let files = find_all_files_inside_direction(path);

    for file in &files {
        println!("{}", file);
    }

    /*
    Why &var_name ??? 
    Remove & before "for file in '&'files". You'll see an error.
    The error message "value used here after move" occurs because in Rust,
    when you pass a value to a function, it is moved by default unless it
    implements the Copy trait. In your code, the files vector is moved into
    the divide_by_ext function, which means you cannot use files again
    after this call.
    */

    let extensions = divide_by_ext(files);

    let no_duplicate_ext_list = remove_duplication(extensions);

    let json = create_json_from_exts(no_duplicate_ext_list);

    let json_string = serde_json::to_string(&json).unwrap();

    println!("{}", json_string);
    

     
}

fn find_all_files_inside_direction(path: &str) -> Vec<String> {
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
                        files.extend(find_all_files_inside_direction(entry.path().to_str().unwrap()));
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
        },
        Err(e) => eprintln!("Error reading directory: {}", e),
    }

    files
}

fn divide_by_ext(files:Vec<String>) -> Vec<String>{
    let mut types:Vec<String> = Vec::new();

    for file in files{
        types.push(String::from(get_file_extension(file)));
    }

    types
}

fn get_file_extension(file:String) -> String{
   let extension =  Path::new(&file[..]).extension().and_then(OsStr::to_str);

   extension.unwrap_or("").to_string()
}

fn remove_duplication(exts: Vec<String>) -> Vec<String> {
    let mut found_exts: Vec<String> = Vec::new();

    for ext in exts {
        
        if !found_exts.contains(&ext) {found_exts.push(ext)}
    }

    found_exts
}

fn create_json_from_exts(exts:Vec<String>) -> HashMap<String , Vec<String>>{

    let mut extensions = HashMap::new();

    for ext in exts{
        extensions.insert(ext,Vec::new());
    }

    extensions
}

/*
&str vs String:
    &str is a string slice, a reference to UTF-8 encoded string data.
    String is an owned, growable UTF-8 encoded string.

to_str() vs to_string_lossy():
    to_str() returns Option<&str>, failing for non-Unicode data.
    to_string_lossy() always succeeds, replacing invalid Unicode with a replacement character.
    Use to_str() when you need to ensure valid Unicode, and to_string_lossy() when you want to handle potential non-Unicode data.

The match keyword in Rust is a powerful control flow operator used for pattern matching.
In the context of match fs::read_dir(path), match is comparing the result of fs::read_dir(path) against a series of patterns.
match allows you to:
Compare a value against multiple patterns.
Execute code based on which pattern matches.
Handle all possible cases, ensuring exhaustiveness.
*/