//! Web-call roleplay: AI guest books a restaurant table; you play the host.
//!
//! Creates a Retell LLM + voice agent (or reuses `RETELL_AGENT_ID`) whose job is
//! to call and reserve dinner for a party of 4. Then starts a web call and prints
//! the join payload (same shape as `create_web_call`).
//!
//! When you join in a browser with the Web SDK, **you** are the restaurant
//! reception — answer, check availability, and confirm or decline the booking.
//! The agent is the guest making the reservation.
//!
//! ```sh
//! export RETELL_API_KEY=key_...
//! # Optional: reuse an agent instead of creating one
//! # export RETELL_AGENT_ID=agent_...
//! # export RETELL_VOICE_ID=retell-Cimo
//! # export RESTAURANT_NAME="Bistro Nova"
//! # export PARTY_SIZE=4
//! # export RESERVATION_TIME="tonight at 7:30pm"
//! # export CUSTOMER_NAME="Alex Rivera"
//!
//! cargo run --example create_web_call_book_restaurant
//! ```
//!
//! Join within ~30s via `RetellWebClient.startCall` with the printed JSON
//! (`access_token`, `call_id`, `transport`, `ice_servers`). See
//! https://docs.retellai.com/deploy/web-call

use retell_sdk::types::{
    CreateAgentParams, CreateRetellLlmParams, CreateWebCallParams, ResponseEngine,
};
use retell_sdk::RetellClient;
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> retell_sdk::Result<()> {
    let api_key = env_required("RETELL_API_KEY");
    let restaurant = env_or("RESTAURANT_NAME", "Bistro Nova");
    let party_size = env_or("PARTY_SIZE", "4");
    let when = env_or("RESERVATION_TIME", "tonight at 7:30pm");
    let customer = env_or("CUSTOMER_NAME", "Alex Rivera");
    let voice_id = env_or("RETELL_VOICE_ID", "retell-Cimo");

    let client = RetellClient::new(api_key);

    let agent_id = match std::env::var("RETELL_AGENT_ID") {
        Ok(id) if !id.is_empty() => {
            eprintln!("Using existing agent {id}");
            id
        }
        _ => {
            eprintln!("Creating restaurant-booking guest agent…");
            create_booking_guest_agent(&client, &voice_id).await?
        }
    };

    let mut vars = HashMap::new();
    vars.insert("restaurant_name".into(), restaurant.clone());
    vars.insert("party_size".into(), party_size.clone());
    vars.insert("reservation_time".into(), when.clone());
    vars.insert("customer_name".into(), customer.clone());

    eprintln!(
        "Starting web call — agent books {restaurant} for {party_size} ({customer}) at {when}"
    );
    eprintln!("You will play restaurant reception when you join.");

    let call = client
        .call()
        .create_web_call(
            &CreateWebCallParams::new(&agent_id)
                .dynamic_variables(vars)
                .metadata(HashMap::from([(
                    "scenario".into(),
                    json!("restaurant_booking_web_roleplay"),
                )])),
        )
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

    // Machine-readable join payload on stdout (status notes go to stderr).
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "call_id": call.call_id,
            "access_token": call.access_token,
            "transport": call.transport,
            "expires_at": call.expires_at,
            "ice_servers": ice_servers,
            "agent_id": agent_id,
            "roleplay": {
                "you": "restaurant reception",
                "agent": "guest booking a table",
                "restaurant_name": restaurant,
                "party_size": party_size,
                "reservation_time": when,
                "customer_name": customer,
            }
        }))
        .unwrap()
    );

    Ok(())
}

async fn create_booking_guest_agent(
    client: &RetellClient,
    voice_id: &str,
) -> retell_sdk::Result<String> {
    let prompt = r#"
You are a polite guest calling a restaurant to book a dinner reservation.
The person on the other end of this call is the restaurant reception — speak to them as such.

Reservation details (dynamic variables — use these exactly):
- Your name: {{customer_name}}
- Party size: {{party_size}} people
- Preferred time: {{reservation_time}}
- Restaurant: {{restaurant_name}}

Goals:
1. Greet the host and say you would like to book a table at {{restaurant_name}}.
2. Ask for a table for {{party_size}} at {{reservation_time}} under the name {{customer_name}}.
3. If that time is full, ask for the closest alternative the same evening.
4. Confirm name, party size, and time before ending.
5. Thank them. When done, use the end_call tool.

Keep replies short (1–2 sentences). Do not invent a confirmation number unless the host gives one.
"#
    .trim();

    let llm = client
        .llm()
        .create(
            &CreateRetellLlmParams::new()
                .general_prompt(prompt)
                .begin_message(
                    "Hi, I'd like to make a dinner reservation — is this a good time?",
                )
                .general_tools(vec![json!({
                    "type": "end_call",
                    "name": "end_call",
                    "description": "End the call once the booking is confirmed or clearly declined."
                })]),
        )
        .await?;
    eprintln!("  llm_id:   {}", llm.llm_id);

    let agent = client
        .agent()
        .create(
            &CreateAgentParams::new(ResponseEngine::retell_llm(&llm.llm_id), voice_id)
                .agent_name("Restaurant booking guest")
                .language("en-US"),
        )
        .await?;
    eprintln!("  agent_id: {}", agent.agent_id);

    Ok(agent.agent_id)
}

fn env_required(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("set {key}"))
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}
