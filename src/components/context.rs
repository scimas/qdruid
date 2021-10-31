use super::druid_types::DruidType;

#[derive(Default)]
pub struct Context {
    timeout: Option<DruidType>,
    priority: Option<DruidType>,
    lane: Option<DruidType>,
    query_id: Option<DruidType>,
    broker_service: Option<DruidType>,
    use_cache: Option<bool>,
    populate_cache: Option<bool>,
    use_result_level_cache: Option<bool>,
    populate_result_level_cache: Option<bool>,
    by_segment: Option<bool>,
    finalize: Option<bool>,
    max_scatter_gather_bytes: Option<DruidType>,
    max_queued_bytes: Option<DruidType>,
    serialize_date_time_as_long: Option<bool>,
    serialize_date_time_as_long_inner: Option<bool>,
    enable_parallel_merge: Option<bool>,
    parallel_merge_parallelism: Option<DruidType>,
    parallel_merge_initial_yield_rows: Option<DruidType>,
    parallel_merge_small_batch_rows: Option<DruidType>,
    use_filter_c_n_f: Option<bool>,
    secondary_partition_pruning: Option<bool>,
    enable_join_left_table_scan_direct: Option<bool>,
    debug: Option<bool>,
    min_ton_n_threshold: Option<DruidType>,
    skip_empty_buckets: Option<bool>,
    vectorize: Option<bool>,
    vector_size: Option<DruidType>,
    vectorize_virtual_columns: Option<bool>,
}

impl Context {
    fn new() -> Self {
        Default::default()
    }

    fn timeout(&mut self, t: DruidType) -> &mut Self {
        self.timeout = Some(t);
        self
    }

    fn priority(&mut self, p: DruidType) -> &mut Self {
        self.priority = Some(p);
        self
    }

    fn lane(&mut self, l: DruidType) -> &mut Self {
        self.lane = Some(l);
        self
    }

    fn query_id(&mut self, qid: DruidType) -> &mut Self {
        self.query_id = Some(qid);
        self
    }

    fn broker_service(&mut self, bs: DruidType) -> &mut Self {
        self.broker_service = Some(bs);
        self
    }

    fn use_cache(&mut self, flag: bool) -> &mut Self {
        self.use_cache = Some(flag);
        self
    }

    fn populate_cache(&mut self, flag: bool) -> &mut Self {
        self.populate_cache = Some(flag);
        self
    }

    fn use_result_level_cache(&mut self, flag: bool) -> &mut Self {
        self.use_result_level_cache = Some(flag);
        self
    }

    fn populate_result_level_cache(&mut self, flag: bool) -> &mut Self {
        self.populate_result_level_cache = Some(flag);
        self
    }

    fn by_segment(&mut self, flag: bool) -> &mut Self {
        self.by_segment = Some(flag);
        self
    }

    fn finalize(&mut self, flag: bool) -> &mut Self {
        self.finalize = Some(flag);
        self
    }

    fn max_scatter_gather_bytes(&mut self, msgb: DruidType) -> &mut Self {
        self.max_scatter_gather_bytes = Some(msgb);
        self
    }

    fn max_queued_bytes(&mut self, mqb: DruidType) -> &mut Self {
        self.max_queued_bytes = Some(mqb);
        self
    }

    fn serialize_date_time_as_long(&mut self, flag: bool) -> &mut Self {
        self.serialize_date_time_as_long = Some(flag);
        self
    }

    fn serialize_date_time_as_long_inner(&mut self, flag: bool) -> &mut Self {
        self.serialize_date_time_as_long_inner = Some(flag);
        self
    }

    fn enable_parallel_merge(&mut self, flag: bool) -> &mut Self {
        self.enable_parallel_merge = Some(flag);
        self
    }

    fn parallel_merge_parallelism(&mut self, pmp: DruidType) -> &mut Self {
        self.parallel_merge_parallelism = Some(pmp);
        self
    }

    fn parallel_merge_initial_yield_rows(&mut self, pmiyr: DruidType) -> &mut Self {
        self.parallel_merge_initial_yield_rows = Some(pmiyr);
        self
    }

    fn parallel_merge_small_batch_rows(&mut self, pmsbr: DruidType) -> &mut Self {
        self.parallel_merge_small_batch_rows = Some(pmsbr);
        self
    }

    fn use_filter_c_n_f(&mut self, flag: bool) -> &mut Self {
        self.use_filter_c_n_f = Some(flag);
        self
    }

    fn secondary_partition_pruning(&mut self, flag: bool) -> &mut Self {
        self.secondary_partition_pruning = Some(flag);
        self
    }

    fn enable_join_left_table_scan_direct(&mut self, flag: bool) -> &mut Self {
        self.enable_join_left_table_scan_direct = Some(flag);
        self
    }

    fn debug(&mut self, flag: bool) -> &mut Self {
        self.debug = Some(flag);
        self
    }

    fn min_ton_n_threshold(&mut self, mtnt: DruidType) -> &mut Self {
        self.min_ton_n_threshold = Some(mtnt);
        self
    }

    fn skip_empty_buckets(&mut self, flag: bool) -> &mut Self {
        self.skip_empty_buckets = Some(flag);
        self
    }

    fn vectorize(&mut self, flag: bool) -> &mut Self {
        self.vectorize = Some(flag);
        self
    }

    fn vector_size(&mut self, vs: DruidType) -> &mut Self {
        self.vector_size = Some(vs);
        self
    }

    fn vectorize_virtual_columns(&mut self, flag: bool) -> &mut Self {
        self.vectorize_virtual_columns = Some(flag);
        self
    }
}
