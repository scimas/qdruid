use serde::{Deserialize, Serialize};

use crate::components::{context::Context, druid_types::DruidSqlType};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sql {
    pub query: String,
    pub result_format: Option<ResultFormat>,
    pub header: Option<bool>,
    pub parameters: Option<Vec<DruidSqlType>>,
    pub context: Option<Context>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResultFormat {
    Object,
    Array,
    ObjectLines,
    ArrayLines,
    Csv,
}

impl Sql {
    pub fn new(query: &str) -> Self {
        Self {
            query: query.into(),
            result_format: None,
            header: None,
            parameters: None,
            context: None,
        }
    }

    pub fn result_format(mut self, result_format: ResultFormat) -> Self {
        self.result_format = Some(result_format);
        self
    }

    pub fn header(mut self, header: bool) -> Self {
        self.header = Some(header);
        self
    }

    pub fn parameters(mut self, parameters: &[DruidSqlType]) -> Self {
        self.parameters = Some(parameters.to_vec());
        self
    }

    pub fn context(mut self, context: Context) -> Self {
        self.context = Some(context);
        self
    }
}
