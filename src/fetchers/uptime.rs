use std::process::Command;

pub fn get_uptime() -> String {
    let mut output =
        String::from_utf8(Command::new("uptime").arg("-p").output().unwrap().stdout).unwrap();
    output.pop();
    output
}
