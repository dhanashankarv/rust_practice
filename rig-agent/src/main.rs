use rig::{
    agent::AgentBuilder, client::CompletionClient, completion::Chat, providers::openai,
};
use std::env::args;

fn read_api_key(filename: &str) -> String {
    std::fs::read_to_string(filename)
        .expect("Failed to read API key from file")
        .trim()
        .to_string()
}

#[tokio::main]
async fn main() {
    // Initialize the provider (e.g., OpenAI)
    let args = args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <api_key_file>", args[0]);
        return;
    }

    let api_key = read_api_key(args[1].as_str());
    let openai_client = openai::Client::new(api_key).expect("Failed to create OpenAI client");
    let gpt_4o = openai_client.completion_model("gpt-4o");

    let agent = AgentBuilder::new(gpt_4o)
        .preamble("You are a helpful assistant that should accurately and precisely answer questions and perform tasks.
                  Do not assume anything that is not explicitly stated in the question.
                  If you don't know the answer, say you don't know. Do not make up answers.
                  Always be concise and to the point.")
        .temperature(0.1)
        .build();

    // Example usage: ask the agent a question
    let mut history = vec![
        "What is the capital of France?".to_string(),
        "The capital of France is Paris.".to_string(),
    ];
    let response = agent.chat("What is the property tax rate in San Jose, California?", &history).await.unwrap();
    println!("Agent response: {}", response);
    history.push(response.clone());
}
