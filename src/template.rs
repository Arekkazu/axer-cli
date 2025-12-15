use std::{fs, io};

use std::fs::{ReadDir};
use std::io::Read;
use std::path::Path;

pub fn create_directory_templates() {
    let path = "templates";
    let exist_dir = exist_dir(path).expect("The directory doesnt exist");

    if !exist_dir {
        fs::create_dir(path).expect("And error ocurred Creating the file");
    }
}

fn exist_dir(path: &str) -> io::Result<bool> {
    let exist = fs::exists(path)?;
    Ok(exist)
}

pub fn check_template() -> io::Result<Vec<String>> {
    let dir_template: &Path = Path::new("templates");
    let templates_folder: ReadDir = fs::read_dir(dir_template)?;
    let list_templates = templates_folder
        .filter_map(|template| {
            let dir_entry = template.ok()?;
            let dir_entry_path = dir_entry.path();
            if !dir_entry_path.is_dir() {
                return None;
            }
            let template_file = dir_entry_path.join("template.toml");
            if !template_file.exists() {
                return None;
            }
            dir_entry
                .path()
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
        })
        .collect::<Vec<String>>();

    Ok(list_templates)
}

pub fn getting_toml_template(template_choiced: &str) -> Result<(), io::Error>  {
    let file = format!("templates/{template_choiced}/template.toml");
    println!("{}", file);
    let path = Path::new(&file);
    let mut file = fs::File::open(path)?;
    let mut contet_toml = String::new();
    file.read_to_string(&mut contet_toml)?;
    print!("contet: {}", contet_toml);
    Ok(())
}
