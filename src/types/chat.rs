use super::{AgentVersionRef, JsonObject, StringMap};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct CreateChatParams {
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<AgentVersionRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<JsonObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retell_llm_dynamic_variables: Option<StringMap>,
}

impl CreateChatParams {
    pub fn new(agent_id: impl Into<String>) -> Self {
        Self {
            agent_id: agent_id.into(),
            agent_version: None,
            metadata: None,
            retell_llm_dynamic_variables: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateSmsChatParams {
    pub from_number: String,
    pub to_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_agent_id: Option<String>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

impl CreateSmsChatParams {
    pub fn new(from_number: impl Into<String>, to_number: impl Into<String>) -> Self {
        Self {
            from_number: from_number.into(),
            to_number: to_number.into(),
            override_agent_id: None,
            extra: JsonObject::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateChatCompletionParams {
    pub chat_id: String,
    pub content: String,
}

impl CreateChatCompletionParams {
    pub fn new(chat_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatStatus {
    Ongoing,
    Ended,
    Error,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Chat {
    pub chat_id: String,
    pub agent_id: String,
    pub chat_status: ChatStatus,
    #[serde(default)]
    pub version: Option<i64>,
    #[serde(default)]
    pub chat_type: Option<String>,
    #[serde(default)]
    pub transcript: Option<String>,
    #[serde(default)]
    pub message_with_tool_calls: Option<Value>,
    #[serde(default)]
    pub chat_analysis: Option<Value>,
    #[serde(default)]
    pub start_timestamp: Option<i64>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListChatsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<super::SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateChatParams {
    #[serde(flatten)]
    pub extra: JsonObject,
}
