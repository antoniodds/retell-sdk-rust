//! Resource-oriented sub-clients accessed via [`crate::RetellClient`].

mod agent;
mod call;
mod chat;
mod concurrency;
mod conversation_flow;
mod knowledge_base;
mod llm;
mod phone_number;
mod voice;

pub use agent::AgentClient;
pub use call::CallClient;
pub use chat::ChatClient;
pub use concurrency::ConcurrencyClient;
pub use conversation_flow::ConversationFlowClient;
pub use knowledge_base::KnowledgeBaseClient;
pub use llm::LlmClient;
pub use phone_number::PhoneNumberClient;
pub use voice::VoiceClient;
