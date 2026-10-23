use super::JsonObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRetellLlmParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_tools: Option<Vec<Value>>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

impl CreateRetellLlmParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn general_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.general_prompt = Some(prompt.into());
        self
    }

    pub fn begin_message(mut self, message: impl Into<String>) -> Self {
        self.begin_message = Some(message.into());
        self
    }

    pub fn general_tools(mut self, tools: Vec<Value>) -> Self {
        self.general_tools = Some(tools);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateRetellLlmParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_message: Option<String>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RetellLlm {
    pub llm_id: String,
    #[serde(default)]
    pub version: Option<i64>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub general_prompt: Option<String>,
    #[serde(default)]
    pub begin_message: Option<String>,
    #[serde(default)]
    pub is_published: Option<bool>,
    #[serde(default)]
    pub last_modification_timestamp: Option<i64>,
    #[serde(flatten)]
    pub extra: JsonObject,
}
