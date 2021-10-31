use super::druid_types::DruidType;

#[derive(Debug, Clone)]
pub enum SearchQuerySpec {
    InsensitiveContains {
        value: DruidType,   // must be String
    },
    Fragment {
        case_sensitive: bool,
        values: Vec<DruidType>, // must be String
    },
    Contains {
        case_sensitive: bool,
        value: DruidType,   // must be String
    },
    Regex {
        pattern: String,
    }
}
