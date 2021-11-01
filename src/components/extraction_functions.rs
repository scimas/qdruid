use super::{
    druid_types::DruidNativeType, granularities::Granularity, lookups::Lookup,
    search_query_specs::SearchQuerySpec,
};
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum ExtractionFunction {
    Regex {
        expr: String,
        index: Option<usize>,
        replace_missing_value: Option<bool>,
        replace_missing_value_with: Option<DruidNativeType>,
    },
    Partial {
        expr: String,
    },
    SearchQuery {
        query: SearchQuerySpec,
    },
    Substring {
        index: usize,
        length: Option<usize>,
    },
    Strlen,
    TimeFormat {
        format: Option<String>,
        time_zone: Option<String>,
        locale: Option<String>,
        granularity: Option<Granularity>,
        as_millis: Option<bool>,
    },
    Time {
        time_format: String,
        result_format: String,
        joda: bool,
    },
    Javascript {
        function: String,
        injective: Option<bool>,
    },
    RegisteredLookup {
        lookup: String,
        retain_missing_value: Option<bool>, // retain = true and replace = not null is illegal
        replace_missing_value_with: Option<DruidNativeType>,
        injective: Option<bool>,
        optimize: Option<bool>,
    },
    Lookup {
        lookup: Lookup,                     // must be of type Map
        retain_missing_value: Option<bool>, // retain = true and replace = not null is illegal
        replace_missing_value_with: Option<DruidNativeType>,
        injective: Option<bool>,
        optimize: Option<bool>,
    },
    Cascade {
        extraction_fns: Vec<ExtractionFunction>,
    },
    StringFormat {
        format: String,
        null_handling: Option<String>, // must be one of nullString, emptyString or returnNull
    },
    Upper {
        locale: Option<String>,
    },
    Lower {
        locale: Option<String>,
    },
    Bucket {
        size: Option<usize>,
        offset: Option<usize>,
    },
}

impl ExtractionFunction {
    fn registered_lookup(
        lookup: &str,
        retain_missing_value: Option<bool>,
        replace_missing_value_with: Option<DruidNativeType>,
        injective: Option<bool>,
        optimize: Option<bool>,
    ) -> Result<Self, LookupError> {
        if let Some(true) = retain_missing_value {
            if let Some(s) = &replace_missing_value_with {
                match s {
                    DruidNativeType::String(s) => {
                        if s.len() != 0 {
                            return Err(LookupError::RetainingReplacingSimultaneously);
                        }
                    }
                    _ => return Err(LookupError::RetainingReplacingSimultaneously),
                }
            }
        }
        Ok(Self::RegisteredLookup {
            lookup: lookup.into(),
            retain_missing_value,
            replace_missing_value_with,
            injective,
            optimize,
        })
    }

    fn lookup(
        lookup: Lookup,
        retain_missing_value: Option<bool>,
        replace_missing_value_with: Option<DruidNativeType>,
        injective: Option<bool>,
        optimize: Option<bool>,
    ) -> Result<Self, LookupError> {
        if let Some(true) = retain_missing_value {
            if let Some(s) = &replace_missing_value_with {
                match s {
                    DruidNativeType::String(s) => {
                        if s.len() != 0 {
                            return Err(LookupError::RetainingReplacingSimultaneously);
                        }
                    }
                    _ => return Err(LookupError::RetainingReplacingSimultaneously),
                }
            }
        }
        match lookup {
            Lookup::Map { .. } => Ok(Self::Lookup {
                lookup,
                retain_missing_value,
                replace_missing_value_with,
                injective,
                optimize,
            }),
            _ => Err(LookupError::LookupNotMap),
        }
    }

    fn string_format(
        format: &str,
        null_handling: Option<String>,
    ) -> Result<Self, NullHandlingError> {
        if let Some(s) = &null_handling {
            if s != "nullString" && s != "emptyString" && s != "returnNull" {
                return Err(NullHandlingError {});
            }
        }
        Ok(Self::StringFormat {
            format: format.into(),
            null_handling,
        })
    }
}

#[derive(Debug)]
pub enum LookupError {
    RetainingReplacingSimultaneously,
    LookupNotMap,
}

impl Display for LookupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookupError::RetainingReplacingSimultaneously => write!(f, "retain_missing_value = true and non null (or non empty String) replace_missing_value_with at the same time are illegal"),
            LookupError::LookupNotMap => write!(f, "lookup must be of type Map for the Lookup extraction function"),
        }
    }
}

impl Error for LookupError {}

#[derive(Debug)]
struct NullHandlingError {}

impl Display for NullHandlingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"null_handling must be one of "nullString", "emptyString" or "returnNull""#
        )
    }
}

impl Error for NullHandlingError {}

#[cfg(test)]
mod tests {
    use super::{ExtractionFunction, NullHandlingError};
    use std::error::Error;

    mod registered_lookup {
        use std::error::Error;

        use crate::components::{
            druid_types::DruidNativeType,
            extraction_functions::{ExtractionFunction, LookupError},
        };

        #[test]
        fn retain_true_replace_some_string_fails() -> Result<(), Box<dyn Error>> {
            match ExtractionFunction::registered_lookup(
                "lookup",
                Some(true),
                Some(DruidNativeType::String("a".into())),
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
            match ExtractionFunction::registered_lookup(
                "lookup",
                Some(true),
                Some(DruidNativeType::Long(2)),
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
            match ExtractionFunction::registered_lookup(
                "lookup",
                Some(true),
                Some(DruidNativeType::String("".into())),
                None,
                None,
            ) {
                Ok(_) => Ok(()),
                Err(LookupError::RetainingReplacingSimultaneously) => Err("empty string not being treated as valid input for replace_missing_value_with with retain_missing_value = true".into()),
                Err(e) => Err(Box::new(e)),
            }
        }
    }

    #[test]
    fn string_format_fails_on_unknown_values() -> Result<(), Box<dyn Error>> {
        match ExtractionFunction::string_format("a", Some("non_existent_option".into())) {
            Err(NullHandlingError {}) => Ok(()),
            _ => Err("did not receive the expected error".into()),
        }
    }
}
