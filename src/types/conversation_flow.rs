use super::JsonObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateConversationFlowParams {
    #[serde(flatten)]
    pub extra: JsonObject,
}

impl CreateConversationFlowParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extra(mut self, key: impl Into<String>, value: Value) -> Self {
        self.extra.insert(key.into(), value);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateConversationFlowParams {
    #[serde(flatten)]
    pub extra: JsonObject,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConversationFlow {
    pub conversation_flow_id: String,
    #[serde(default)]
    pub version: Option<i64>,
    #[serde(flatten)]
    pub extra: JsonObject,
}
