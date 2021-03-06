use serde::{Deserialize, Serialize};

use crate::components::{
    context::Context, data_sources::DataSource, filters::Filter, virtual_columns::VirtaulColumn,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeBoundary {
    query_type: String,
    data_source: DataSource,
    bound: Option<String>,
    filter: Option<Filter>,
    virtual_columns: Option<Vec<VirtaulColumn>>,
    context: Option<Context>,
}

impl TimeBoundary {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            query_type: "timeBoundary".into(),
            data_source,
            bound: None,
            filter: None,
            virtual_columns: None,
            context: None,
        }
    }

    pub fn bound(mut self, bound: &str) -> Self {
        self.bound = Some(bound.into());
        self
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn virtual_columns(mut self, virtual_columns: &[VirtaulColumn]) -> Self {
        self.virtual_columns = Some(virtual_columns.to_vec());
        self
    }

    pub fn context(mut self, context: Context) -> Self {
        self.context = Some(context);
        self
    }
}
