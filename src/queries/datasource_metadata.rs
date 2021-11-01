use crate::components::{
    context::Context, data_sources::DataSource, virtual_columns::VirtaulColumn,
};

#[derive(Debug, Clone)]
pub struct DataSourceMetadata {
    query_type: String,
    data_source: DataSource,
    virtual_columns: Option<Vec<VirtaulColumn>>,
    context: Option<Context>,
}

impl DataSourceMetadata {
    fn new(data_source: DataSource) -> Self {
        Self {
            query_type: "dataSourceMetadata".into(),
            data_source,
            virtual_columns: None,
            context: None,
        }
    }

    fn virtual_columns(&mut self, virtual_columns: &[VirtaulColumn]) -> &mut Self {
        self.virtual_columns = Some(virtual_columns.to_vec());
        self
    }

    fn context(&mut self, context: Context) -> &mut Self {
        self.context = Some(context);
        self
    }
}
