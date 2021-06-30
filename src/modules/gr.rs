use std::{
    env::{current_dir, set_current_dir},
    path::PathBuf,
    str::FromStr,
};

pub fn gr() -> Result<(), String> {
    let fs_root = PathBuf::from_str("/").unwrap();
    let git_dir = PathBuf::from_str(".git").unwrap();

    while current_dir().unwrap() != fs_root && !git_dir.is_dir() {
        match set_current_dir("..") {
            Err(_) => return Err("Failed changing directory".to_string()),
            _ => {}
        }
    }

    if PathBuf::from_str(".git").unwrap().is_dir() {
        println!("{}", current_dir().unwrap().to_str().unwrap());
        Ok(())
    } else {
        Err("Not under a git repository".to_string())
    }
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Jun 22 2021, 01:09 [CST]
