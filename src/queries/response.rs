use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{Granularity, Interval, DruidNativeType};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DruidResponse {
    QueryError {
        error: String,
        error_message: String,
        error_class: String,
        host: String,
    },
    Timeseries(Vec<TimeseriesResult>),
    TopN(Vec<TopNResult>),
    GroupBy(Vec<GroupByResult>),
    Scan(Vec<ScanResult>),
    Search(Vec<SearchResult>),
    TimeBoundary(Vec<TimeBoundaryResult>),
    SegmentMetadata(Vec<SegmentMetadataResult>),
    DataSourceMetadata(Vec<DataSourceMetadataResult>),
    Sql(Vec<SqlResult>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeseriesResult {
    timestamp: Option<DateTime<Utc>>,
    result: HashMap<String, DruidNativeType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopNResult {
    timestamp: DateTime<Utc>,
    result: Vec<HashMap<String, DruidNativeType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupByResult {
    timestamp: DateTime<Utc>,
    version: String,
    event: HashMap<String, DruidNativeType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum ScanResult {
    List {
        segment_id: String,
        columns: Vec<String>,
        events: Vec<HashMap<String, DruidNativeType>>,
    },
    CompactedList {
        segment_id: String,
        columns: Vec<String>,
        events: Vec<Vec<DruidNativeType>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    timestamp: DateTime<Utc>,
    result: Vec<HashMap<String, DruidNativeType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeBoundaryResult {
    timestamp: DateTime<Utc>,
    result: TimeBoundaryTimes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeBoundaryTimes {
    min_time: Option<DateTime<Utc>>,
    max_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentMetadataResult {
    id: String,
    intervals: Vec<Interval>,
    columns: HashMap<String, HashMap<String, DruidNativeType>>,
    aggregators: HashMap<String, HashMap<String, DruidNativeType>>,
    query_granularity: Granularity,
    size: usize,
    num_rows: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceMetadataResult {
    timestamp: DateTime<Utc>,
    result: DataSourceMetadataTimes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceMetadataTimes {
    max_ingested_event_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum SqlResult {
    Object(HashMap<String, DruidNativeType>),
    Array(Vec<DruidNativeType>),
    Csv(String),
}
