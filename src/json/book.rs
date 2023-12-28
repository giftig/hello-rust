use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(super) struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String
}
