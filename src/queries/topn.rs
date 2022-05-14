use serde::{Deserialize, Serialize};

use crate::components::{
    aggregations::Aggregator, context::Context, data_sources::DataSource,
    dimension_specs::DimensionSpec, filters::Filter, granularities::Granularity,
    intervals::Interval, post_aggregations::PostAggregator, topn_metric_specs::TopNMetricSpec,
    virtual_columns::VirtaulColumn,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopN {
    query_type: String,
    data_source: DataSource,
    intervals: Vec<Interval>,
    granularity: Granularity,
    dimension: DimensionSpec,
    threshold: usize,
    metric: TopNMetricSpec,
    filter: Option<Filter>,
    aggregations: Option<Vec<Aggregator>>,
    post_aggregations: Option<Vec<PostAggregator>>,
    virtual_columns: Option<Vec<VirtaulColumn>>,
    context: Option<Context>,
}

impl TopN {
    pub fn new(
        data_source: DataSource,
        intervals: &[Interval],
        granularity: Granularity,
        dimension: DimensionSpec,
        threshold: usize,
        metric: TopNMetricSpec,
    ) -> Self {
        Self {
            query_type: "topN".into(),
            data_source,
            intervals: intervals.to_vec(),
            granularity,
            dimension,
            threshold,
            metric,
            filter: None,
            aggregations: None,
            post_aggregations: None,
            virtual_columns: None,
            context: None,
        }
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

    pub fn virtual_columns(mut self, virtual_columns: &[VirtaulColumn]) -> Self {
        self.virtual_columns = Some(virtual_columns.to_vec());
        self
    }

    pub fn context(mut self, context: Context) -> Self {
        self.context = Some(context);
        self
    }
}
