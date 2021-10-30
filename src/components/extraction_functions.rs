use super::{
    druid_types::DruidType, granularities::Granularity, lookups::Lookup,
    search_query_specs::SearchQuerySpec,
};

pub enum ExtractionFunction {
    Regex {
        expr: String,
        index: Option<usize>,
        replace_missing_value: Option<bool>,
        replace_missing_value_with: Option<DruidType>,
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
        replace_missing_value_with: Option<DruidType>,
        injective: Option<bool>,
        optimize: Option<bool>,
    },
    Lookup {
        lookup: Lookup,                     // must be of type Map
        retain_missing_value: Option<bool>, // retain = true and replace = not null is illegal
        replace_missing_value_with: Option<DruidType>,
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
