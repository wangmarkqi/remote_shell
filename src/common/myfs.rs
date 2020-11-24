use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::path::PathBuf;

// std::fs::create_dir_all()
// std::path::Path::exists()
// std::path::Path::new().extension()
// std::fs::create_dir_all()
// std::path::Path::exists()
// std::path::Path::new().extension()
pub fn read_file_as_txt(file: &str) -> anyhow::Result<String> {
    let contents = read_to_string(file)?;
    Ok(contents)
}

pub fn write_file_as_txt(path: &str, content: String) -> anyhow::Result<String> {
    let mut output: File = File::create(path)?;
    write!(output, "{}", content)?;
    Ok("ok".to_string())
}

pub fn read_file_as_hex(inputfile: &str) -> anyhow::Result<String> {
    let mut _inputfile = File::open(inputfile)?;
    let mut v: Vec<u8> = Vec::new();
    _inputfile.read_to_end(&mut v)?;
    let str = hex::encode(v);
    Ok(str)
}

pub fn write_file_as_hex(path_str: &str, content: &str) -> anyhow::Result<String> {
    let binary = hex::decode(content).expect("hex decode fail");
    let p = std::path::Path::new(path_str);
    std::fs::write(p, binary)?;
    Ok(format!("save {} success", path_str))
}

pub fn get_dot_env(name: &str) -> String {
    dotenv::dotenv().ok();
    if let Ok(v) = std::env::var(name) {
        return v;
    }
    panic!("!!!!!!!!!!no env var: {}", name);
}


pub fn parse_input(input: &str) -> anyhow::Result<(String, Vec<String>)> {
    let mut parts = input.trim().split_whitespace();
    let command = parts.next().ok_or(anyhow!("none command provided"))?;
    let args: Vec<String> = parts.map(|e| e.to_string()).collect();
    Ok((command.to_string(), args))
}


pub fn create_file_dir(file_path: &str) -> bool {
    let fp = std::path::Path::new(file_path);
    if let Some(p) = fp.parent() {
        if let Ok(_) = std::fs::create_dir_all(p) {
            return true;
        }
    }
    false
}
