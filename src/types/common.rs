use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Arbitrary JSON object (metadata, dynamic variables extras, etc.).
pub type JsonObject = HashMap<String, Value>;

/// String map used for Retell LLM dynamic variables.
pub type StringMap = HashMap<String, String>;

/// Pagination / sort query parameters shared by list endpoints.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_key: Option<String>,
}

impl ListParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn sort_order(mut self, order: SortOrder) -> Self {
        self.sort_order = Some(order);
        self
    }

    pub fn pagination_key(mut self, key: impl Into<String>) -> Self {
        self.pagination_key = Some(key.into());
        self
    }

    pub(crate) fn as_query(&self) -> Vec<(&'static str, String)> {
        let mut q = Vec::new();
        if let Some(limit) = self.limit {
            q.push(("limit", limit.to_string()));
        }
        if let Some(order) = &self.sort_order {
            q.push((
                "sort_order",
                match order {
                    SortOrder::Ascending => "ascending".into(),
                    SortOrder::Descending => "descending".into(),
                },
            ));
        }
        if let Some(key) = &self.pagination_key {
            q.push(("pagination_key", key.clone()));
        }
        q
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// Standard paginated list envelope.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub has_more: bool,
    #[serde(default)]
    pub pagination_key: Option<String>,
    #[serde(default)]
    pub total: Option<i64>,
}

/// Agent version: numeric version or tag (`"latest"`, `"prod"`, …).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AgentVersionRef {
    Number(i64),
    Tag(String),
}

impl From<i64> for AgentVersionRef {
    fn from(v: i64) -> Self {
        Self::Number(v)
    }
}

impl From<&str> for AgentVersionRef {
    fn from(v: &str) -> Self {
        Self::Tag(v.to_string())
    }
}

impl From<String> for AgentVersionRef {
    fn from(v: String) -> Self {
        Self::Tag(v)
    }
}

/// Response engine attached to a voice agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ResponseEngine {
    #[serde(rename = "retell-llm")]
    RetellLlm {
        llm_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        version: Option<f64>,
    },
    #[serde(rename = "custom-llm")]
    CustomLlm { llm_websocket_url: String },
    #[serde(rename = "conversation-flow")]
    ConversationFlow {
        conversation_flow_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        version: Option<f64>,
    },
}

impl ResponseEngine {
    pub fn retell_llm(llm_id: impl Into<String>) -> Self {
        Self::RetellLlm {
            llm_id: llm_id.into(),
            version: None,
        }
    }

    pub fn custom_llm(websocket_url: impl Into<String>) -> Self {
        Self::CustomLlm {
            llm_websocket_url: websocket_url.into(),
        }
    }

    pub fn conversation_flow(id: impl Into<String>) -> Self {
        Self::ConversationFlow {
            conversation_flow_id: id.into(),
            version: None,
        }
    }
}
