use std::io::{BufRead, BufReader};
use std::process::Command;

// Run a command with returning code and output
pub fn run_cmd_with_output(cmd: &str, args: &[&str]) -> Result<i32, String> {
    let mut cmd = Command::new(cmd);
    cmd.args(args);

    let mut child = cmd
        .spawn()
        .map_err(|e| format!("Failed to start command: {}", e))?;

    let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
    let reader = BufReader::new(stdout);

    for line_res in reader.lines() {
        let line = line_res.map_err(|e| format!("Failed to read line: {}", e))?;
        println!("{}", line);
    }

    let status = child
        .wait()
        .map_err(|e| format!("Failed to wait for command: {}", e))?;

    let code = status.code().unwrap_or(-1);
    Ok(code)
}

// Run a command with returning code
pub fn run_cmd(cmd: &str, args: &[&str]) -> Result<i32, String> {
    let mut cmd = Command::new(cmd);
    cmd.args(args);

    let mut child = cmd
        .spawn()
        .map_err(|e| format!("Failed to start command: {}", e))?;

    let status = child
        .wait()
        .map_err(|e| format!("Failed to wait for command: {}", e))?;

    let code = status.code().unwrap_or(-1);
    Ok(code)
}
