use super::get_files;
use std::collections::HashMap;
// use crate::services::file_manager;
use std::fs::File;
use std::io::{self, BufReader, Read};

pub fn gather_html_content(path:&str){
    
    let parsed :Vec<String> = get_html_file_list(path);

    for key in parsed {
        // println!("{}", key);
        let _ = get_html_classes(key);
    }

}

fn get_html_file_list(path:&str) -> Vec<String>{
    let mut html_files:Vec<String> = Vec::new();

    let files = get_files::list(path);

    let parsed : HashMap<String, Vec<String>> = serde_json::from_str(&files).expect("Error!");

    for key in parsed.get("html").unwrap_or(&Vec::new()){
        html_files.push(key.to_string());
        println!("{}",key);
    }

    html_files
}

fn get_html_classes(file:String) -> io::Result<()> {

    // println!("{}",file.replace("./","/").to_string());
    let filed = File::open(file.replace("./","").to_string()).expect("Failed to open config file");
    let mut reader = BufReader::new(filed);

    let mut contents = String::new();
    
    reader.read_to_string(&mut contents);

    println!("File contents:\n{}", contents);

    Ok(())
}