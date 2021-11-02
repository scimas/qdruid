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
    pub fn regex(
        expr: String,
        index: Option<usize>,
        replace_missing_value: Option<bool>,
        replace_missing_value_with: Option<DruidNativeType>,
    ) -> Self {
        Self::Regex {
            expr,
            index,
            replace_missing_value,
            replace_missing_value_with,
        }
    }

    pub fn partial(expr: String) -> Self {
        Self::Partial { expr }
    }

    pub fn search_query(query: SearchQuerySpec) -> Self {
        Self::SearchQuery { query }
    }

    pub fn substring(index: usize, length: Option<usize>) -> Self {
        Self::Substring { index, length }
    }

    pub fn strlen() -> Self {
        Self::Strlen {}
    }

    pub fn time_format(
        format: Option<String>,
        time_zone: Option<String>,
        locale: Option<String>,
        granularity: Option<Granularity>,
        as_millis: Option<bool>,
    ) -> Self {
        Self::TimeFormat {
            format,
            time_zone,
            locale,
            granularity,
            as_millis,
        }
    }

    pub fn time(time_format: String, result_format: String, joda: bool) -> Self {
        Self::Time {
            time_format,
            result_format,
            joda,
        }
    }

    pub fn javascript(function: String, injective: Option<bool>) -> Self {
        Self::Javascript {
            function,
            injective,
        }
    }

    pub fn registered_lookup(
        lookup: String,
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
            lookup,
            retain_missing_value,
            replace_missing_value_with,
            injective,
            optimize,
        })
    }

    pub fn lookup(
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

    pub fn cascade(extraction_fns: Vec<ExtractionFunction>) -> Self {
        Self::Cascade { extraction_fns }
    }

    pub fn string_format(
        format: String,
        null_handling: Option<String>,
    ) -> Result<Self, NullHandlingError> {
        if let Some(s) = &null_handling {
            if s != "nullString" && s != "emptyString" && s != "returnNull" {
                return Err(NullHandlingError {});
            }
        }
        Ok(Self::StringFormat {
            format,
            null_handling,
        })
    }

    pub fn upper(locale: Option<String>) -> Self {
        Self::Upper { locale }
    }

    pub fn lower(locale: Option<String>) -> Self {
        Self::Lower { locale }
    }

    pub fn bucket(size: Option<usize>, offset: Option<usize>) -> Self {
        Self::Bucket { size, offset }
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
pub struct NullHandlingError {}

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
                "lookup".to_owned(),
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
                "lookup".to_owned(),
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
                "lookup".to_owned(),
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
        match ExtractionFunction::string_format("a".to_owned(), Some("non_existent_option".into()))
        {
            Err(NullHandlingError {}) => Ok(()),
            _ => Err("did not receive the expected error".into()),
        }
    }
}
