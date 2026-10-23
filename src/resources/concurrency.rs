use crate::error::Result;
use crate::http::HttpClient;
use crate::types::Concurrency;
use reqwest::Method;
use serde_json::Value;

/// Account concurrency & identity helpers (`client.concurrency()`).
#[derive(Clone)]
pub struct ConcurrencyClient {
    pub(crate) http: HttpClient,
}

impl ConcurrencyClient {
    /// `GET /get-concurrency`
    pub async fn get(&self) -> Result<Concurrency> {
        self.http
            .send_empty(Method::GET, "/get-concurrency")
            .await
    }

    /// `GET /get-api-key-info`
    pub async fn api_key_info(&self) -> Result<Value> {
        self.http
            .send_empty(Method::GET, "/get-api-key-info")
            .await
    }
}
