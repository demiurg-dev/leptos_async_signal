use serde::{Deserialize, Serialize};

/// A post representation model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub body: String,
}
