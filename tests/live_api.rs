//! Optional live tests against the real Retell API.
//!
//! ```sh
//! export RETELL_API_KEY=key_...
//! cargo test --features live-tests --test live_api -- --ignored --nocapture
//! ```

#![cfg(feature = "live-tests")]

use retell_sdk::RetellClient;

fn client() -> RetellClient {
    let key = std::env::var("RETELL_API_KEY").expect("RETELL_API_KEY must be set for live tests");
    RetellClient::new(key)
}

#[tokio::test]
#[ignore = "hits the live Retell API — requires RETELL_API_KEY"]
async fn live_list_voices() {
    let voices = client().voice().list().await.expect("list voices");
    assert!(!voices.is_empty(), "expected at least one platform voice");
    println!("voices: {}", voices.len());
}

#[tokio::test]
#[ignore = "hits the live Retell API — requires RETELL_API_KEY"]
async fn live_get_concurrency() {
    let c = client().concurrency().get().await.expect("concurrency");
    println!(
        "concurrency {} / {}",
        c.current_concurrency, c.concurrency_limit
    );
    assert!(c.concurrency_limit >= 0);
}

#[tokio::test]
#[ignore = "hits the live Retell API — requires RETELL_API_KEY"]
async fn live_list_agents() {
    let page = client().agent().list_all().await.expect("list agents");
    println!("agents: {}", page.items.len());
}

#[tokio::test]
#[ignore = "hits the live Retell API — requires RETELL_API_KEY"]
async fn live_api_key_info() {
    let info = client()
        .concurrency()
        .api_key_info()
        .await
        .expect("api key info");
    println!("{info}");
}
