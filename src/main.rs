use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn main() {
    let cmd_file_path = "C:\\Users\\Qu1nSpl0it\\Desktop\\rust\\Windows-Commands\\cmd.txt";
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
        let quoted_cmd = format!("\"{}\"", cmd);
        let output = Command::new("cmd")
            .arg("/C")
            .arg(&quoted_cmd)
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
                        let parts: Vec<&str> = cmd.split(" ").collect();
                        let mut example_cmd = String::new();
                        example_cmd.push_str(parts[0]);
                        example_cmd.push_str(" ");
                        for i in 1..parts.len() {
                            example_cmd.push_str("<");
                            example_cmd.push_str(parts[i]);
                            example_cmd.push_str(">");
                            example_cmd.push_str(" ");
                        }
                        println!("Error: {:?}\nExample command: {:?}", stderr, example_cmd);
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