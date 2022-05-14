use serde::{Deserialize, Serialize};

use super::{druid_types::DruidNativeType, ordering::InvalidOrderingError};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
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
    pub fn arithmetic(
        name: String,
        func: String,
        fields: Vec<PostAggregator>,
        ordering: Option<String>,
    ) -> Result<Self, InvalidOrderingError> {
        if let Some(s) = &ordering {
            if s.to_lowercase() != "numericfirst" {
                return Err(InvalidOrderingError::new(s.into(), "numericFirst".into()));
            }
        }
        Ok(Self::Arithmetic {
            name,
            func,
            fields,
            ordering,
        })
    }

    pub fn field_access(name: String, field_name: String) -> Self {
        Self::FieldAccess { name, field_name }
    }

    pub fn finalizing_field_access(name: String, field_name: String) -> Self {
        Self::FinalizingFieldAccess { name, field_name }
    }

    pub fn constant(name: &str, value: DruidNativeType) -> Result<Self, NonNumericConstant> {
        match value {
            DruidNativeType::Long(_) | DruidNativeType::Double(_) | DruidNativeType::Float(_) => {
                Ok(Self::Constant {
                    name: name.into(),
                    value,
                })
            }
            _ => Err(NonNumericConstant {}),
        }
    }

    pub fn long_greatest(name: String, fields: Vec<PostAggregator>) -> Self {
        Self::LongGreatest { name, fields }
    }

    pub fn double_greatest(name: String, fields: Vec<PostAggregator>) -> Self {
        Self::DoubleGreatest { name, fields }
    }

    pub fn long_least(name: String, fields: Vec<PostAggregator>) -> Self {
        Self::LongLeast { name, fields }
    }

    pub fn double_least(name: String, fields: Vec<PostAggregator>) -> Self {
        Self::DoubleLeast { name, fields }
    }

    pub fn javascript(name: String, field_names: Vec<String>, function: String) -> Self {
        Self::Javascript {
            name,
            field_names,
            function,
        }
    }

    pub fn hyper_unique_cardinality(name: String, field_name: String) -> Self {
        Self::HyperUniqueCardinality { name, field_name }
    }
}

#[derive(Debug, Clone, Copy, thiserror::Error)]
#[error("constant must be numeric")]
pub struct NonNumericConstant;

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
