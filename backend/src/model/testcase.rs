use basemodel::with_basemodel;
use chrono::{DateTime, Local};
use serde::Deserialize;

#[with_basemodel]
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TestCase {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub headers: serde_json::Value,
    pub query: serde_json::Value,
    pub body: Option<serde_json::Value>,
    pub method: String,
}
