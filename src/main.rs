use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() {
    let cmd_output = Command::new("cmd")
        .args(&["/c", "where", "cmd"])
        .output()
        .expect("failed to execute process");

    let cmd_commands = String::from_utf8_lossy(&cmd_output.stdout);
    let cmd_commands: Vec<&str> = cmd_commands.split('\n').collect();

    let ps_output = Command::new("powershell")
        .args(&["Get-Command"])
        .output()
        .expect("failed to execute process");

    let ps_commands = String::from_utf8_lossy(&ps_output.stdout);
    let ps_commands: Vec<&str> = ps_commands.split('\n').collect();

    let mut cmd_available_cmds = Vec::new();
    for command in cmd_commands.iter() {
        if command.starts_with("C:\\Windows\\System32") {
            let mut split = command.split("\\");
            let name = split.last().unwrap();
            cmd_available_cmds.push(name.to_string());
        }
    }

    let mut ps_available_cmds = Vec::new();
    for command in ps_commands.iter() {
        let mut split = command.split(" ");
        let name = split.next().unwrap();
        ps_available_cmds.push(name.to_string());
    }

    let mut cmd_file = match File::create("cmds") {
        Err(why) => panic!("couldn't create cmds: {}", why),
        Ok(file) => file,
    };

    for cmd in cmd_available_cmds {
        match cmd_file.write_all(cmd.as_bytes()) {
            Err(why) => panic!("couldn't write to cmds: {}", why),
            Ok(_) => {
                match cmd_file.write_all(b"\n") {
                    Err(why) => panic!("couldn't write to cmds: {}", why),
                    Ok(_) => {}
                }
            }
        }
    }

    let mut ps_file = match File::create("ps-cmds") {
        Err(why) => panic!("couldn't create ps-cmds: {}", why),
        Ok(file) => file,
    };

    for cmd in ps_available_cmds {
        match ps_file.write_all(cmd.as_bytes()) {
            Err(why) => panic!("couldn't write to ps-cmds: {}", why),
            Ok(_) => {
                match ps_file.write_all(b"\n") {
                    Err(why) => panic!("couldn't write to ps-cmds: {}", why),
                    Ok(_) => {}
                }
            }
        }
    }
}