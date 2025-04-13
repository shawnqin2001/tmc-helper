use std::error::Error;
use std::process::Command;
// Run a command with returning code and output

// Run a command with returning code
pub fn run_cmd(cmd: &str, args: &[&str]) -> Result<String, Box<dyn Error>> {
    let mut cmd = Command::new(cmd);
    cmd.args(args);
    let output = cmd.output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
