//! List agents in your workspace.
//!
//! ```sh
//! export RETELL_API_KEY=key_...
//! cargo run --example list_agents
//! ```

use retell_sdk::RetellClient;

#[tokio::main]
async fn main() -> retell_sdk::Result<()> {
    let api_key = std::env::var("RETELL_API_KEY").expect("set RETELL_API_KEY");
    let client = RetellClient::new(api_key);

    let page = client.agent().list_all().await?;
    println!(
        "{} agents (has_more={})",
        page.items.len(),
        page.has_more
    );
    for agent in page.items {
        println!(
            "  {}  [{}]  {}",
            agent.agent_id, agent.channel, agent.agent_name
        );
    }
    Ok(())
}
