use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[serde(rename_all = "camelCase")]
#[error(r#"{what} is not a valid ordering, should be {should_be}"#)]
pub struct InvalidOrderingError {
    what: String,
    should_be: String,
}

impl InvalidOrderingError {
    pub fn new(what: String, should_be: String) -> Self {
        Self { what, should_be }
    }
}
