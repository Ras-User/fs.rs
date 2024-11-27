use std::collections::HashMap;
use super::extension_manager;
use std::fs::File;
use std::io::BufReader;
use crate::struction::config::Config;

pub fn categorize(
    files: Vec<String>,
    json: HashMap<String, Vec<String>>,
) -> HashMap<String, Vec<String>> {
    let mut json: HashMap<String, Vec<String>> = json;

    for file in files {
        let ext: String = extension_manager::get((&file).to_string());

        if json.contains_key(ext.as_str()) {
            json.entry(ext).or_insert_with(Vec::new).push(file);
        }
    }

    json
}

pub fn read_config(address:String) -> Config{
    let file = File::open(address).expect("Failed to open config file");
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).expect("Failed to parse config");

    config
}