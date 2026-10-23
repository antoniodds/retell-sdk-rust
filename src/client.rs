//! Main [`RetellClient`] entry point.

use crate::config::ClientConfig;
use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::resources::{
    AgentClient, CallClient, ChatClient, ConcurrencyClient, ConversationFlowClient,
    KnowledgeBaseClient, LlmClient, PhoneNumberClient, VoiceClient,
};
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;

/// Asynchronous Retell AI API client.
///
/// Construct with [`RetellClient::new`] or [`RetellClient::builder`], then access
/// resource sub-clients such as [`RetellClient::agent`] and [`RetellClient::call`].
///
/// ```no_run
/// use retell_sdk::RetellClient;
///
/// # async fn demo() -> retell_sdk::Result<()> {
/// let client = RetellClient::new("key_...");
/// let voices = client.voice().list().await?;
/// println!("{} voices", voices.len());
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct RetellClient {
    http: HttpClient,
}

impl RetellClient {
    /// Create a client with the given API key and default settings.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_config(ClientConfig::new(api_key))
    }

    /// Start a configuration builder.
    pub fn builder(api_key: impl Into<String>) -> ClientConfig {
        ClientConfig::new(api_key)
    }

    /// Create a client from an explicit [`ClientConfig`].
    pub fn with_config(config: ClientConfig) -> Self {
        let inner = Client::builder()
            .timeout(config.timeout)
            .build()
            .expect("failed to build reqwest client");

        Self {
            http: HttpClient {
                inner,
                api_key: Arc::new(config.api_key),
                base_url: Arc::new(config.base_url),
                user_agent: Arc::new(config.user_agent),
            },
        }
    }

    /// Create a client pointing at a custom base URL (e.g. wiremock).
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self::with_config(ClientConfig::new(api_key).base_url(base_url))
    }

    /// Override request timeout after construction via a new client.
    pub fn with_timeout(api_key: impl Into<String>, timeout: Duration) -> Self {
        Self::with_config(ClientConfig::new(api_key).timeout(timeout))
    }

    /// Voice agents & chat agents.
    pub fn agent(&self) -> AgentClient {
        AgentClient {
            http: self.http.clone(),
        }
    }

    /// Phone / web / batch calls.
    pub fn call(&self) -> CallClient {
        CallClient {
            http: self.http.clone(),
        }
    }

    /// API & SMS chats.
    pub fn chat(&self) -> ChatClient {
        ChatClient {
            http: self.http.clone(),
        }
    }

    /// Phone numbers.
    pub fn phone_number(&self) -> PhoneNumberClient {
        PhoneNumberClient {
            http: self.http.clone(),
        }
    }

    /// Voice catalog.
    pub fn voice(&self) -> VoiceClient {
        VoiceClient {
            http: self.http.clone(),
        }
    }

    /// Knowledge bases.
    pub fn knowledge_base(&self) -> KnowledgeBaseClient {
        KnowledgeBaseClient {
            http: self.http.clone(),
        }
    }

    /// Retell LLM response engines.
    pub fn llm(&self) -> LlmClient {
        LlmClient {
            http: self.http.clone(),
        }
    }

    /// Conversation flows.
    pub fn conversation_flow(&self) -> ConversationFlowClient {
        ConversationFlowClient {
            http: self.http.clone(),
        }
    }

    /// Concurrency limits and API key info.
    pub fn concurrency(&self) -> ConcurrencyClient {
        ConcurrencyClient {
            http: self.http.clone(),
        }
    }
}

impl From<ClientConfig> for RetellClient {
    fn from(config: ClientConfig) -> Self {
        Self::with_config(config)
    }
}

/// Ensure `Error` is nameable from client docs.
#[allow(dead_code)]
fn _error_link(_: Error) -> Result<()> {
    Ok(())
}
