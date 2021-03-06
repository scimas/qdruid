use serde::{Deserialize, Serialize};

use super::{
    druid_types::DruidNativeType,
    extraction_functions::{ExtractionFunction, LookupError},
    lookups::Lookup,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DimensionSpec {
    Default {
        dimension: String,
        output_name: String,
        output_type: Option<String>,
    },
    Extraction {
        dimension: String,
        output_name: String,
        extraction_fn: ExtractionFunction,
        output_type: Option<String>,
    },
    ListFiltered {
        delegate: Box<DimensionSpec>,
        values: Vec<String>,
        is_whitelist: Option<bool>,
    },
    RegexFiltered {
        delegate: Box<DimensionSpec>,
        pattern: String,
    },
    PrefixFiltered {
        delegate: Box<DimensionSpec>,
        prefix: String,
    },
    Lookup {
        dimension: String,
        output_name: String,
        retain_missing_value: Option<bool>, // retain = true and replace = not null is illegal
        replace_missing_value_with: Option<DruidNativeType>,
        lookup: Option<Lookup>, // must be of type Map
        optimize: Option<bool>,
        name: Option<String>,
    },
}

impl DimensionSpec {
    pub fn default(dimension: String, output_name: String, output_type: Option<String>) -> Self {
        Self::Default {
            dimension,
            output_name,
            output_type,
        }
    }

    pub fn extraction(
        dimension: String,
        output_name: String,
        extraction_fn: ExtractionFunction,
        output_type: Option<String>,
    ) -> Self {
        Self::Extraction {
            dimension,
            output_name,
            extraction_fn,
            output_type,
        }
    }

    pub fn list_filtered(
        delegate: DimensionSpec,
        values: Vec<String>,
        is_whitelist: Option<bool>,
    ) -> Self {
        Self::ListFiltered {
            delegate: Box::new(delegate),
            values,
            is_whitelist,
        }
    }

    pub fn regex_filtered(delegate: DimensionSpec, pattern: String) -> Self {
        Self::RegexFiltered {
            delegate: Box::new(delegate),
            pattern,
        }
    }

    pub fn prefix_filtered(delegate: DimensionSpec, prefix: String) -> Self {
        Self::PrefixFiltered {
            delegate: Box::new(delegate),
            prefix,
        }
    }

    pub fn lookup(
        dimension: String,
        output_name: String,
        retain_missing_value: Option<bool>,
        replace_missing_value_with: Option<DruidNativeType>,
        lookup: Option<Lookup>,
        optimize: Option<bool>,
        name: Option<String>,
    ) -> Result<Self, LookupError> {
        if let Some(true) = retain_missing_value {
            if let Some(s) = &replace_missing_value_with {
                match s {
                    DruidNativeType::String(s) => {
                        if !s.is_empty() {
                            return Err(LookupError::RetainingReplacingSimultaneously);
                        }
                    }
                    _ => return Err(LookupError::RetainingReplacingSimultaneously),
                }
            }
        }
        match lookup {
            Some(Lookup::Map { .. }) | None => Ok(Self::Lookup {
                dimension,
                output_name,
                retain_missing_value,
                replace_missing_value_with,
                lookup,
                optimize,
                name,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::components::{druid_types::DruidNativeType, extraction_functions::LookupError};

    use super::DimensionSpec;

    #[test]
    fn retain_true_replace_some_string_fails() -> Result<(), Box<dyn Error>> {
        match DimensionSpec::lookup(
            "dim".to_owned(),
            "out_name".to_owned(),
            Some(true),
            Some(DruidNativeType::String("a".into())),
            None,
            None,
            None,
        ) {
            Err(LookupError::RetainingReplacingSimultaneously) => Ok(()),
            Err(e) => Err(Box::new(e)),
            _ => Err("did not receive the expected error".into()),
        }
    }

    #[test]
    fn retain_true_replace_some_fails() -> Result<(), Box<dyn Error>> {
        match DimensionSpec::lookup(
            "dim".to_owned(),
            "out_name".to_owned(),
            Some(true),
            Some(DruidNativeType::Long(2)),
            None,
            None,
            None,
        ) {
            Err(LookupError::RetainingReplacingSimultaneously) => Ok(()),
            Err(e) => Err(Box::new(e)),
            _ => Err("did not receive the expected error".into()),
        }
    }

    #[test]
    fn retain_true_replace_empty_string_succeeds() -> Result<(), Box<dyn Error>> {
        match DimensionSpec::lookup(
            "dim".to_owned(),
            "out_name".to_owned(),
            Some(true),
            Some(DruidNativeType::String("".into())),
            None,
            None,
            None,
        ) {
            Ok(_) => Ok(()),
            Err(LookupError::RetainingReplacingSimultaneously) => Err("empty string not being treated as valid input for replace_missing_value_with with retain_missing_value = true".into()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
