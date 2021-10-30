use super::druid_types::DruidType;

pub enum PostAggregator {
    Arithmetic {
        name: String,
        func: String,
        fields: Vec<PostAggregator>,
        ordering: Option<String>,
    },
    FieldAccess {
        name: String,
        field_name: String,
    },
    FinalizingFieldAccess {
        name: String,
        field_name: String,
    },
    Constant {
        name: String,
        value: DruidType,   // must be numeric
    },
    LongGreatest {
        name: String,
        fields: Vec<PostAggregator>,
    },
    DoubleGreatest {
        name: String,
        fields: Vec<PostAggregator>,
    },
    LongLeast {
        name: String,
        fields: Vec<PostAggregator>,
    },
    DoubleLeast {
        name: String,
        fields: Vec<PostAggregator>,
    },
    Javascript {
        name: String,
        field_names: Vec<String>,
        function: String,
    },
    HyperUniqueCardinality {
        name: String,
        field_name: String,
    }
}
