use std::process::Command;

fn main() {
    let output = Command::new("cmd")
        .args(&["/C", "tpmtool.exe", "drivertracing", "stop"])
        .output()
        .expect("failed to execute process");

    let result = String::from_utf8_lossy(&output.stdout);
    println!("{}", result);
}