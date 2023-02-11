use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() {
    let output = Command::new("cmd")
        .args(&["/c", "where", "cmd"])
        .output()
        .expect("failed to execute process");

    let commands = String::from_utf8_lossy(&output.stdout);
    let commands: Vec<&str> = commands.split('\n').collect();

    let mut available_cmds = Vec::new();
    for command in commands.iter() {
        if command.starts_with("C:\\Windows\\System32") {
            let split = command.split("\\");
            let name = split.last().unwrap();
            available_cmds.push(name.to_string());
        }
    }

    let file = match File::create("available-cmds") {
        Err(why) => panic!("couldn't create available-cmds: {}", why),
        Ok(file) => file,
    };

    for cmd in available_cmds {
        match file.write_all(cmd.as_bytes()) {
            Err(why) => panic!("couldn't write to available-cmds: {}", why),
            Ok(_) => {
                match file.write_all(b"\n") {
                    Err(why) => panic!("couldn't write to available-cmds: {}", why),
                    Ok(_) => {}
                }
            }
        }
    }
}