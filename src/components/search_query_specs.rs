use std::{error::Error, fmt::Display};

use super::druid_types::DruidNativeType;

#[derive(Debug, Clone)]
pub enum SearchQuerySpec {
    InsensitiveContains {
        value: DruidNativeType, // must be String
    },
    Fragment {
        case_sensitive: bool,
        values: Vec<DruidNativeType>, // must be String
    },
    Contains {
        case_sensitive: bool,
        value: DruidNativeType, // must be String
    },
    Regex {
        pattern: String,
    },
}

impl SearchQuerySpec {
    pub fn insensitive_contains(value: DruidNativeType) -> Result<Self, ValueNotString> {
        match &value {
            DruidNativeType::String(_) => Ok(Self::InsensitiveContains { value }),
            _ => Err(ValueNotString {}),
        }
    }

    pub fn fragment(
        case_sensitive: bool,
        values: Vec<DruidNativeType>,
    ) -> Result<Self, ValueNotString> {
        if values
            .iter()
            .any(|val| !matches!(val, DruidNativeType::String(_)))
        {
            // some value is not String
            Err(ValueNotString {})
        } else {
            Ok(Self::Fragment {
                case_sensitive,
                values,
            })
        }
    }

    pub fn contains(case_sensitive: bool, value: DruidNativeType) -> Result<Self, ValueNotString> {
        match &value {
            DruidNativeType::String(_) => Ok(Self::Contains {
                case_sensitive,
                value,
            }),
            _ => Err(ValueNotString {}),
        }
    }

    pub fn regex(pattern: String) -> Self {
        Self::Regex { pattern }
    }
}

#[derive(Debug)]
pub struct ValueNotString {}

impl Display for ValueNotString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value must be String")
    }
}

impl Error for ValueNotString {}
