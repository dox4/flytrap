use basemodel::with_basemodel;
use chrono::{DateTime, Local};

#[with_basemodel]
pub struct Variable {
    pub name: String,
    pub value: String,
}
