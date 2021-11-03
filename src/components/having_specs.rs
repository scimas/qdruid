use super::{druid_types::DruidNativeType, filters::Filter};

#[derive(Debug, Clone)]
pub enum HavingSpec {
    Filter {
        filter: Filter,
    },
    EqualTo {
        aggregation: String,
        value: DruidNativeType,
    },
    GreaterThan {
        aggregation: String,
        value: DruidNativeType,
    },
    LessThan {
        aggregation: String,
        value: DruidNativeType,
    },
    DimSelector {
        dimension: String,
        value: DruidNativeType,
    },
    And {
        having_specs: Vec<HavingSpec>,
    },
    Or {
        having_specs: Vec<HavingSpec>,
    },
    Not {
        having_spec: Box<HavingSpec>,
    },
}

impl HavingSpec {
    pub fn filter(filter: Filter) -> Self {
        Self::Filter { filter }
    }

    pub fn equal_to(aggregation: String, value: DruidNativeType) -> Self {
        Self::EqualTo { aggregation, value }
    }

    pub fn greater_than(aggregation: String, value: DruidNativeType) -> Self {
        Self::GreaterThan { aggregation, value }
    }

    pub fn less_than(aggregation: String, value: DruidNativeType) -> Self {
        Self::LessThan { aggregation, value }
    }

    pub fn dim_selector(dimension: String, value: DruidNativeType) -> Self {
        Self::DimSelector { dimension, value }
    }

    pub fn and(having_specs: Vec<HavingSpec>) -> Self {
        Self::And { having_specs }
    }

    pub fn or(having_specs: Vec<HavingSpec>) -> Self {
        Self::Or { having_specs }
    }

    pub fn not(having_spec: HavingSpec) -> Self {
        Self::Not {
            having_spec: Box::new(having_spec),
        }
    }
}
