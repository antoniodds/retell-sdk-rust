use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Concurrency {
    pub current_concurrency: i64,
    pub concurrency_limit: i64,
    #[serde(default)]
    pub base_concurrency: Option<i64>,
    #[serde(default)]
    pub purchased_concurrency: Option<i64>,
    #[serde(default)]
    pub concurrency_purchase_limit: Option<i64>,
    #[serde(default)]
    pub remaining_purchase_limit: Option<i64>,
    #[serde(default)]
    pub reserved_inbound_concurrency: Option<i64>,
}
