use basemodel::with_basemodel;
use chrono::{DateTime, Local};
use serde::Deserialize;

#[with_basemodel]
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub(crate) struct TestCase {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) headers: serde_json::Value,
    pub(crate) query: serde_json::Value,
    pub(crate) body: Option<serde_json::Value>,
    pub(crate) method: String,
}
