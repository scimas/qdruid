pub enum LimitSpec {
    Default {
        limit: Option<usize>,
        offset: Option<usize>,
        columns: Option<Vec<LimitSpec>>,
    },
    OrderByColumnSpec {
        dimension: String,
        direction: String,
        dimension_order: Option<String>,
    },
}
