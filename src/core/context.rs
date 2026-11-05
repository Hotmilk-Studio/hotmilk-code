use std::collections::VecDeque;
use anyhow::Result;

pub struct ContextWindow {
    max_tokens: usize,
    history: VecDeque<ChatMessage>,
}

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
    pub tokens: usize,
}

#[derive(Clone, Debug)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

impl ContextWindow {
    pub fn new(max_tokens: usize) -> Self {
        Self {
            max_tokens,
            history: VecDeque::new(),
        }
    }

    pub fn add_message(&mut self, role: Role, content: String) {
        let tokens = content.len() / 4; // Approximation simplifiée
        self.history.push_back(ChatMessage {
            role,
            content,
            tokens,
        });
        self.truncate();
    }

    fn truncate(&mut self) {
        let mut current_tokens: usize = self.history.iter().map(|m| m.tokens).sum();
        while current_tokens > self.max_tokens && self.history.len() > 1 {
            if let Some(msg) = self.history.pop_front() {
                current_tokens -= msg.tokens;
            }
        }
    }

    pub fn build_prompt(&self) -> String {
        let mut prompt = String::new();
        for msg in &self.history {
            match msg.role {
                Role::System => prompt.push_str(&format!("<|system|>\n{}\n", msg.content)),
                Role::User => prompt.push_str(&format!("<|user|>\n{}\n", msg.content)),
                Role::Assistant => prompt.push_str(&format!("<|assistant|>\n{}\n", msg.content)),
                Role::Tool => prompt.push_str(&format!("<|tool|>\n{}\n", msg.content)),
            }
        }
        prompt
    }
}
