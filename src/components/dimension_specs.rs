use super::{druid_types::DruidType, extraction_functions::ExtractionFunction, lookups::Lookup};

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
        replace_missing_value_with: Option<DruidType>,
        lookup: Option<Lookup>, // must be of type Map
        optimize: Option<bool>,
        name: Option<String>,
    },
}
