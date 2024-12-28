use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub body: String,
}

impl Post {
    pub fn body_lines(&self) -> impl Iterator<Item = &str>  {
        self.body.lines().map(|line| line.trim()).filter(|line| !line.is_empty())
    }
}