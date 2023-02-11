use std::process::Command;
use std::io::{self, Read};

fn generate_example_command(command: &str) -> String {
    // Parse the command string to determine the command name and its parameters
    let parts: Vec<&str> = command.split_whitespace().collect();
    let command_name = parts[0];
    let parameters = parts[1..].join(" ");

    // Generate an example command string based on the command name and parameters
    let example_command = match command_name {
        "dir" => format!("dir {}", parameters),
        "echo" => format!("echo {}", parameters),
        // Add cases for other commands as needed
        _ => format!("{} {}", command_name, parameters),
    };

    example_command
}

fn main() {
    // Read the commands from the file
    let mut file = match std::fs::File::open("C:\\Users\\Qu1nSp0it\\Desktop\\rust\\Windows-Commands\\cmd.txt") {
        Ok(file) => file,
        Err(error) => {
            println!("Error opening file: {}", error);
            return;
        },
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(error) => {
            println!("Error reading file: {}", error);
            return;
        },
    };

    // Iterate over each line in the file and run the command
    for line in contents.lines() {
        let example_command = generate_example_command(line);
        let output = Command::new("cmd")
            .args(&["/C", &example_command])
            .output()
            .expect("Failed to run command");

        if !output.status.success() {
            println!("Error running command: {}", String::from_utf8_lossy(&output.stderr));
        } else {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
}