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
    pub fn count(name: &str) -> Self {
        Self::Count {
            name: name.into()
        }
    }

    pub fn long_sum(name: &str, field_name: &str) -> Self {
        Self::LongSum {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn double_sum(name: &str, field_name: &str) -> Self {
        Self::DoubleSum {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn float_sum(name: &str, field_name: &str) -> Self {
        Self::FloatSum {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn long_min(name: &str, field_name: &str) -> Self {
        Self::LongMin {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn double_min(name: &str, field_name: &str) -> Self {
        Self::DoubleMin {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn float_min(name: &str, field_name: &str) -> Self {
        Self::FloatMin {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn long_max(name: &str, field_name: &str) -> Self {
        Self::LongMax {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn double_max(name: &str, field_name: &str) -> Self {
        Self::DoubleMax {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn float_max(name: &str, field_name: &str) -> Self {
        Self::FloatMax {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn long_first(name: &str, field_name: &str) -> Self {
        Self::LongFirst {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn double_first(name: &str, field_name: &str) -> Self {
        Self::DoubleFirst {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn float_first(name: &str, field_name: &str) -> Self {
        Self::FloatFirst {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn string_first(name: &str, field_name: &str) -> Self {
        Self::StringFirst {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn long_last(name: &str, field_name: &str) -> Self {
        Self::LongLast {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn double_last(name: &str, field_name: &str) -> Self {
        Self::DoubleLast {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn float_last(name: &str, field_name: &str) -> Self {
        Self::FloatLast {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn string_last(name: &str, field_name: &str) -> Self {
        Self::StringLast {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn long_any(name: &str, field_name: &str) -> Self {
        Self::LongAny {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn double_any(name: &str, field_name: &str) -> Self {
        Self::DoubleAny {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn float_any(name: &str, field_name: &str) -> Self {
        Self::FloatAny {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn string_any(name: &str, field_name: &str) -> Self {
        Self::StringAny {
            name: name.into(),
            field_name: field_name.into(),
        }
    }

    pub fn javascript(name: &str, field_names: &[String], fn_aggregate: &str, fn_combine: &str, fn_reset: &str) -> Self {
        Self::Javascript {
            name: name.into(),
            field_names: field_names.to_vec(),
            fn_aggregate: fn_aggregate.into(),
            fn_combine: fn_combine.into(),
            fn_reset: fn_reset.into(),
        }
    }

    pub fn filtered(filter: Filter, aggregator: Aggregator) -> Self {
        Self::Filtered {
            filter,
            aggregator: Box::new(aggregator),
        }
    }

    pub fn grouping(name: &str, groupings: &[String]) -> Self {
        Self::Grouping {
            name: name.into(),
            groupings: groupings.to_vec(),
        }
    }

    pub fn cardinality(name: &str, fields: &[DimensionSpec], by_row: Option<bool>, round: Option<bool>) -> Self {
        Self::Cardinality {
            name: name.into(),
            fields: fields.to_vec(),
            by_row,
            round,
        }
    }

    pub fn hyper_unique(name: &str, field_name: &str, is_input_hyper_unique: Option<bool>, round: Option<bool>) -> Self {
        Self::HyperUnique {
            name: name.into(),
            field_name: field_name.into(),
            is_input_hyper_unique,
            round
        }
    }
}
