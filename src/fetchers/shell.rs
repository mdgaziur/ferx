use std::process::Command;

pub fn get_shell() -> String {
    let shell_executable =
        std::env::var("SHELL").map_or_else(|_| "Could not detect".to_string(), |v| v);

    // run with --version and return the output
    let mut output = String::from_utf8(
        Command::new(shell_executable)
            .arg("--version")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    output.pop();
    output
}
