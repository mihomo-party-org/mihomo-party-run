#![windows_subsystem = "windows"]

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::os::windows::ffi::OsStrExt;
use std::process::Command;
use winapi::um::winuser::MessageBoxW;

fn main() {
    if let Err(e) = run() {
        show_error(&format!("Error: {}", e));
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Invalid arguments".into());
    }
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get executable directory");
    let param_path = exe_dir.join("param.txt");
    let content = fs::read_to_string(param_path).unwrap_or(String::new());
    let params = content.trim();
    let mut cmd = Command::new(&args[1]);
    cmd.arg(params);
    if let Err(e) = cmd.spawn() {
        return Err(format!("Failed to start program\n{}\n{}\n请尝试以管理员权限启动软件", e, &args[1]).into());
    }
    Ok(())
}

fn show_error(message: &str) {
    let wide: Vec<u16> = OsStr::new(message).encode_wide().chain(Some(0)).collect();
    let title: Vec<u16> = OsStr::new("Mihomo Party Runner")
        .encode_wide()
        .chain(Some(0))
        .collect();
    unsafe {
        MessageBoxW(std::ptr::null_mut(), wide.as_ptr(), title.as_ptr(), 0);
    }
}
