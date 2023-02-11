use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn main() {
    let cmd_file_path = "C:\\Users\\Qu1nSp0it\\Desktop\\rust\\Windows-Commands\\cmd.txt";
    let cmd_file = match File::open(cmd_file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return;
        },
    };
    let cmd_reader = BufReader::new(cmd_file);
    for line in cmd_reader.lines() {
        let cmd = line.unwrap();
        let output = Command::new("cmd")
            .arg("/C")
            .arg(&cmd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();
        match output {
            Ok(o) => {
                if !o.status.success() {
                    let stderr = String::from_utf8_lossy(&o.stderr);
                    if stderr.contains("is not recognized as an internal or external command") {
                        println!("Command not recognized: {:?}", cmd);
                    } else {
                        println!("{:?}", stderr);
                    }
                } else {
                    let stdout = String::from_utf8_lossy(&o.stdout);
                    let mut output_file = File::create("output.txt").unwrap();
                    output_file.write_all(stdout.as_bytes()).unwrap();
                }
            },
            Err(e) => println!("Failed to run command: {:?}", e),
        }
    }
}