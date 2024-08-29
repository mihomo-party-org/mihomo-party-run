use std::env;
use std::fs;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <program_path>", args[0]);
        std::process::exit(1);
    }
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get executable directory");
    let param_path = exe_dir.join("param.txt");
    let content = fs::read_to_string(param_path).unwrap_or(String::new());
    let params: Vec<&str> = content.trim().split_whitespace().collect();
    let mut cmd = Command::new(&args[1]);
    cmd.args(params);
    cmd.spawn().expect("Failed to start program");
    Ok(())
}
