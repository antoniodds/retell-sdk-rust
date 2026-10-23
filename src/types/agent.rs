use super::{JsonObject, ResponseEngine};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Body for `POST /create-agent`.
#[derive(Debug, Clone, Serialize)]
pub struct CreateAgentParams {
    pub response_engine: ResponseEngine,
    pub voice_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_transfer_agent: Option<bool>,
    /// Additional AgentRequest fields (voice_temperature, webhook URLs, …).
    #[serde(flatten)]
    pub extra: JsonObject,
}

impl CreateAgentParams {
    pub fn new(response_engine: ResponseEngine, voice_id: impl Into<String>) -> Self {
        Self {
            response_engine,
            voice_id: voice_id.into(),
            agent_name: None,
            language: None,
            is_transfer_agent: None,
            extra: JsonObject::new(),
        }
    }

    pub fn agent_name(mut self, name: impl Into<String>) -> Self {
        self.agent_name = Some(name.into());
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn extra(mut self, key: impl Into<String>, value: Value) -> Self {
        self.extra.insert(key.into(), value);
        self
    }
}

/// Body for `PATCH /update-agent/{agent_id}`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateAgentParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_engine: Option<ResponseEngine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

/// Body for `POST /v2/list-agents`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAgentsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<Value>,
}

/// Agent returned by create / get / update.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Agent {
    pub agent_id: String,
    pub version: i64,
    #[serde(default)]
    pub agent_name: Option<String>,
    #[serde(default)]
    pub voice_id: Option<String>,
    #[serde(default)]
    pub response_engine: Option<ResponseEngine>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub is_published: Option<bool>,
    #[serde(default)]
    pub is_transfer_agent: Option<bool>,
    #[serde(default)]
    pub last_modification_timestamp: Option<i64>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

/// Summary row from `POST /v2/list-agents`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentListItem {
    pub agent_id: String,
    pub is_suspended: bool,
    pub agent_name: String,
    pub channel: String,
    pub user_modified_timestamp: i64,
    #[serde(default)]
    pub tags: Value,
    #[serde(default)]
    pub is_transfer_agent: Option<bool>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

/// Chat agent create/update/response (shares most fields with voice agents).
#[derive(Debug, Clone, Serialize)]
pub struct CreateChatAgentParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

impl CreateChatAgentParams {
    pub fn new() -> Self {
        Self {
            agent_name: None,
            extra: JsonObject::new(),
        }
    }

    pub fn agent_name(mut self, name: impl Into<String>) -> Self {
        self.agent_name = Some(name.into());
        self
    }

    pub fn extra(mut self, key: impl Into<String>, value: Value) -> Self {
        self.extra.insert(key.into(), value);
        self
    }
}

impl Default for CreateChatAgentParams {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatAgent {
    pub agent_id: String,
    #[serde(default)]
    pub agent_name: Option<String>,
    #[serde(default)]
    pub version: Option<i64>,
    #[serde(flatten)]
    pub extra: JsonObject,
}
