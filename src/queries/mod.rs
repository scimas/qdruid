use self::{
    datasource_metadata::DataSourceMetadata, groupby::GroupBy, scan::Scan, search::Search,
    segment_metadata::SegmentMetadata, sql::Sql, time_boundary::TimeBoundary,
    timeseries::Timeseries, topn::TopN,
};

pub mod datasource_metadata;
pub mod groupby;
pub mod scan;
pub mod search;
pub mod segment_metadata;
pub mod sql;
pub mod time_boundary;
pub mod timeseries;
pub mod topn;

#[derive(Debug, Clone)]
pub enum Query {
    Timeseries(Timeseries),
    TopN(TopN),
    GroupBy(GroupBy),
    Scan(Scan),
    Search(Search),
    TimeBoundary(TimeBoundary),
    SegmentMetadata(SegmentMetadata),
    DataSourceMetadata(DataSourceMetadata),
    Sql(Sql),
}

impl From<Timeseries> for Query {
    fn from(timeseries_query: Timeseries) -> Self {
        Self::Timeseries(timeseries_query)
    }
}

impl From<TopN> for Query {
    fn from(topn_query: TopN) -> Self {
        Self::TopN(topn_query)
    }
}

impl From<GroupBy> for Query {
    fn from(groupby_query: GroupBy) -> Self {
        Self::GroupBy(groupby_query)
    }
}

impl From<Scan> for Query {
    fn from(scan_query: Scan) -> Self {
        Self::Scan(scan_query)
    }
}

impl From<Search> for Query {
    fn from(search_query: Search) -> Self {
        Self::Search(search_query)
    }
}

impl From<TimeBoundary> for Query {
    fn from(timeboundary_query: TimeBoundary) -> Self {
        Self::TimeBoundary(timeboundary_query)
    }
}

impl From<SegmentMetadata> for Query {
    fn from(segment_metadata_query: SegmentMetadata) -> Self {
        Self::SegmentMetadata(segment_metadata_query)
    }
}

impl From<DataSourceMetadata> for Query {
    fn from(datasource_metadata_query: DataSourceMetadata) -> Self {
        Self::DataSourceMetadata(datasource_metadata_query)
    }
}

impl From<Sql> for Query {
    fn from(sql_query: Sql) -> Self {
        Self::Sql(sql_query)
    }
}