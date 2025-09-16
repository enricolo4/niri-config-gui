use niri_ipc::*;

use std::collections::HashMap;

// Mock para testes
pub struct MockCommandExecutor {
    responses: HashMap<(String, Vec<String>), Result<String, String>>,
}

impl MockCommandExecutor {
    pub fn new() -> Self {
        Self {
            responses: HashMap::new(),
        }
    }

    // Configura resposta para um comando específico
    pub fn expect_command(
        &mut self,
        program: &str,
        args: &[&str],
        response: Result<String, String>,
    ) {
        let key = (
            program.to_string(),
            args.iter().map(|s| s.to_string()).collect(),
        );
        self.responses.insert(key, response);
    }
}

impl CommandExecutor for MockCommandExecutor {
    fn execute(&self, program: &str, args: &[&str]) -> Result<String, String> {
        let key = (
            program.to_string(),
            args.iter().map(|s| s.to_string()).collect(),
        );

        self.responses
            .get(&key)
            .cloned()
            .unwrap_or_else(|| Err(format!("Comando não mockado: {} {:?}", program, args)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_niri_command_success() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_command("niri", &["msg", "--help"], Ok("Help text".to_string()));

        let result = execute_niri_command_with_executor(&mock, &["msg", "--help"]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Help text");
    }

    #[test]
    fn test_execute_niri_command_failure() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_command(
            "niri",
            &["msg", "invalid"],
            Err("Command failed".to_string()),
        );

        let result = execute_niri_command_with_executor(&mock, &["msg", "invalid"]);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Command failed");
    }
}
