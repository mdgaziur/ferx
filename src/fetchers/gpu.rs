use std::process::Command;

pub fn get_gpu() -> String {
    let mut output = String::from_utf8(
        Command::new("/bin/sh")
            .arg("-c")
            .arg("PATH=\"/sbin:$PATH\" lspci -mm | awk -F '\\\"|\\\" \\\"' '/3D|VGA/ {print $3 \" \" $4}'") // https://github.com/dylanaraps/neofetch/pull/468
            .output()
            .unwrap()
            .stdout
    )
        .unwrap();
    output.pop();
    output
}