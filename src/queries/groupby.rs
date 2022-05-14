use serde::{Deserialize, Serialize};

use crate::components::{
    aggregations::Aggregator, context::Context, data_sources::DataSource,
    dimension_specs::DimensionSpec, filters::Filter, granularities::Granularity,
    having_specs::HavingSpec, intervals::Interval, limit_specs::LimitSpec,
    post_aggregations::PostAggregator, virtual_columns::VirtaulColumn,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupBy {
    query_type: String,
    data_source: DataSource,
    intervals: Vec<Interval>,
    granularity: Granularity,
    dimensions: Vec<DimensionSpec>,
    limit_spec: Option<LimitSpec>,
    having: Option<HavingSpec>,
    filter: Option<Filter>,
    aggregations: Option<Vec<Aggregator>>,
    post_aggregations: Option<Vec<PostAggregator>>,
    subtotals_spec: Option<Vec<Vec<String>>>,
    virtual_columns: Option<Vec<VirtaulColumn>>,
    context: Option<Context>,
}

impl GroupBy {
    pub fn new(
        data_source: DataSource,
        intervals: &[Interval],
        granularity: Granularity,
        dimensions: &[DimensionSpec],
    ) -> Self {
        Self {
            query_type: "groupBy".into(),
            data_source,
            intervals: intervals.to_vec(),
            granularity,
            dimensions: dimensions.to_vec(),
            limit_spec: None,
            having: None,
            filter: None,
            aggregations: None,
            post_aggregations: None,
            subtotals_spec: None,
            virtual_columns: None,
            context: None,
        }
    }

    pub fn limit_spec(mut self, limit_spec: LimitSpec) -> Self {
        self.limit_spec = Some(limit_spec);
        self
    }

    pub fn having(mut self, having: HavingSpec) -> Self {
        self.having = Some(having);
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

    pub fn subtotals_spec(mut self, subtotals_spec: &[Vec<String>]) -> Self {
        self.subtotals_spec = Some(subtotals_spec.to_vec());
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
