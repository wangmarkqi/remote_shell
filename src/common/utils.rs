use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::path::PathBuf;

use super::order::Order;
use std::net::SocketAddr;
use std::time::Duration;
// std::fs::create_dir_all()
// std::path::Path::exists()
// std::path::Path::new().extension()
// std::fs::create_dir_all()
// std::path::Path::exists()
// std::path::Path::new().extension()


pub fn read_file_as_u8(inputfile: &str) -> anyhow::Result<Vec<u8>> {
    let mut _inputfile = File::open(inputfile)?;
    let mut v: Vec<u8> = Vec::new();
    _inputfile.read_to_end(&mut v)?;
    Ok(v)
}

pub fn write_file_as_u8(path_str: &str, binary: &Vec<u8>) -> anyhow::Result<String> {
    let p = std::path::Path::new(path_str);
    std::fs::write(p, binary)?;
    Ok(format!("保存成功,地址：{}", path_str))
}


pub fn parse_input(input: &str) -> anyhow::Result<(String, Vec<String>)> {
    let mut parts = input.trim().split_whitespace();
    let command = parts.next().ok_or(anyhow!("none command provided"))?;
    let args: Vec<String> = parts.map(|e| e.to_string()).collect();
    Ok((command.to_string(), args))
}


pub fn create_file_dir(file_path: &str) -> bool {
    let file_path = std::path::Path::new(file_path);
    let dir = file_path.parent().unwrap();
    if !dir.exists() {
        std::fs::create_dir_all(dir).expect("create dir fail");
        return true;
    };
    false
}
