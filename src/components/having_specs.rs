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
