use std::fs;
use std::process::Command;

fn main() {
    let cmd_file = "C:\\Users\\Qu1nSpl0it\\Desktop\\rust\\Windows-Commands\\cmd.txt";
    let output_file = "output.txt";
    let cmds = fs::read_to_string(cmd_file)
        .expect("Failed to read file");

    for cmd in cmds.lines() {
        let output = Command::new("cmd")
            .args(&["/C", cmd])
            .output()
            .expect("Failed to run command");
        
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        let error_str = String::from_utf8_lossy(&output.stderr).to_string();

        if !error_str.is_empty() {
            println!("Error: {}", error_str);
        } else {
            fs::write(output_file, output_str)
                .expect("Failed to write file");
        }
    }
}