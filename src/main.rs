
mod services;
mod operators;
mod struction;


fn main() {
    let config = services::file_manager::read_config(("fs.config.json").to_string());

    let _ = operators::get_files::categorize();

    //  let file = operators::get_files::list(&(config.directory));

     

     let _ = operators::html_analyzor::gather_html_content(&(config.directory));

}
