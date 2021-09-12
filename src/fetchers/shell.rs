use std::process::Command;

pub fn get_shell() -> String {
    let shell_executable =
        std::env::var("SHELL").map_or_else(|_| "Could not detect".to_string(), |v| v);

    // run with --version and return the output
    let output = String::from_utf8(
        Command::new(shell_executable)
            .arg("--version")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    output.split("\n").collect::<Vec<&str>>()[0].to_string()
}
