//! Create a web call access token for browser testing.
//!
//! Free to try — no phone number required. Use the returned `access_token` with
//! Retell's web client / WebRTC stack.
//!
//! ```sh
//! export RETELL_API_KEY=key_...
//! export RETELL_AGENT_ID=agent_...
//! cargo run --example create_web_call
//! ```

use retell_sdk::types::CreateWebCallParams;
use retell_sdk::RetellClient;

#[tokio::main]
async fn main() -> retell_sdk::Result<()> {
    let api_key = std::env::var("RETELL_API_KEY").expect("set RETELL_API_KEY");
    let agent_id = std::env::var("RETELL_AGENT_ID").expect("set RETELL_AGENT_ID");

    let client = RetellClient::new(api_key);
    let call = client
        .call()
        .create_web_call(&CreateWebCallParams::new(agent_id))
        .await?;

    println!("call_id:      {}", call.call_id);
    println!("transport:   {}", call.transport);
    println!("expires_at:  {}", call.expires_at);
    println!("access_token (truncate): {}…", &call.access_token[..call.access_token.len().min(32)]);
    println!("ice_servers: {}", call.ice_servers.len());
    Ok(())
}
