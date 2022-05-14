use serde::{Deserialize, Serialize};

use crate::components::{
    aggregations::Aggregator, context::Context, data_sources::DataSource, filters::Filter,
    granularities::Granularity, intervals::Interval, post_aggregations::PostAggregator,
    virtual_columns::VirtaulColumn,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timeseries {
    query_type: String,
    data_source: DataSource,
    intervals: Vec<Interval>,
    granularity: Granularity,
    descending: Option<bool>,
    filter: Option<Filter>,
    aggregations: Option<Vec<Aggregator>>,
    post_aggregations: Option<Vec<PostAggregator>>,
    limit: Option<usize>,
    virtual_columns: Option<Vec<VirtaulColumn>>,
    context: Option<Context>,
}

impl Timeseries {
    pub fn new(data_source: DataSource, intervals: &[Interval], granularity: Granularity) -> Self {
        Self {
            query_type: "timeseries".into(),
            data_source,
            intervals: intervals.to_vec(),
            granularity,
            descending: None,
            filter: None,
            aggregations: None,
            post_aggregations: None,
            limit: None,
            virtual_columns: None,
            context: None,
        }
    }

    pub fn descending(mut self, descending: bool) -> Self {
        self.descending = Some(descending);
        self
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn aggregations(mut self, aggregations: &[Aggregator]) -> Self {
        self.aggregations = Some(aggregations.to_vec());
        self
    }

    pub fn post_aggregations(mut self, post_aggregations: &[PostAggregator]) -> Self {
        self.post_aggregations = Some(post_aggregations.to_vec());
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
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
