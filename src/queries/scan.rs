use serde::{Deserialize, Serialize};

use crate::components::{
    context::Context, data_sources::DataSource, filters::Filter, intervals::Interval,
    virtual_columns::VirtaulColumn,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scan {
    query_type: String,
    data_source: DataSource,
    intervals: Vec<Interval>,
    result_format: Option<String>,
    filter: Option<Filter>,
    columns: Option<Vec<String>>,
    batch_size: Option<usize>,
    limit: Option<usize>,
    offset: Option<usize>,
    order: Option<String>,
    legacy: Option<bool>,
    virtual_columns: Option<Vec<VirtaulColumn>>,
    context: Option<Context>,
}

impl Scan {
    pub fn new(data_source: DataSource, intervals: &[Interval]) -> Self {
        Self {
            query_type: "scan".into(),
            data_source,
            intervals: intervals.to_vec(),
            result_format: None,
            filter: None,
            columns: None,
            batch_size: None,
            limit: None,
            offset: None,
            order: None,
            legacy: None,
            virtual_columns: None,
            context: None,
        }
    }

    pub fn result_format(&mut self, result_format: String) -> &mut Self {
        self.result_format = Some(result_format);
        self
    }

    pub fn filter(&mut self, filter: Filter) -> &mut Self {
        self.filter = Some(filter);
        self
    }

    pub fn columns(&mut self, columns: &[String]) -> &mut Self {
        self.columns = Some(columns.to_vec());
        self
    }

    pub fn batch_size(&mut self, batch_size: usize) -> &mut Self {
        self.batch_size = Some(batch_size);
        self
    }

    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(&mut self, offset: usize) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    pub fn order(&mut self, order: &str) -> &mut Self {
        self.order = Some(order.into());
        self
    }

    pub fn legacy(&mut self, legacy: bool) -> &mut Self {
        self.legacy = Some(legacy);
        self
    }

    pub fn virtual_columns(&mut self, virtual_columns: &[VirtaulColumn]) -> &mut Self {
        self.virtual_columns = Some(virtual_columns.to_vec());
        self
    }

    pub fn context(&mut self, context: Context) -> &mut Self {
        self.context = Some(context);
        self
    }
}
