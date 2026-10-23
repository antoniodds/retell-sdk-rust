//! Client configuration.

use std::time::Duration;

/// Default production API base URL.
pub const DEFAULT_BASE_URL: &str = "https://api.retellai.com";

/// Builder-style options for [`crate::RetellClient`].
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub(crate) api_key: String,
    pub(crate) base_url: String,
    pub(crate) timeout: Duration,
    pub(crate) user_agent: String,
}

impl ClientConfig {
    /// Create config with the given API key and sensible defaults.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout: Duration::from_secs(60),
            user_agent: format!("retell-sdk-rust/{}", env!("CARGO_PKG_VERSION")),
        }
    }

    /// Override the API base URL (useful for testing / proxies).
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into().trim_end_matches('/').to_string();
        self
    }

    /// Per-request timeout (default: 60s).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Custom `User-Agent` header value.
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = ua.into();
        self
    }

    /// Build a [`crate::RetellClient`] from this config.
    pub fn build(self) -> crate::RetellClient {
        crate::RetellClient::with_config(self)
    }
}
