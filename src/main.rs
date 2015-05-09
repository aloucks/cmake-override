#![feature(path_ext)]
#![feature(exit_status)]

use std::process::{Command};
use std::path::PathBuf;
use std::env;
use std::fs::PathExt;

extern crate term;
use term::color;

macro_rules! color_println {
    ($color:expr, $fmt:expr, $($args:tt)*) => ({
        let mut t = term::stdout().unwrap();
        t.fg($color).unwrap();
        println!($fmt, $($args)*); // writeln!(t, ...) is causing an ICE.
        t.reset().unwrap();
    })
}

static CMAKE_EXE_PATH : &'static str = "CMAKE_EXE_PATH";
static CMAKE_GEN_NAME : &'static str = "CMAKE_GEN_NAME";

fn main() {
    let search_paths = vec![
        "C:\\Program Files\\CMake\\bin\\cmake.exe",
        "C:\\Program Files (x86)\\CMake\\bin\\cmake.exe"
    ];
    let mut paths : Vec<PathBuf> = Vec::new();
    for path_str in &search_paths {
        paths.push(PathBuf::from(&path_str));
    }
    if let Ok(path_str) = env::var(CMAKE_EXE_PATH) {
        paths.clear();
        paths.push(PathBuf::from(&path_str));
    }
    let mut path : Option<&PathBuf> = None;
    for p in &paths {
        if p.exists() {
            path = Some(p);
        }
    }
    let status = match path {
        Some(ref path) => cmake(path),
        None => {
            color_println!(color::RED, "CMake executable not found in: {:?}", &paths);
            -1
        }
    };
    env::set_exit_status(status);
}

fn cmake(path: &PathBuf) -> i32 {
    let name_str = match env::var(CMAKE_GEN_NAME) {
        Ok(name) => name,
        Err(_) => "MSYS Makefiles".to_string()
    };
    let mut cmd = Command::new(&path);
    let args = env::args();
    if args.len() > 1 {
        cmd.arg("-G");
        cmd.arg(name_str);
    }
    for arg in args.skip(1) {
        cmd.arg(arg);
    }
    color_println!(color::YELLOW, "Execute: {:?}", &cmd);
    return cmd.status().unwrap().code().unwrap();
}
