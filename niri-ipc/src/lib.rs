use std::process::Command;

pub trait CommandExecutor {
    fn execute(&self, program: &str, args: &[&str]) -> Result<String, String>;
}

pub struct SystemCommandExecutor;

impl CommandExecutor for SystemCommandExecutor {
    fn execute(&self, program: &str, args: &[&str]) -> Result<String, String> {
        let mut cmd = Command::new("niri");

        cmd.args(args);

        let output = cmd
            .output()
            .map_err(|e| format!("Failed to execute: {}: {}", program, e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Command {} failed : {}", program, stderr));
        }

        String::from_utf8(output.stdout)
            .map_err(|_| "Failed to converter command. invalid UTF-8".to_string())
    }
}

pub fn execute_niri_command_with_executor<E: CommandExecutor>(
    executor: &E,
    args: &[&str],
) -> Result<String, String> {
    executor.execute("niri", args)
}

pub fn niri_execute_command(args: &[&str]) -> Result<String, String> {
    execute_niri_command_with_executor(&SystemCommandExecutor, args)
}
pub fn execute_niri_help() -> Result<String, String> {
    niri_execute_command(&["msg", "--help"])
}

pub fn get_outputs() -> Result<String, String> {
    niri_execute_command(&["msg", "outputs"])
}

pub fn get_workspaces() -> Result<String, String> {
    niri_execute_command(&["msg", "workspaces"])
}

pub fn get_version() -> Result<String, String> {
    niri_execute_command(&["msg", "version"])
}
