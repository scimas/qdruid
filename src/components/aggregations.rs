use super::filters::Filter;

pub enum Aggregation {
    Count {
        name: String,
    },
    LongSum {
        name: String,
        field_name: String,
    },
    DoubleSum {
        name: String,
        field_name: String,
    },
    FloatSum {
        name: String,
        field_name: String,
    },
    LongMin {
        name: String,
        field_name: String,
    },
    DoubleMin {
        name: String,
        field_name: String,
    },
    FloatMin {
        name: String,
        field_name: String,
    },
    LongMax {
        name: String,
        field_name: String,
    },
    DoubleMax {
        name: String,
        field_name: String,
    },
    FloatMax {
        name: String,
        field_name: String,
    },
    LongFirst {
        name: String,
        field_name: String,
    },
    DoubleFirst {
        name: String,
        field_name: String,
    },
    FloatFirst {
        name: String,
        field_name: String,
    },
    StringFirst {
        name: String,
        field_name: String,
    },
    LongLast {
        name: String,
        field_name: String,
    },
    DoubleLast {
        name: String,
        field_name: String,
    },
    FloatLast {
        name: String,
        field_name: String,
    },
    StringLast {
        name: String,
        field_name: String,
    },
    LongAny {
        name: String,
        field_name: String,
    },
    DoubleAny {
        name: String,
        field_name: String,
    },
    FloatAny {
        name: String,
        field_name: String,
    },
    StringAny {
        name: String,
        field_name: String,
    },
    Javascript {
        name: String,
        field_names: Vec<String>,
        fn_aggregate: String,
        fn_combine: String,
        fn_reset: String,
    },
    Filtered {
        filter: Filter,
        aggregator: Box<Aggregation>,
    },
    Grouping {
        name: String,
        groupings: Vec<String>,
    },
}
