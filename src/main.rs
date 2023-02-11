use std::process::Command;

fn main() {
    let output = Command::new("cmd")
        .args(&["/c", "where", "cmd"])
        .output()
        .expect("failed to execute process");

    let commands = String::from_utf8_lossy(&output.stdout);
    for command in commands.lines() {
        println!("{}", command);
    }
}