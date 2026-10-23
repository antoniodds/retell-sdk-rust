# retell-sdk (Rust)

Asynchronous Rust client for the [Retell AI](https://www.retellai.com/) REST API.

Built with **reqwest** (async HTTP) and **serde** (JSON). The API surface is modeled after Retell’s Node/Python SDKs: a main `RetellClient` plus modular sub-clients such as `client.agent()` and `client.call()`.

> Spec revision: based on [docs.retellai.com/openapi.yaml](https://docs.retellai.com/openapi.yaml) (API v3).

## Features

- Async/`await` throughout (`tokio` + `reqwest`)
- Bearer API-key auth
- Resource clients for agents, calls, chats, phone numbers, voices, knowledge bases, Retell LLMs, conversation flows, and concurrency
- Typed request/response models with `serde_json` escape hatches for forward-compatible fields
- Structured errors (`401`, `402`, `404`, `429`, …) via `retell_sdk::Error`
- Mock tests with `wiremock`; optional live tests behind a feature flag

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
retell-sdk = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

From git (unreleased main):

```toml
retell-sdk = { git = "https://github.com/antoniodds/retell-sdk-rust" }
```

Local path while developing:

```toml
retell-sdk = { path = "../retell-sdk-rust" }
```

## Configure

1. Create a free Retell account at [dashboard.retellai.com](https://dashboard.retellai.com/)  
2. Open **Settings → API Keys** and create a key.
3. Export it (never commit keys):

```bash
export RETELL_API_KEY=key_xxxxxxxx
```

Optional builder knobs:

```rust
use std::time::Duration;
use retell_sdk::{ClientConfig, RetellClient};

let client = RetellClient::with_config(
    ClientConfig::new(std::env::var("RETELL_API_KEY").unwrap())
        .timeout(Duration::from_secs(30))
        .base_url("https://api.retellai.com"), // default
);
```

## Quick examples

### List voices

```rust
use retell_sdk::RetellClient;

#[tokio::main]
async fn main() -> retell_sdk::Result<()> {
    let client = RetellClient::new(std::env::var("RETELL_API_KEY")?);
    for v in client.voice().list().await? {
        println!("{} ({})", v.voice_name, v.voice_id);
    }
    Ok(())
}
```

### Create an LLM + agent

```rust
use retell_sdk::types::{CreateAgentParams, CreateRetellLlmParams, ResponseEngine};
use retell_sdk::RetellClient;

#[tokio::main]
async fn main() -> retell_sdk::Result<()> {
    let client = RetellClient::new(std::env::var("RETELL_API_KEY")?);

    let llm = client
        .llm()
        .create(
            &CreateRetellLlmParams::new()
                .general_prompt("You are a friendly receptionist.")
                .begin_message("Hi, how can I help you today?"),
        )
        .await?;

    let agent = client
        .agent()
        .create(
            &CreateAgentParams::new(ResponseEngine::retell_llm(llm.llm_id), "retell-Cimo")
                .agent_name("Receptionist"),
        )
        .await?;

    println!("created agent {}", agent.agent_id);
    Ok(())
}
```

### Start a web call (no phone number needed)

```rust
use retell_sdk::types::CreateWebCallParams;
use retell_sdk::RetellClient;

#[tokio::main]
async fn main() -> retell_sdk::Result<()> {
    let client = RetellClient::new(std::env::var("RETELL_API_KEY")?);
    let agent_id = std::env::var("RETELL_AGENT_ID")?;

    let web = client
        .call()
        .create_web_call(&CreateWebCallParams::new(agent_id))
        .await?;

    println!("call_id={} token={}", web.call_id, web.access_token);
    Ok(())
}
```

### Place an outbound phone call

Requires a Retell (or imported) number and usually a payment method:

```rust
use retell_sdk::types::CreatePhoneCallParams;
use retell_sdk::RetellClient;

#[tokio::main]
async fn main() -> retell_sdk::Result<()> {
    let client = RetellClient::new(std::env::var("RETELL_API_KEY")?);
    let call = client
        .call()
        .create_phone_call(&CreatePhoneCallParams::new(
            "+14157774444", // your Retell number
            "+12137774445", // destination (E.164)
        ).override_agent_id("agent_..."))
        .await?;
    println!("started {}", call.call_id);
    Ok(())
}
```

### Error handling

```rust
match client.agent().get("agent_missing").await {
    Ok(agent) => println!("{:?}", agent),
    Err(e) if e.is_unauthorized() => eprintln!("bad API key"),
    Err(e) if e.is_rate_limited() => eprintln!("slow down"),
    Err(e) => eprintln!("request failed: {e}"),
}
```

## Runnable examples

```bash
export RETELL_API_KEY=key_...
cargo run --example list_voices
cargo run --example list_agents
cargo run --example create_agent   # prints agent_id; creates LLM + voice agent

export RETELL_AGENT_ID=agent_...
cargo run --example create_web_call

# Web-call roleplay: agent books dinner for 4; you play restaurant reception
cargo run --example create_web_call_book_restaurant
# Join within ~30s with the printed access_token via the Retell Web SDK
```

## Testing

### Unit / mock tests (default)

No API key required — uses [wiremock](https://crates.io/crates/wiremock):

```bash
cargo test
```

### Live API smoke tests

```bash
export RETELL_API_KEY=key_...
cargo test --features live-tests --test live_api -- --ignored --nocapture
```

## Manual end-to-end checklist

Use this to confirm the SDK against a real trial workspace:

| Step | What to do | Cost |
|------|------------|------|
| 1 | Sign up at [dashboard.retellai.com](https://dashboard.retellai.com/) | Free ($10 credits) |
| 2 | **Settings → API Keys → Add** → copy key → `export RETELL_API_KEY=...` | Free |
| 3 | `cargo run --example list_voices` | Free |
| 4 | `cargo run --example list_agents` (empty until you create one) | Free |
| 5 | In the dashboard: **Agents → Create** (Single prompt template is fine) → copy `agent_id` | Free |
| 6 | `RETELL_AGENT_ID=... cargo run --example create_web_call` | Free (token only) |
| 7 | Dashboard **Test** (browser mic) or feed `access_token` into Retell’s web SDK | Free for dashboard web test |
| 8 | Optional: create LLM + agent via the “Create an LLM + agent” snippet above, then list again | Uses tiny credit if you place real calls |
| 9 | Optional phone: add a payment method, buy a number, `create_phone_call` | Paid ($2+/mo number + per-minute) |

Tips:

- **Web calls / playground tests do not require a phone number or card.**
- Trial credits cover API usage; buying a number needs billing.
- Keys live under **System Settings → API Keys**. Prefer a scoped key if you share it with CI.
- Base URL is always `https://api.retellai.com` (override only for mocks/proxies).

## API coverage

| Sub-client | Notable methods |
|------------|-----------------|
| `agent()` | `create`, `get`, `update`, `delete`, `list` / `list_all`, versions, chat agents |
| `call()` | `create_phone_call`, `create_web_call`, `get`, `list`, `stop`, `delete`, batch, live ops |
| `chat()` | `create`, `create_sms`, `get`, `list`, `end`, completions |
| `phone_number()` | `create`, `import`, `get`, `list`, `update`, `delete` |
| `voice()` | `list`, `get`, clone / community helpers |
| `knowledge_base()` | CRUD + sources |
| `llm()` | Retell LLM response-engine CRUD |
| `conversation_flow()` | flows + components |
| `concurrency()` | `get`, `api_key_info` |

Complex nested agent / flow configuration fields that change often in the OpenAPI spec are accepted via `extra` maps or `serde_json::Value` so the crate stays compatible as Retell ships new options. Core identifiers, statuses, and list envelopes are strongly typed.

## License

Apache-2.0
