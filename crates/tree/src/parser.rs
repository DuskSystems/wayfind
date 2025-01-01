use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    pub input: Arc<str>,
    pub raw: Arc<str>,
    pub parts: Vec<Part>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Part {
    Static {
        prefix: Vec<u8>,
    },

    Dynamic {
        name: String,
        constraint: Option<String>,
    },

    Wildcard {
        name: String,
        constraint: Option<String>,
    },
}
