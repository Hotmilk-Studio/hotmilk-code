use std::process::Command;
use anyhow::{Result, anyhow};
use std::io::{self, Write};

pub struct ToolExecutor;

impl ToolExecutor {
    /// Executes a shell command after user confirmation.
    pub fn execute_shell(command: &str) -> Result<String> {
        println!("\n⚠️  [SECURITY] Execution Request: `{}`", command);
        print!("Allow? (y/N): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == "y" {
            let output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()?;

            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            if output.status.success() {
                Ok(stdout)
            } else {
                Err(anyhow!("Command failed with status {}. Error: {}", output.status, stderr))
            }
        } else {
            Err(anyhow!("Execution denied by user"))
        }
    }

    /// Writes content to a file after user confirmation.
    pub fn write_file(path: &str, content: &str) -> Result<()> {
        println!("\n⚠️  [SECURITY] Write Request: `{}`", path);
        print!("Allow? (y/N): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == "y" {
            std::fs::write(path, content)?;
            Ok(())
        } else {
            Err(anyhow!("Write denied by user"))
        }
    }

    /// Search for a pattern in the current directory using ripgrep (rg).
    pub fn search_codebase(pattern: &str) -> Result<String> {
        let output = Command::new("rg")
            .arg("-n")
            .arg(pattern)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        if stdout.is_empty() {
            Ok("No matches found.".to_string())
        } else {
            Ok(stdout)
        }
    }
}
