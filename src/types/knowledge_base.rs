use super::JsonObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct CreateKnowledgeBaseParams {
    pub knowledge_base_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_texts: Option<Vec<KnowledgeBaseText>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_refresh: Option<bool>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

impl CreateKnowledgeBaseParams {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            knowledge_base_name: name.into(),
            knowledge_base_texts: None,
            knowledge_base_urls: None,
            enable_auto_refresh: None,
            extra: JsonObject::new(),
        }
    }

    pub fn texts(mut self, texts: Vec<KnowledgeBaseText>) -> Self {
        self.knowledge_base_texts = Some(texts);
        self
    }

    pub fn urls(mut self, urls: Vec<String>) -> Self {
        self.knowledge_base_urls = Some(urls);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBaseText {
    pub title: String,
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KnowledgeBase {
    pub knowledge_base_id: String,
    pub knowledge_base_name: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub knowledge_base_sources: Option<Value>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AddKnowledgeBaseSourcesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_texts: Option<Vec<KnowledgeBaseText>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_urls: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: JsonObject,
}
