use crate::error::Result;
use crate::http::{enc, HttpClient};
use crate::types::{AddKnowledgeBaseSourcesParams, CreateKnowledgeBaseParams, KnowledgeBase};
use reqwest::Method;

/// Knowledge base operations (`client.knowledge_base()`).
#[derive(Clone)]
pub struct KnowledgeBaseClient {
    pub(crate) http: HttpClient,
}

impl KnowledgeBaseClient {
    /// `POST /create-knowledge-base`
    pub async fn create(&self, params: &CreateKnowledgeBaseParams) -> Result<KnowledgeBase> {
        self.http
            .send_json(Method::POST, "/create-knowledge-base", params)
            .await
    }

    /// `GET /get-knowledge-base/{knowledge_base_id}`
    pub async fn get(&self, knowledge_base_id: &str) -> Result<KnowledgeBase> {
        self.http
            .send_empty(
                Method::GET,
                &format!("/get-knowledge-base/{}", enc(knowledge_base_id)),
            )
            .await
    }

    /// `GET /list-knowledge-bases`
    pub async fn list(&self) -> Result<Vec<KnowledgeBase>> {
        self.http
            .send_empty(Method::GET, "/list-knowledge-bases")
            .await
    }

    /// `DELETE /delete-knowledge-base/{knowledge_base_id}`
    pub async fn delete(&self, knowledge_base_id: &str) -> Result<()> {
        self.http
            .send_no_content(
                Method::DELETE,
                &format!("/delete-knowledge-base/{}", enc(knowledge_base_id)),
            )
            .await
    }

    /// `POST /add-knowledge-base-sources/{knowledge_base_id}`
    pub async fn add_sources(
        &self,
        knowledge_base_id: &str,
        params: &AddKnowledgeBaseSourcesParams,
    ) -> Result<KnowledgeBase> {
        self.http
            .send_json(
                Method::POST,
                &format!("/add-knowledge-base-sources/{}", enc(knowledge_base_id)),
                params,
            )
            .await
    }

    /// `DELETE /delete-knowledge-base-source/{knowledge_base_id}/source/{source_id}`
    pub async fn delete_source(
        &self,
        knowledge_base_id: &str,
        source_id: &str,
    ) -> Result<()> {
        self.http
            .send_no_content(
                Method::DELETE,
                &format!(
                    "/delete-knowledge-base-source/{}/source/{}",
                    enc(knowledge_base_id),
                    enc(source_id)
                ),
            )
            .await
    }
}
