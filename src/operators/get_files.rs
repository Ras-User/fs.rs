use  crate::services;

use std::fs::File;
use std::io::Write;
use services::extension_manager;
use services::direction_manager;
use services::file_manager;
use std::collections::HashMap;

pub fn categorize()-> Result<(), Box<dyn std::error::Error>>{
    let path = "./src";
    let files:Vec<String> = worker(path);

    let extensions: Vec<String> = extension_manager::get_ext_list((&files).to_vec());

    let no_duplicate_ext_list: Vec<String> = extension_manager::remove_duplication(extensions);

    let json: HashMap<String, Vec<String>> = extension_manager::get_json(no_duplicate_ext_list);

    let categorized_json = file_manager::categorize(files, json);

    let string_cat_json = serde_json::to_string(&categorized_json).unwrap();

    let mut file = File::create("output.json")?;
    file.write_all(string_cat_json.as_bytes())?;

    Ok(())   
}

pub fn list(path:&str) -> String{
    let files:Vec<String> = worker(path);

    let extensions: Vec<String> = extension_manager::get_ext_list((&files).to_vec());

    let no_duplicate_ext_list: Vec<String> = extension_manager::remove_duplication(extensions);

    let json: HashMap<String, Vec<String>> = extension_manager::get_json(no_duplicate_ext_list);

    let categorized_json = file_manager::categorize(files, json);

    let string_cat_json = serde_json::to_string(&categorized_json).unwrap();

    string_cat_json
}

fn worker(path:&str) -> Vec<String>{
    let files = direction_manager::find_all_files_inside_direction(path);

    files
}

/* Tutorial:
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

Why &var_name ???
    Remove & before "for file in '&'files". You'll see an error.
    The error message "value used here after move" occurs because in Rust,
    when you pass a value to a function, it is moved by default unless it
    implements the Copy trait. In your code, the files vector is moved into
    the divide_by_ext function, which means you cannot use files again
    after this call.
*/
