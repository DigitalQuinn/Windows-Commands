use std::process::Command;
use std::str;

fn main() {
    let output = Command::new("reg")
        .args(&["query", "HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\", "/s"])
        .output()
        .expect("Failed to execute 'reg' command");
    let registry = str::from_utf8(&output.stdout).unwrap();

    for line in registry.lines() {
        if line.contains("    (Default)    REG_SZ    ") {
            let key = line.split("    ").nth(0).unwrap();
            let value = line.split("    ").nth(3).unwrap();
            if value.contains(" ") {
                println!("{}: {}", key, value);
            }
        }
    }
}