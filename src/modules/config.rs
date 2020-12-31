use std::env::var;
use std::path::Path;
use std::fs;
// use std::io::Result;
use once_cell::sync::Lazy;

use crate::modules::helpers;

pub struct Config {
    pub defaults: bool,
    pub systemd: bool
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    read()
});

fn get_dir() -> String {
    let user = var("SUDO_USER").unwrap();
    let args: &[&str] = &["passwd", &user];
    let command = ("getent", args);
    let output = helpers::run::with_shell(command);
    let output_as_string = helpers::trim::user_home(output);
    [output_as_string,String::from("/.config")].join("")
}

fn read() -> Config {
    let full_path = [get_dir(), String::from("/wg-switch/config")].join("");
    let mut content = String::from("");
    if Path::new(&full_path).exists() {
        content = fs::read_to_string(&full_path)
            .expect("Failed to read config file");
    }
    helpers::trim::config(content)
}

// fn is_valid_file(path: String) -> Result<bool> {
//     // Exists
//     let exists = Path::new(&path).exists();
//     // Is file
//     let metadata = fs::metadata(&path)?;
//     let file_type = metadata.file_type();
//     let is_file = file_type.is_file() && !file_type.is_dir();
//     let result = exists && is_file;
//     return result;
// }

pub fn print() {
    println!("Using values from config file: {}", !CONFIG.defaults);
    println!("Enable/disable systemd service: {}", CONFIG.systemd);
}
