use std::{error::Error, fmt::Display};

use super::druid_types::DruidNativeType;

#[derive(Debug, Clone)]
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
        value: DruidNativeType, // must be numeric
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
    },
}

impl PostAggregator {
    fn constant(name: &str, value: DruidNativeType) -> Result<Self, NonNumericConstant> {
        match value {
            DruidNativeType::Long(_) | DruidNativeType::Double(_) | DruidNativeType::Float(_) => Ok(Self::Constant {
                name: name.into(),
                value,
            }),
            _ => Err(NonNumericConstant {}),
        }
    }
}

#[derive(Debug)]
struct NonNumericConstant {}

impl Display for NonNumericConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "constant must be numeric")
    }
}

impl Error for NonNumericConstant {}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::components::druid_types::DruidNativeType;

    use super::{NonNumericConstant, PostAggregator};

    #[test]
    fn contant_must_be_numeric() -> Result<(), Box<dyn Error>> {
        match PostAggregator::constant("name", DruidNativeType::String("heh".into())) {
            Err(NonNumericConstant {}) => Ok(()),
            _ => Err("did not receive the expected error".into()),
        }
    }
}
