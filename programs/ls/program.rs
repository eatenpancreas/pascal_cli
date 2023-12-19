use std::process::Command;

pub fn run() {
    let out = Command::new("ls")
        .output()
        .expect("Failed to execute command");
    
    println!("{}", String::from_utf8_lossy(&out.stdout));
}