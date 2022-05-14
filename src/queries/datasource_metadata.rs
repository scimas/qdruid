use serde::{Deserialize, Serialize};

use crate::components::{
    context::Context, data_sources::DataSource, virtual_columns::VirtaulColumn,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceMetadata {
    query_type: String,
    data_source: DataSource,
    virtual_columns: Option<Vec<VirtaulColumn>>,
    context: Option<Context>,
}

impl DataSourceMetadata {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            query_type: "dataSourceMetadata".into(),
            data_source,
            virtual_columns: None,
            context: None,
        }
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
