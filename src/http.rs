//! Internal HTTP helpers shared by resource clients.

use crate::error::{Error, Result};
use reqwest::{Client, Method, RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct HttpClient {
    pub(crate) inner: Client,
    pub(crate) api_key: Arc<String>,
    pub(crate) base_url: Arc<String>,
    pub(crate) user_agent: Arc<String>,
}

impl HttpClient {
    pub(crate) fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        self.inner
            .request(method, url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("User-Agent", self.user_agent.as_str())
            .header("Accept", "application/json")
    }

    pub(crate) async fn send_empty<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
    ) -> Result<T> {
        let response = self.request(method, path).send().await?;
        Self::parse(response).await
    }

    pub(crate) async fn send_json<B: Serialize, T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let response = self
            .request(method, path)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?;
        Self::parse(response).await
    }

    pub(crate) async fn send_no_content(&self, method: Method, path: &str) -> Result<()> {
        let response = self.request(method, path).send().await?;
        let status = response.status();
        if status.is_success() || status == StatusCode::NO_CONTENT {
            return Ok(());
        }
        Err(Error::from_response(response).await)
    }

    pub(crate) async fn send_query<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<T> {
        let response = self.request(method, path).query(query).send().await?;
        Self::parse(response).await
    }

    pub(crate) async fn send_query_no_content(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<()> {
        let response = self.request(method, path).query(query).send().await?;
        let status = response.status();
        if status.is_success() || status == StatusCode::NO_CONTENT {
            return Ok(());
        }
        Err(Error::from_response(response).await)
    }

    pub(crate) async fn send_json_query<B: Serialize, T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: &B,
        query: &[(&str, String)],
    ) -> Result<T> {
        let response = self
            .request(method, path)
            .header("Content-Type", "application/json")
            .query(query)
            .json(body)
            .send()
            .await?;
        Self::parse(response).await
    }

    async fn parse<T: DeserializeOwned>(response: Response) -> Result<T> {
        let status = response.status();
        if status.is_success() {
            let bytes = response.bytes().await?;
            if bytes.is_empty() {
                // Some endpoints return 200/201 with empty body; deserialize `()` via null.
                return serde_json::from_value(serde_json::Value::Null).map_err(Error::Decode);
            }
            serde_json::from_slice(&bytes).map_err(Error::Decode)
        } else {
            Err(Error::from_response(response).await)
        }
    }
}

/// Escape a path segment for use in URLs.
pub(crate) fn enc(segment: &str) -> String {
    urlencoding_minimal(segment)
}

fn urlencoding_minimal(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            // Keep '+' for E.164 phone numbers in path (Retell expects +1415...)
            b'+' => out.push('+'),
            _ => {
                out.push('%');
                out.push(char::from(hex(b >> 4)));
                out.push(char::from(hex(b & 0xf)));
            }
        }
    }
    out
}

fn hex(n: u8) -> u8 {
    if n < 10 {
        b'0' + n
    } else {
        b'A' + (n - 10)
    }
}
