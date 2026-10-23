//! Create a Retell LLM response engine and a voice agent.
//!
//! Agents require a response engine (`retell-llm`, conversation flow, or custom
//! LLM websocket) plus a `voice_id`. This example creates a simple Retell LLM,
//! then attaches it to a new agent.
//!
//! ```sh
//! export RETELL_API_KEY=key_...
//! # Optional:
//! # export RETELL_VOICE_ID=retell-Cimo
//! # export AGENT_NAME="My agent"
//! # export AGENT_PROMPT="You are a helpful phone assistant."
//! # export AGENT_BEGIN_MESSAGE="Hi, how can I help you today?"
//!
//! cargo run --example create_agent
//! ```

use retell_sdk::types::{CreateAgentParams, CreateRetellLlmParams, ResponseEngine};
use retell_sdk::RetellClient;
use serde_json::json;

#[tokio::main]
async fn main() -> retell_sdk::Result<()> {
    let api_key = std::env::var("RETELL_API_KEY").expect("set RETELL_API_KEY");
    let voice_id = env_or("RETELL_VOICE_ID", "retell-Cimo");
    let agent_name = env_or("AGENT_NAME", "SDK demo agent");
    let prompt = env_or(
        "AGENT_PROMPT",
        "You are a friendly, concise phone assistant. Keep answers to 1–2 sentences.",
    );
    let begin_message = env_or("AGENT_BEGIN_MESSAGE", "Hi, how can I help you today?");

    let client = RetellClient::new(api_key);

    eprintln!("Creating Retell LLM…");
    let llm = client
        .llm()
        .create(
            &CreateRetellLlmParams::new()
                .general_prompt(&prompt)
                .begin_message(&begin_message)
                .general_tools(vec![json!({
                    "type": "end_call",
                    "name": "end_call",
                    "description": "End the call when the conversation is finished."
                })]),
        )
        .await?;
    eprintln!("  llm_id: {}", llm.llm_id);

    eprintln!("Creating agent…");
    let agent = client
        .agent()
        .create(
            &CreateAgentParams::new(ResponseEngine::retell_llm(&llm.llm_id), &voice_id)
                .agent_name(&agent_name)
                .language("en-US"),
        )
        .await?;

    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "agent_id": agent.agent_id,
            "version": agent.version,
            "agent_name": agent.agent_name,
            "voice_id": agent.voice_id,
            "llm_id": llm.llm_id,
            "language": agent.language,
        }))
        .unwrap()
    );

    eprintln!(
        "\nExport for other examples:\n  export RETELL_AGENT_ID={}",
        agent.agent_id
    );

    Ok(())
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}
