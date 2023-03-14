use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeleteByIds {
    pub ids: Vec<uuid::Uuid>,
}
