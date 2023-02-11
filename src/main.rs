use std::process::Command;

fn main() {
    let output = Command::new("tpmtool.exe")
        .args(&["drive", "tracing", "stop"])
        .output()
        .expect("failed to execute process");

    let result = String::from_utf8_lossy(&output.stdout);
    println!("{}", result);
}