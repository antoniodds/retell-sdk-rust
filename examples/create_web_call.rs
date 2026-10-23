//! Create a web call and print connection details.
//!
//! Free to try — no phone number required. Join within ~30 seconds using the
//! returned `access_token` with the [Retell Web SDK](https://docs.retellai.com/deploy/web-call)
//! (`RetellWebClient.startCall({ accessToken, callId, transport, iceServers })`).
//!
//! ```sh
//! export RETELL_API_KEY=key_...
//! export RETELL_AGENT_ID=agent_...
//! cargo run --example create_web_call
//! ```

use retell_sdk::types::CreateWebCallParams;
use retell_sdk::RetellClient;
use serde_json::json;

#[tokio::main]
async fn main() -> retell_sdk::Result<()> {
    let api_key = std::env::var("RETELL_API_KEY").expect("set RETELL_API_KEY");
    let agent_id = std::env::var("RETELL_AGENT_ID").expect("set RETELL_AGENT_ID");

    let client = RetellClient::new(api_key);
    let call = client
        .call()
        .create_web_call(&CreateWebCallParams::new(agent_id))
        .await?;

    let ice_servers: Vec<_> = call
        .ice_servers
        .iter()
        .map(|s| {
            let urls = match &s.urls {
                retell_sdk::types::IceUrls::One(u) => json!(u),
                retell_sdk::types::IceUrls::Many(u) => json!(u),
            };
            let mut obj = json!({ "urls": urls });
            if let Some(u) = &s.username {
                obj["username"] = json!(u);
            }
            if let Some(c) = &s.credential {
                obj["credential"] = json!(c);
            }
            obj
        })
        .collect();

    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "call_id": call.call_id,
            "access_token": call.access_token,
            "transport": call.transport,
            "expires_at": call.expires_at,
            "ice_servers": ice_servers,
        }))
        .unwrap()
    );

    Ok(())
}
