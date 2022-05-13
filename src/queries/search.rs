use serde::{Deserialize, Serialize};

use crate::components::{
    context::Context, data_sources::DataSource, filters::Filter, granularities::Granularity,
    intervals::Interval, search_query_specs::SearchQuerySpec, virtual_columns::VirtaulColumn,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    query_type: String,
    data_source: DataSource,
    intervals: Vec<Interval>,
    query: SearchQuerySpec,
    granularity: Option<Granularity>,
    filter: Option<Filter>,
    limit: Option<usize>,
    search_dimensions: Option<Vec<String>>,
    sort: Option<String>,
    virtual_columns: Option<Vec<VirtaulColumn>>,
    context: Option<Context>,
}

impl Search {
    pub fn new(data_source: DataSource, intervals: &[Interval], query: SearchQuerySpec) -> Self {
        Self {
            query_type: "search".into(),
            data_source,
            intervals: intervals.to_vec(),
            query,
            granularity: None,
            filter: None,
            limit: None,
            search_dimensions: None,
            sort: None,
            virtual_columns: None,
            context: None,
        }
    }

    pub fn granularity(&mut self, granularity: Granularity) -> &mut Self {
        self.granularity = Some(granularity);
        self
    }

    pub fn filter(&mut self, filter: Filter) -> &mut Self {
        self.filter = Some(filter);
        self
    }

    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn search_dimensions(&mut self, search_dimensions: &[String]) -> &mut Self {
        self.search_dimensions = Some(search_dimensions.to_vec());
        self
    }

    pub fn sort(&mut self, sort: &str) -> &mut Self {
        self.sort = Some(sort.into());
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
