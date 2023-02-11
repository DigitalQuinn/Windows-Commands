use std::process::Command;

fn main() {
    let output = Command::new("cmd")
        .args(&["/c", "where", "cmd"])
        .output()
        .expect("failed to execute process");

    let commands = String::from_utf8_lossy(&output.stdout);
    let commands: Vec<&str> = commands.split('\n').collect();

    for command in commands.iter() {
        if command.starts_with("C:\\Windows\\System32") {
            let split = command.split("\\");
            let name = split.last().unwrap();
            println!("{}", name);
        }
    }
}