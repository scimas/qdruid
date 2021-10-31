use super::{druid_types::DruidType, filters::Filter};

pub enum HavingSpec {
    Filter {
        filter: Filter,
    },
    EqualTo {
        aggregation: String,
        value: DruidType,
    },
    GreaterThan {
        aggregation: String,
        value: DruidType,
    },
    LessThan {
        aggregation: String,
        value: DruidType,
    },
    DimSelector {
        dimension: String,
        value: DruidType,
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
