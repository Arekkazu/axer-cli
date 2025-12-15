use std::io;
use std::io::Error;

use crate::template::check_template;
use colored::Colorize;
use inquire::{InquireError, Select};

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Inquire(InquireError),
    NoTemplates,
}

impl AppError {
    pub fn print(&self) {
        match self {
            AppError::NoTemplates => {
                eprintln!("{}", "There are no templates".red());
            }
            AppError::Io(e) => {
                eprintln!("IO error: {}", e);
            }
            AppError::Inquire(e) => {
                eprintln!("UI error: {}", e);
            }
        }
    }
}
impl From<io::Error> for AppError {
    fn from(value: Error) -> Self {
        AppError::Io(value)
    }
}

impl From<InquireError> for AppError {
    fn from(value: InquireError) -> Self {
        AppError::Inquire(value)
    }
}

pub fn tui() {
    match tui_inner() {
        Ok(_) => {}
        Err(e) => e.print(), // impresión aquí
    }
}
pub fn tui_inner() -> Result<String, AppError> {
    let templates = get_templates()?;
    println!("{:?}", templates);
    if templates.is_empty() {
        return Err(AppError::NoTemplates);
    }

    let template = show_select_templates(templates.clone())?;

    println!("{}, {}", "Selected: ".green(), template);

    //MANDANDO FUNCION A GENERATOR DICIENDO QUE VA ESCOGER ESE TEMPLATE

    Ok(template)
}
fn get_templates() -> io::Result<Vec<String>> {
    check_template()
}
fn show_select_templates(templates: Vec<String>) -> Result<String, InquireError> {
    let selection = Select::new("What project you want to do?: ", templates).prompt()?;
    Ok(selection)
}
