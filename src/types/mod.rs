//! Request and response types for the Retell API.
//!
//! Complex nested configuration objects that vary widely across agent /
//! conversation-flow setups are represented with [`serde_json::Value`] so the
//! SDK stays forward-compatible with OpenAPI revisions. Commonly used fields
//! are strongly typed.

mod agent;
mod call;
mod chat;
mod common;
mod concurrency;
mod conversation_flow;
mod knowledge_base;
mod llm;
mod phone_number;
#[cfg(test)]
mod serde_tests;
mod voice;

pub use agent::*;
pub use call::*;
pub use chat::*;
pub use common::*;
pub use concurrency::*;
pub use conversation_flow::*;
pub use knowledge_base::*;
pub use llm::*;
pub use phone_number::*;
pub use voice::*;
