use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Ordering {
    Alphanumeric,
    Lexicographic,
    Numeric,
    Strlen,
}
