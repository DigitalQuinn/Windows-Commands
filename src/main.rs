use std::fs;
use std::process::Command;

fn main() {
    let contents = fs::read_to_string("cmd.txt")
        .expect("Something went wrong reading the file");

    let commands = contents.lines();

    for cmd in commands {
        let output = Command::new("cmd")
            .args(&["/C", cmd])
            .output()
            .expect("failed to execute process");

        let result = String::from_utf8_lossy(&output.stderr);
        if !result.contains("is not recognized") {
            println!("{}", result);
        }
    }
}