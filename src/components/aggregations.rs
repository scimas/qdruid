use super::{dimension_specs::DimensionSpec, filters::Filter};

#[derive(Debug, Clone)]
pub enum Aggregator {
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
        aggregator: Box<Aggregator>,
    },
    Grouping {
        name: String,
        groupings: Vec<String>,
    },
    Cardinality {
        name: String,
        fields: Vec<DimensionSpec>,
        by_row: Option<bool>,
        round: Option<bool>,
    },
    HyperUnique {
        name: String,
        field_name: String,
        is_input_hyper_unique: Option<bool>,
        round: Option<bool>,
    },
}

impl Aggregator {
    pub fn count(name: String) -> Self {
        Self::Count { name }
    }

    pub fn long_sum(name: String, field_name: String) -> Self {
        Self::LongSum { name, field_name }
    }

    pub fn double_sum(name: String, field_name: String) -> Self {
        Self::DoubleSum { name, field_name }
    }

    pub fn float_sum(name: String, field_name: String) -> Self {
        Self::FloatSum { name, field_name }
    }

    pub fn long_min(name: String, field_name: String) -> Self {
        Self::LongMin { name, field_name }
    }

    pub fn double_min(name: String, field_name: String) -> Self {
        Self::DoubleMin { name, field_name }
    }

    pub fn float_min(name: String, field_name: String) -> Self {
        Self::FloatMin { name, field_name }
    }

    pub fn long_max(name: String, field_name: String) -> Self {
        Self::LongMax { name, field_name }
    }

    pub fn double_max(name: String, field_name: String) -> Self {
        Self::DoubleMax { name, field_name }
    }

    pub fn float_max(name: String, field_name: String) -> Self {
        Self::FloatMax { name, field_name }
    }

    pub fn long_first(name: String, field_name: String) -> Self {
        Self::LongFirst { name, field_name }
    }

    pub fn double_first(name: String, field_name: String) -> Self {
        Self::DoubleFirst { name, field_name }
    }

    pub fn float_first(name: String, field_name: String) -> Self {
        Self::FloatFirst { name, field_name }
    }

    pub fn string_first(name: String, field_name: String) -> Self {
        Self::StringFirst { name, field_name }
    }

    pub fn long_last(name: String, field_name: String) -> Self {
        Self::LongLast { name, field_name }
    }

    pub fn double_last(name: String, field_name: String) -> Self {
        Self::DoubleLast { name, field_name }
    }

    pub fn float_last(name: String, field_name: String) -> Self {
        Self::FloatLast { name, field_name }
    }

    pub fn string_last(name: String, field_name: String) -> Self {
        Self::StringLast { name, field_name }
    }

    pub fn long_any(name: String, field_name: String) -> Self {
        Self::LongAny { name, field_name }
    }

    pub fn double_any(name: String, field_name: String) -> Self {
        Self::DoubleAny { name, field_name }
    }

    pub fn float_any(name: String, field_name: String) -> Self {
        Self::FloatAny { name, field_name }
    }

    pub fn string_any(name: String, field_name: String) -> Self {
        Self::StringAny { name, field_name }
    }

    pub fn javascript(
        name: String,
        field_names: Vec<String>,
        fn_aggregate: String,
        fn_combine: String,
        fn_reset: String,
    ) -> Self {
        Self::Javascript {
            name,
            field_names,
            fn_aggregate,
            fn_combine,
            fn_reset,
        }
    }

    pub fn filtered(filter: Filter, aggregator: Aggregator) -> Self {
        Self::Filtered {
            filter,
            aggregator: Box::new(aggregator),
        }
    }

    pub fn grouping(name: String, groupings: Vec<String>) -> Self {
        Self::Grouping { name, groupings }
    }

    pub fn cardinality(
        name: String,
        fields: Vec<DimensionSpec>,
        by_row: Option<bool>,
        round: Option<bool>,
    ) -> Self {
        Self::Cardinality {
            name,
            fields,
            by_row,
            round,
        }
    }

    pub fn hyper_unique(
        name: String,
        field_name: String,
        is_input_hyper_unique: Option<bool>,
        round: Option<bool>,
    ) -> Self {
        Self::HyperUnique {
            name,
            field_name,
            is_input_hyper_unique,
            round,
        }
    }
}
