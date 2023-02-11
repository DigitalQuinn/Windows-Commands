use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::env;

fn main() {
    let current_exe = env::current_exe().unwrap();
    let current_dir = current_exe.parent().unwrap();
    println!("Current working directory: {:?}", current_dir);

    let cmd_file_path = current_dir.join("cmd.txt");
    let cmd_file = File::open(cmd_file_path).unwrap();
    let cmd_reader = BufReader::new(cmd_file);
    for line in cmd_reader.lines() {
        let cmd = line.unwrap();
        let cmd_copy = cmd.clone();
        let current_dir_clone = current_dir.clone();
        let output = Command::new("sh")
            .arg("-c")
            .arg(&cmd_copy)
            .current_dir(&current_dir_clone)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output();
        match output {
            Ok(o) => {
                if !o.status.success() {
                    let stderr = String::from_utf8_lossy(&o.stderr);
                    if stderr.contains("command not found") {
                        println!("Command not recognized: {:?}", cmd);
                    } else {
                        println!("{:?}", stderr);
                    }
                }
            },
            Err(e) => println!("Failed to run command: {:?}", e),
        }
    }
}