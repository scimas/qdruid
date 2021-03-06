use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::components::{
    druid_types::DruidNativeType, granularities::Granularity, intervals::Interval,
};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[serde(rename_all = "camelCase")]
#[error("query error: {error_message}")]
pub struct QueryError {
    pub error: String,
    pub error_message: String,
    pub error_class: String,
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeseriesResult {
    pub timestamp: Option<DateTime<Utc>>,
    pub result: HashMap<String, DruidNativeType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopNResult {
    pub timestamp: DateTime<Utc>,
    pub result: Vec<HashMap<String, DruidNativeType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupByResult {
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub event: HashMap<String, DruidNativeType>,
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
    pub timestamp: DateTime<Utc>,
    pub result: Vec<HashMap<String, DruidNativeType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeBoundaryResult {
    pub timestamp: DateTime<Utc>,
    pub result: TimeBoundaryTimes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeBoundaryTimes {
    pub min_time: Option<DateTime<Utc>>,
    pub max_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentMetadataResult {
    pub id: String,
    pub intervals: Vec<Interval>,
    pub columns: HashMap<String, HashMap<String, DruidNativeType>>,
    pub aggregators: HashMap<String, HashMap<String, DruidNativeType>>,
    pub query_granularity: Granularity,
    pub size: usize,
    pub num_rows: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceMetadataResult {
    pub timestamp: DateTime<Utc>,
    pub result: DataSourceMetadataTimes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceMetadataTimes {
    pub max_ingested_event_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum SqlResult {
    Object(HashMap<String, DruidNativeType>),
    Array(Vec<DruidNativeType>),
    Csv(String),
}
