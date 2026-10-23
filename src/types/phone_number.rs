use super::JsonObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreatePhoneNumberParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_agents: Option<Vec<AgentWeight>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_agents: Option<Vec<AgentWeight>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_webhook_url: Option<String>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

impl CreatePhoneNumberParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn area_code(mut self, code: i32) -> Self {
        self.area_code = Some(code);
        self
    }

    pub fn nickname(mut self, name: impl Into<String>) -> Self {
        self.nickname = Some(name.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentWeight {
    pub agent_id: String,
    pub weight: f64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdatePhoneNumberParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_agents: Option<Vec<AgentWeight>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_agents: Option<Vec<AgentWeight>>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImportPhoneNumberParams {
    pub phone_number: String,
    #[serde(flatten)]
    pub extra: JsonObject,
}

impl ImportPhoneNumberParams {
    pub fn new(phone_number: impl Into<String>) -> Self {
        Self {
            phone_number: phone_number.into(),
            extra: JsonObject::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PhoneNumber {
    pub phone_number: String,
    pub phone_number_type: String,
    pub last_modification_timestamp: i64,
    #[serde(default)]
    pub phone_number_pretty: Option<String>,
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(default)]
    pub area_code: Option<i32>,
    #[serde(default)]
    pub inbound_agents: Option<Value>,
    #[serde(default)]
    pub outbound_agents: Option<Value>,
    #[serde(flatten)]
    pub extra: JsonObject,
}
