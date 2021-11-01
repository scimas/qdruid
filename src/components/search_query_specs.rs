use super::druid_types::DruidNativeType;

#[derive(Debug, Clone)]
pub enum SearchQuerySpec {
    InsensitiveContains {
        value: DruidNativeType,   // must be String
    },
    Fragment {
        case_sensitive: bool,
        values: Vec<DruidNativeType>, // must be String
    },
    Contains {
        case_sensitive: bool,
        value: DruidNativeType,   // must be String
    },
    Regex {
        pattern: String,
    }
}
