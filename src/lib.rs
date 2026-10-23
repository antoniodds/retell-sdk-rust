//! # retell-sdk
//!
//! Asynchronous Rust client for the [Retell AI](https://www.retellai.com/) API.
//!
//! ## Quick start
//!
//! ```no_run
//! use retell_sdk::RetellClient;
//!
//! #[tokio::main]
//! async fn main() -> retell_sdk::Result<()> {
//!     let client = RetellClient::new(std::env::var("RETELL_API_KEY").unwrap());
//!     let agents = client.agent().list_all().await?;
//!     for agent in agents.items {
//!         println!("{} — {}", agent.agent_id, agent.agent_name);
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ## Resource clients
//!
//! | Method | Resource |
//! |--------|----------|
//! | [`RetellClient::agent`] | Voice & chat agents |
//! | [`RetellClient::call`] | Phone / web / batch calls |
//! | [`RetellClient::chat`] | API & SMS chats |
//! | [`RetellClient::phone_number`] | Phone numbers |
//! | [`RetellClient::voice`] | Voice catalog |
//! | [`RetellClient::knowledge_base`] | Knowledge bases |
//! | [`RetellClient::llm`] | Retell LLM response engines |
//! | [`RetellClient::conversation_flow`] | Conversation flows |
//! | [`RetellClient::concurrency`] | Concurrency & API key info |
//!
//! Authentication uses a bearer API key from the
//! [Retell dashboard](https://dashboard.retellai.com/).


mod client;
mod config;
mod error;
mod http;
pub mod resources;
pub mod types;

pub use client::RetellClient;
pub use config::{ClientConfig, DEFAULT_BASE_URL};
pub use error::{ApiErrorBody, Error, Result};

/// Crate version string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
