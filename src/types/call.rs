use super::{AgentVersionRef, JsonObject, StringMap};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `POST /v2/create-phone-call`
#[derive(Debug, Clone, Serialize)]
pub struct CreatePhoneCallParams {
    pub from_number: String,
    pub to_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_agent_version: Option<AgentVersionRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<JsonObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retell_llm_dynamic_variables: Option<StringMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

impl CreatePhoneCallParams {
    pub fn new(from_number: impl Into<String>, to_number: impl Into<String>) -> Self {
        Self {
            from_number: from_number.into(),
            to_number: to_number.into(),
            override_agent_id: None,
            override_agent_version: None,
            metadata: None,
            retell_llm_dynamic_variables: None,
            idempotency_key: None,
            extra: JsonObject::new(),
        }
    }

    pub fn override_agent_id(mut self, id: impl Into<String>) -> Self {
        self.override_agent_id = Some(id.into());
        self
    }

    pub fn dynamic_variables(mut self, vars: StringMap) -> Self {
        self.retell_llm_dynamic_variables = Some(vars);
        self
    }

    pub fn metadata(mut self, metadata: JsonObject) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// `POST /v3/create-web-call`
#[derive(Debug, Clone, Serialize)]
pub struct CreateWebCallParams {
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<AgentVersionRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<JsonObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retell_llm_dynamic_variables: Option<StringMap>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

impl CreateWebCallParams {
    pub fn new(agent_id: impl Into<String>) -> Self {
        Self {
            agent_id: agent_id.into(),
            agent_version: None,
            metadata: None,
            retell_llm_dynamic_variables: None,
            extra: JsonObject::new(),
        }
    }

    pub fn agent_version(mut self, version: impl Into<AgentVersionRef>) -> Self {
        self.agent_version = Some(version.into());
        self
    }

    pub fn dynamic_variables(mut self, vars: StringMap) -> Self {
        self.retell_llm_dynamic_variables = Some(vars);
        self
    }

    pub fn metadata(mut self, metadata: JsonObject) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// `POST /v2/register-phone-call`
#[derive(Debug, Clone, Serialize)]
pub struct RegisterPhoneCallParams {
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<CallDirection>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallDirection {
    Inbound,
    Outbound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallStatus {
    Registered,
    NotConnected,
    Ongoing,
    Ended,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallType {
    PhoneCall,
    WebCall,
}

/// Unified call object (phone or web). Extra telephony fields live in `extra`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Call {
    pub call_id: String,
    pub agent_id: String,
    pub agent_version: i64,
    pub call_status: CallStatus,
    #[serde(default)]
    pub call_type: Option<CallType>,
    #[serde(default)]
    pub agent_name: Option<String>,
    #[serde(default)]
    pub from_number: Option<String>,
    #[serde(default)]
    pub to_number: Option<String>,
    #[serde(default)]
    pub direction: Option<CallDirection>,
    #[serde(default)]
    pub metadata: Option<Value>,
    #[serde(default)]
    pub transcript: Option<String>,
    #[serde(default)]
    pub transcript_object: Option<Value>,
    #[serde(default)]
    pub call_analysis: Option<Value>,
    #[serde(default)]
    pub start_timestamp: Option<i64>,
    #[serde(default)]
    pub end_timestamp: Option<i64>,
    #[serde(default)]
    pub duration_ms: Option<i64>,
    #[serde(default)]
    pub recording_url: Option<String>,
    #[serde(default)]
    pub public_log_url: Option<String>,
    #[serde(default)]
    pub disconnection_reason: Option<String>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

/// `POST /v3/create-web-call` response.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateWebCallResponse {
    pub call_id: String,
    pub access_token: String,
    pub transport: String,
    pub ice_servers: Vec<IceServer>,
    pub expires_at: i64,
    #[serde(flatten)]
    pub extra: JsonObject,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IceServer {
    pub urls: IceUrls,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub credential: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IceUrls {
    One(String),
    Many(Vec<String>),
}

/// `POST /v3/list-calls`
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListCallsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<super::SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_total: Option<bool>,
}

/// `POST /create-batch-call`
#[derive(Debug, Clone, Serialize)]
pub struct CreateBatchCallParams {
    pub from_number: String,
    pub tasks: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_timestamp: Option<f64>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchCall {
    pub batch_call_id: String,
    pub name: String,
    pub from_number: String,
    pub scheduled_timestamp: f64,
    pub total_task_count: f64,
    #[serde(flatten)]
    pub extra: JsonObject,
}
