use basemodel::with_basemodel;
use chrono::{DateTime, Local};

#[with_basemodel]
pub(crate) struct Variable {
    pub(crate) name: String,
    pub(crate) value: String,
}
