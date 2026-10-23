//! List available voices.
//!
//! ```sh
//! export RETELL_API_KEY=key_...
//! cargo run --example list_voices
//! ```

use retell_sdk::RetellClient;

#[tokio::main]
async fn main() -> retell_sdk::Result<()> {
    let api_key = std::env::var("RETELL_API_KEY").expect("set RETELL_API_KEY");
    let client = RetellClient::new(api_key);

    let voices = client.voice().list().await?;
    println!("Found {} voices:\n", voices.len());
    for v in voices.iter().take(20) {
        println!(
            "  {:<28} {:<12} {} ({})",
            v.voice_id, v.provider, v.voice_name, v.gender
        );
    }
    if voices.len() > 20 {
        println!("  … and {} more", voices.len() - 20);
    }
    Ok(())
}
