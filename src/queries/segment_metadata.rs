use serde::{Deserialize, Serialize};

use crate::components::{
    context::Context, data_sources::DataSource, intervals::Interval, to_include::ToInclude,
    virtual_columns::VirtaulColumn,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentMetadata {
    query_type: String,
    data_source: DataSource,
    intervals: Option<Vec<Interval>>,
    to_include: Option<ToInclude>,
    merge: Option<bool>,
    analysis_types: Option<Vec<String>>,
    lenient_aggregator_merge: Option<bool>,
    virtual_columns: Option<Vec<VirtaulColumn>>,
    context: Option<Context>,
}

impl SegmentMetadata {
    pub fn new(data_source: DataSource) -> Self {
        Self {
            query_type: "segmentMetadata".into(),
            data_source,
            intervals: None,
            to_include: None,
            merge: None,
            analysis_types: None,
            lenient_aggregator_merge: None,
            virtual_columns: None,
            context: None,
        }
    }

    pub fn intervals(&mut self, intervals: &[Interval]) -> &mut Self {
        self.intervals = Some(intervals.to_vec());
        self
    }

    pub fn to_include(&mut self, to_include: ToInclude) -> &mut Self {
        self.to_include = Some(to_include);
        self
    }

    pub fn merge(&mut self, merge: bool) -> &mut Self {
        self.merge = Some(merge);
        self
    }

    pub fn analysis_types(&mut self, analysis_types: &[String]) -> &mut Self {
        self.analysis_types = Some(analysis_types.to_vec());
        self
    }

    pub fn lenient_aggregator_merge(&mut self, lenient_aggregator_merge: bool) -> &mut Self {
        self.lenient_aggregator_merge = Some(lenient_aggregator_merge);
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
