use std::process::Command;

fn main() {
    let ip_address = "1.1.1.1";
    let output = Command::new("ping")
        .arg(ip_address)
        .output()
        .expect("Failed to run command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);
}