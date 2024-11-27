use std::ffi::OsStr;
use std::path::Path;
use std::collections::HashMap;

pub fn get(file: String) -> String {
    let extension = Path::new(&file[..]).extension().and_then(OsStr::to_str);

    extension.unwrap_or("").to_string()
}

pub fn get_ext_list(files: Vec<String>) -> Vec<String> {
    let mut types: Vec<String> = Vec::new();

    for file in files {
        types.push(String::from(get(file)));
    }

    types
}

pub fn remove_duplication(exts: Vec<String>) -> Vec<String> {
    let mut found_exts: Vec<String> = Vec::new();

    for ext in exts {
        if !found_exts.contains(&ext) {
            found_exts.push(ext)
        }
    }

    found_exts
}

pub fn get_json(exts: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut extensions = HashMap::new();

    for ext in exts {
        extensions.insert(ext, Vec::new());
    }

    extensions
}
