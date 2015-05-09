#![feature(path_ext)]
#![feature(exit_status)]

use std::process::{Command};
use std::path::Path;
use std::env;
use std::fs::PathExt;

extern crate term;
use term::color;

macro_rules! color_println {
    ($color:expr, $fmt:expr, $($args:tt)*) => ({
        let mut t = term::stdout().unwrap();
        t.fg($color).unwrap();
        println!($fmt, $($args)*); // Using writeln!(t, ...) is correct, but it's causing an ICE.
        t.reset().unwrap();
    })
}

fn main() {
    let var_name = "CMAKE_EXE_PATH";
    let search_paths = vec![
        "C:\\Program Files\\CMake\\bin\\cmake.exe",
        "C:\\Program Files (x86)\\CMake\\bin\\cmake.exe"
    ];
    let mut status = -1;
    match env::var(var_name) {
        Ok(path_str) => {
            let path = Path::new(&path_str);
            if path.exists() {
                status = execute_cmake(path);
            }
            else {
                color_println!(color::RED, "CMake not found in env[{}]: {}", &var_name, &path_str);
            }
        }
        Err(_) => {
            let mut found = false;
            for path_str in &search_paths {
                let path = Path::new(&path_str);
                if path.exists() {
                    status = execute_cmake(path);
                    found = true;
                    break;
                }
            }
            if !found {
                color_println!(color::RED, "CMake not found in: {:?}", &search_paths);
            }
        }
    };

    env::set_exit_status(status);
}

fn execute_cmake(path: &Path) -> i32 {
    let var_name = "CMAKE_GENERATOR_NAME";
    let gen_name = match env::var(var_name) {
        Ok(path_str) => path_str.to_string(),
        Err(_) => "MSYS Makefiles".to_string()
    };
    let mut cmd = Command::new(&path.to_str().unwrap().to_string());
    let args = env::args();
    if args.len() > 1 {
        cmd.arg("-G");
        cmd.arg(gen_name);
    }
    for arg in args.skip(1) {
        cmd.arg(arg);
    }
    color_println!(color::YELLOW, "execute: {:?}", &cmd);
    return cmd.status().unwrap().code().unwrap();
}
