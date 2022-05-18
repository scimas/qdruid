use serde::{Deserialize, Serialize};

/// Query context
///
/// The intended pattern for creating the query context is to build it up from
/// an empty one.
///
/// # Examples
///
/// ```
/// # use query_druid::components::context::Context;
/// let empty_context = Context::new();
/// assert!(matches!(empty_context, Context { use_cache: None, .. }));
///
/// let use_cache_context = Context::new().use_cache(true);
/// assert!(matches!(use_cache_context, Context { use_cache: Some(true), .. }));
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "context", rename_all = "camelCase")]
pub struct Context {
    pub timeout: Option<u64>,
    pub priority: Option<u64>,
    pub lane: Option<String>,
    pub query_id: Option<String>,
    pub broker_service: Option<String>,
    pub use_cache: Option<bool>,
    pub populate_cache: Option<bool>,
    pub use_result_level_cache: Option<bool>,
    pub populate_result_level_cache: Option<bool>,
    pub by_segment: Option<bool>,
    pub finalize: Option<bool>,
    pub max_scatter_gather_bytes: Option<u64>,
    pub max_queued_bytes: Option<u64>,
    pub serialize_date_time_as_long: Option<bool>,
    pub serialize_date_time_as_long_inner: Option<bool>,
    pub enable_parallel_merge: Option<bool>,
    pub parallel_merge_parallelism: Option<u64>,
    pub parallel_merge_initial_yield_rows: Option<u64>,
    pub parallel_merge_small_batch_rows: Option<u64>,
    pub use_filter_c_n_f: Option<bool>,
    pub secondary_partition_pruning: Option<bool>,
    pub enable_join_left_table_scan_direct: Option<bool>,
    pub debug: Option<bool>,
    pub min_top_n_threshold: Option<u64>,
    pub skip_empty_buckets: Option<bool>,
    pub vectorize: Option<bool>,
    pub vector_size: Option<u64>,
    pub vectorize_virtual_columns: Option<bool>,
    pub sql_query_id: Option<String>,
    pub sql_time_zone: Option<String>,
    pub sql_stringify_arrays: Option<bool>,
    pub use_approximate_count_distinct: Option<bool>,
    pub use_grouping_set_for_exact_distinct: Option<bool>,
    pub use_approximate_top_n: Option<bool>,
}

impl Context {
    /// Create an empty query context.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the `timeout` parameter.
    pub fn timeout(mut self, t: u64) -> Self {
        self.timeout = Some(t);
        self
    }

    /// Set the `priority` parameter.
    pub fn priority(mut self, p: u64) -> Self {
        self.priority = Some(p);
        self
    }

    /// Set the `lane` parameter.
    pub fn lane(mut self, l: String) -> Self {
        self.lane = Some(l);
        self
    }

    /// Set the `queryId` parameter.
    pub fn query_id(mut self, qid: String) -> Self {
        self.query_id = Some(qid);
        self
    }

    /// Set the `brokerService` parameter.
    pub fn broker_service(mut self, bs: String) -> Self {
        self.broker_service = Some(bs);
        self
    }

    /// Set the `useCache` parameter.
    pub fn use_cache(mut self, flag: bool) -> Self {
        self.use_cache = Some(flag);
        self
    }

    /// Set the `populateCache` parameter.
    pub fn populate_cache(mut self, flag: bool) -> Self {
        self.populate_cache = Some(flag);
        self
    }

    /// Set the `useResultLevelCache` parameter.
    pub fn use_result_level_cache(mut self, flag: bool) -> Self {
        self.use_result_level_cache = Some(flag);
        self
    }

    /// Set the `populateResultLevelCache` parameter.
    pub fn populate_result_level_cache(mut self, flag: bool) -> Self {
        self.populate_result_level_cache = Some(flag);
        self
    }

    /// Set the `bySegment` parameter.
    pub fn by_segment(mut self, flag: bool) -> Self {
        self.by_segment = Some(flag);
        self
    }

    /// Set the `finalize` parameter.
    pub fn finalize(mut self, flag: bool) -> Self {
        self.finalize = Some(flag);
        self
    }

    /// Set the `maxScatterGatherBytes` parameter.
    pub fn max_scatter_gather_bytes(mut self, msgb: u64) -> Self {
        self.max_scatter_gather_bytes = Some(msgb);
        self
    }

    /// Set the `maxQueuedBytes` parameter.
    pub fn max_queued_bytes(mut self, mqb: u64) -> Self {
        self.max_queued_bytes = Some(mqb);
        self
    }

    /// Set the `serializeDateTimeAsLong` parameter.
    pub fn serialize_date_time_as_long(mut self, flag: bool) -> Self {
        self.serialize_date_time_as_long = Some(flag);
        self
    }

    /// Set the `erializeDateTimeAsLongInner` parameter.
    pub fn serialize_date_time_as_long_inner(mut self, flag: bool) -> Self {
        self.serialize_date_time_as_long_inner = Some(flag);
        self
    }

    /// Set the `enableParallelMerge` parameter.
    pub fn enable_parallel_merge(mut self, flag: bool) -> Self {
        self.enable_parallel_merge = Some(flag);
        self
    }

    /// Set the `paralleMergeParallelism` parameter.
    pub fn parallel_merge_parallelism(mut self, pmp: u64) -> Self {
        self.parallel_merge_parallelism = Some(pmp);
        self
    }

    /// Set the `parallelMergeInitialYieldRows` parameter.
    pub fn parallel_merge_initial_yield_rows(mut self, pmiyr: u64) -> Self {
        self.parallel_merge_initial_yield_rows = Some(pmiyr);
        self
    }

    /// Set the `parallelMergeSmallBatchRows` parameter.
    pub fn parallel_merge_small_batch_rows(mut self, pmsbr: u64) -> Self {
        self.parallel_merge_small_batch_rows = Some(pmsbr);
        self
    }

    /// Set the `useFilterCNF` parameter.
    pub fn use_filter_c_n_f(mut self, flag: bool) -> Self {
        self.use_filter_c_n_f = Some(flag);
        self
    }

    /// Set the `secondaryPartitionPruning` parameter.
    pub fn secondary_partition_pruning(mut self, flag: bool) -> Self {
        self.secondary_partition_pruning = Some(flag);
        self
    }

    /// Set the `enableJoinLeftTableScanDirect` parameter.
    pub fn enable_join_left_table_scan_direct(mut self, flag: bool) -> Self {
        self.enable_join_left_table_scan_direct = Some(flag);
        self
    }

    /// Set the `debug` parameter.
    pub fn debug(mut self, flag: bool) -> Self {
        self.debug = Some(flag);
        self
    }

    /// Set the `minTopNThreshold` parameter.
    pub fn min_top_n_threshold(mut self, mtnt: u64) -> Self {
        self.min_top_n_threshold = Some(mtnt);
        self
    }

    /// Set the `skipEmptyBuckets` parameter.
    pub fn skip_empty_buckets(mut self, flag: bool) -> Self {
        self.skip_empty_buckets = Some(flag);
        self
    }

    /// Set the `vectorize` parameter.
    pub fn vectorize(mut self, flag: bool) -> Self {
        self.vectorize = Some(flag);
        self
    }

    /// Set the `vectorSize` parameter.
    pub fn vector_size(mut self, vs: u64) -> Self {
        self.vector_size = Some(vs);
        self
    }

    /// Set the `vectorizeVirtualColumns` parameter.
    pub fn vectorize_virtual_columns(mut self, flag: bool) -> Self {
        self.vectorize_virtual_columns = Some(flag);
        self
    }

    /// Set the `sqlQueryId` parameter.
    pub fn sql_query_id(mut self, sqid: String) -> Self {
        self.sql_query_id = Some(sqid);
        self
    }

    /// Set the `sqlTimeZone` parameter.
    pub fn sql_time_zone(mut self, stz: String) -> Self {
        self.sql_time_zone = Some(stz);
        self
    }

    /// Set the `sqlStringifyArrays` parameter.
    pub fn sql_stringify_arrays(mut self, flag: bool) -> Self {
        self.sql_stringify_arrays = Some(flag);
        self
    }

    /// Set the `useApproximateCountDistinct` parameter.
    pub fn use_approximate_count_distinct(mut self, flag: bool) -> Self {
        self.use_approximate_count_distinct = Some(flag);
        self
    }

    /// Set the `useGroupingSetForExactDistinct` parameter.
    pub fn use_grouping_set_for_exact_distinct(mut self, flag: bool) -> Self {
        self.use_grouping_set_for_exact_distinct = Some(flag);
        self
    }

    /// Set the `useApproximateTopN` parameter.
    pub fn use_approximate_top_n(mut self, flag: bool) -> Self {
        self.use_approximate_top_n = Some(flag);
        self
    }
}
