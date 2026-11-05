# 🥛 Hotmilk Code

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Organization](https://img.shields.io/badge/Org-Hotmilk%20Studio-orange.svg)]()

**Hotmilk Code** is a high-performance, local-first AI coding assistant CLI designed for privacy, speed, and total control.

Built for developers who want the power of agentic coding without sending their proprietary codebase to the cloud.

## ✨ Key Features

- 🏠 **100% Local & Private**: Works with Ollama and llama.cpp. No data leaves your machine.
- 🤖 **Agentic Workflow**: ReAct-based loop for autonomous file editing and shell execution.
- 🛠️ **Workspace Aware**: Local indexing and RAG for deep context of your project.
- 🎨 **Premium TUI**: A modern, amber-toned terminal interface for a warm coding experience.
- ⚡ **Optimized for Small Models**: Fine-tuned system prompts for Qwen, DeepSeek, and Llama.

## 🚀 Quick Start

### Prerequisites
- [Ollama](https://ollama.ai/) or [llama.cpp](https://github.com/ggerganov/llama.cpp)
- Rust (latest stable)

### Installation
```bash
git clone https://github.com/hotmilk-studio/hotmilk-code.git
cd hotmilk-code
cargo install --path .
```

### Usage
```bash
hotmilk-code "Refactor the authentication logic in src/auth.rs to use JWT"
```

## 🛠 Architecture

- **Core**: ReAct agent loop managing state and tool calls.
- **Providers**: Pluggable backend for different local LLM runners.
- **Tools**: Secure interface for File System and Shell operations.
- **UI**: Custom TUI components for streaming and diff viewing.

## 📄 License
Distributed under the Apache License 2.0. See `LICENSE` for more information.

Copyright (c) Hotmilk Studio
