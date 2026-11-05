use crate::providers::traits::{LLMProvider, LLMRequest, LLMResponse};
use crate::tools::fs_ops::ToolExecutor;
use crate::core::context::ContextWindow;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum AgentAction {
    Shell { command: String },
    WriteFile { path: String, content: String },
    ReadFile { path: String },
    Search { pattern: String },
    FinalAnswer { answer: String },
}

pub struct Agent {
    provider: Box<dyn LLMProvider>,
    context: ContextWindow,
}

impl Agent {
    pub fn new(provider: Box<dyn LLMProvider>) -> Self {
        let mut context = ContextWindow::new(16384); // Expanded context for industrial use
        context.add_message(
            crate::core::context::Role::System,
            "You are Hotmilk Code, an elite AI software engineer.
You MUST respond using a JSON object for actions.
Available tools:
- {\"action\": \"shell\", \"command\": \"...\"} : Execute shell commands (ls, git, cargo, etc.)
- {\"action\": \"write_file\", \"path\": \"...\", \"content\": \"...\"} : Write or overwrite a file.
- {\"action\": \"read_file\", \"path\": \"...\"} : Read a file's content.
- {\"action\": \"search\", \"pattern\": \"...\"} : Search for a string in the codebase using ripgrep.
- {\"action\": \"final_answer\", \"answer\": \"...\"} : Provide the final result to the user.

Guidelines:
1. ALWAYS search or read files before making changes.
2. Use shell commands to verify your changes (e.g., run tests).
3. Be concise and precise.
4. If a tool fails, analyze the error and try a different approach.
".to_string()
        );

        Self {
            provider,
            context,
        }
    }

    pub async fn run(&mut self, user_input: &str) -> Result<String> {
        self.context.add_message(crate::core::context::Role::User, user_input.to_string());

        loop {
            let request = LLMRequest {
                prompt: self.context.build_prompt(),
                system_prompt: "".to_string(),
                temperature: 0.0,
                max_tokens: 4096,
            };

            let response = self.provider.generate(request).await?;
            let content = response.content;

            println!("\n🤖 [AGENT]:\n{}", content);
            self.context.add_message(crate::core::context::Role::Assistant, content.clone());

            if let Some(action) = self.parse_action(&content) {
                match action {
                    AgentAction::Shell { command } => {
                        match ToolExecutor::execute_shell(&command) {
                            Ok(obs) => self.context.add_message(crate::core::context::Role::Tool, format!("Shell output: {}", obs)),
                            Err(e) => self.context.add_message(crate::core::context::Role::Tool, format!("Shell error: {}", e)),
                        }
                    },
                    AgentAction::WriteFile { path, content } => {
                        match ToolExecutor::write_file(&path, &content) {
                            Ok(_) => self.context.add_message(crate::core::context::Role::Tool, format!("Successfully wrote to {}", path)),
                            Err(e) => self.context.add_message(crate::core::context::Role::Tool, format!("Write error: {}", e)),
                        }
                    },
                    AgentAction::ReadFile { path } => {
                        let content = std::fs::read_to_string(&path).unwrap_or_else(|_| "Error reading file".to_string());
                        self.context.add_message(crate::core::context::Role::Tool, format!("File content of {}: {}", path, content));
                    },
                    AgentAction::Search { pattern } => {
                        match ToolExecutor::search_codebase(&pattern) {
                            Ok(obs) => self.context.add_message(crate::core::context::Role::Tool, format!("Search results: {}", obs)),
                            Err(e) => self.context.add_message(crate::core::context::Role::Tool, format!("Search error: {}", e)),
                        }
                    },
                    AgentAction::FinalAnswer { answer } => {
                        return Ok(answer);
                    }
                }
            } else {
                return Err(anyhow::anyhow!("Agent failed to produce a valid JSON action. Check if the model is too small or requires better prompting."));
            }
        }
    }

    fn parse_action(&self, content: &str) -> Option<AgentAction> {
        if let Some(start) = content.find('{') {
            if let Some(end) = content.rfind('}') {
                let json_str = &content[start..=end];
                return serde_json::from_str::<AgentAction>(json_str).ok();
            }
        }
        None
    }
}
