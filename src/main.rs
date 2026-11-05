mod core;
mod providers;
mod tools;
mod ui;

use clap::Parser;
use crate::providers::ollama::OllamaProvider;
use crate::providers::traits::LLMProvider;
use crate::core::agent::Agent;
use crate::ui::banner::BANNER;

#[derive(Parser)]
#[command(name = "hotmilk-code", about = "Local AI Coding Assistant")]
struct Args {
    prompt: Option<String>,
    #[arg(short, long, default_value = "http://localhost:11434")]
    endpoint: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("{}", BANNER);

    let args = Args::parse();

    let provider = Box::new(OllamaProvider::new(args.endpoint));
    let agent = Agent::new(provider);

    if let Some(prompt) = args.prompt {
        let result = agent.run(&prompt).await?;
        println!("\n✨ Result:\n{}", result);
    } else {
        println!("Enter a prompt to start (or 'exit' to quit):");
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let input = input.trim();
            if input == "exit" { break; }

            let result = agent.run(input).await?;
            println!("\n✨ Result:\n{}", result);
        }
    }

    Ok(())
}
