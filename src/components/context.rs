use serde::{Deserialize, Serialize};

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
    pub fn new() -> Self {
        Default::default()
    }

    pub fn timeout(&mut self, t: u64) -> &mut Self {
        self.timeout = Some(t);
        self
    }

    pub fn priority(&mut self, p: u64) -> &mut Self {
        self.priority = Some(p);
        self
    }

    pub fn lane(&mut self, l: String) -> &mut Self {
        self.lane = Some(l);
        self
    }

    pub fn query_id(&mut self, qid: String) -> &mut Self {
        self.query_id = Some(qid);
        self
    }

    pub fn broker_service(&mut self, bs: String) -> &mut Self {
        self.broker_service = Some(bs);
        self
    }

    pub fn use_cache(&mut self, flag: bool) -> &mut Self {
        self.use_cache = Some(flag);
        self
    }

    pub fn populate_cache(&mut self, flag: bool) -> &mut Self {
        self.populate_cache = Some(flag);
        self
    }

    pub fn use_result_level_cache(&mut self, flag: bool) -> &mut Self {
        self.use_result_level_cache = Some(flag);
        self
    }

    pub fn populate_result_level_cache(&mut self, flag: bool) -> &mut Self {
        self.populate_result_level_cache = Some(flag);
        self
    }

    pub fn by_segment(&mut self, flag: bool) -> &mut Self {
        self.by_segment = Some(flag);
        self
    }

    pub fn finalize(&mut self, flag: bool) -> &mut Self {
        self.finalize = Some(flag);
        self
    }

    pub fn max_scatter_gather_bytes(&mut self, msgb: u64) -> &mut Self {
        self.max_scatter_gather_bytes = Some(msgb);
        self
    }

    pub fn max_queued_bytes(&mut self, mqb: u64) -> &mut Self {
        self.max_queued_bytes = Some(mqb);
        self
    }

    pub fn serialize_date_time_as_long(&mut self, flag: bool) -> &mut Self {
        self.serialize_date_time_as_long = Some(flag);
        self
    }

    pub fn serialize_date_time_as_long_inner(&mut self, flag: bool) -> &mut Self {
        self.serialize_date_time_as_long_inner = Some(flag);
        self
    }

    pub fn enable_parallel_merge(&mut self, flag: bool) -> &mut Self {
        self.enable_parallel_merge = Some(flag);
        self
    }

    pub fn parallel_merge_parallelism(&mut self, pmp: u64) -> &mut Self {
        self.parallel_merge_parallelism = Some(pmp);
        self
    }

    pub fn parallel_merge_initial_yield_rows(&mut self, pmiyr: u64) -> &mut Self {
        self.parallel_merge_initial_yield_rows = Some(pmiyr);
        self
    }

    pub fn parallel_merge_small_batch_rows(&mut self, pmsbr: u64) -> &mut Self {
        self.parallel_merge_small_batch_rows = Some(pmsbr);
        self
    }

    pub fn use_filter_c_n_f(&mut self, flag: bool) -> &mut Self {
        self.use_filter_c_n_f = Some(flag);
        self
    }

    pub fn secondary_partition_pruning(&mut self, flag: bool) -> &mut Self {
        self.secondary_partition_pruning = Some(flag);
        self
    }

    pub fn enable_join_left_table_scan_direct(&mut self, flag: bool) -> &mut Self {
        self.enable_join_left_table_scan_direct = Some(flag);
        self
    }

    pub fn debug(&mut self, flag: bool) -> &mut Self {
        self.debug = Some(flag);
        self
    }

    pub fn min_top_n_threshold(&mut self, mtnt: u64) -> &mut Self {
        self.min_top_n_threshold = Some(mtnt);
        self
    }

    pub fn skip_empty_buckets(&mut self, flag: bool) -> &mut Self {
        self.skip_empty_buckets = Some(flag);
        self
    }

    pub fn vectorize(&mut self, flag: bool) -> &mut Self {
        self.vectorize = Some(flag);
        self
    }

    pub fn vector_size(&mut self, vs: u64) -> &mut Self {
        self.vector_size = Some(vs);
        self
    }

    pub fn vectorize_virtual_columns(&mut self, flag: bool) -> &mut Self {
        self.vectorize_virtual_columns = Some(flag);
        self
    }

    pub fn sql_query_id(&mut self, sqid: String) -> &mut Self {
        self.sql_query_id = Some(sqid);
        self
    }

    pub fn sql_time_zone(&mut self, stz: String) -> &mut Self {
        self.sql_time_zone = Some(stz);
        self
    }

    pub fn sql_stringify_arrays(&mut self, flag: bool) -> &mut Self {
        self.sql_stringify_arrays = Some(flag);
        self
    }

    pub fn use_approximate_count_distinct(&mut self, flag: bool) -> &mut Self {
        self.use_approximate_count_distinct = Some(flag);
        self
    }

    pub fn use_grouping_set_for_exact_distinct(&mut self, flag: bool) -> &mut Self {
        self.use_grouping_set_for_exact_distinct = Some(flag);
        self
    }

    pub fn use_approximate_top_n(&mut self, flag: bool) -> &mut Self {
        self.use_approximate_top_n = Some(flag);
        self
    }
}
