use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum VirtaulColumn {
    Expression {
        name: String,
        expression: String,
        output_type: Option<String>,
    },
}

impl VirtaulColumn {
    pub fn expression(name: String, expression: String, output_type: Option<String>) -> Self {
        Self::Expression {
            name,
            expression,
            output_type,
        }
    }
}
