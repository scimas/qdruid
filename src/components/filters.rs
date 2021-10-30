use super::{
    dimension_specs::DimensionSpec, extraction_functions::ExtractionFunction, intervals::Interval,
    search_query_specs::SearchQuerySpec,
};

pub enum Filter {
    Selector {
        dimension: DimensionSpec,
        value: String,
        extraction_fn: Option<ExtractionFunction>,
    },
    ColumnComparison {
        dimensions: Vec<DimensionSpec>,
    },
    Regex {
        dimension: DimensionSpec,
        pattern: String,
        extraction_fn: Option<ExtractionFunction>,
    },
    And {
        fields: Vec<Filter>,
    },
    Or {
        fields: Vec<Filter>,
    },
    Not {
        field: Box<Filter>,
    },
    Javascript {
        dimension: DimensionSpec,
        function: String,
        extraction_fn: Option<ExtractionFunction>,
    },
    Extraction {
        dimension: DimensionSpec,
        value: String,
        extraction_fn: ExtractionFunction,
    },
    Search {
        dimension: DimensionSpec,
        query: SearchQuerySpec,
        extraction_fn: Option<ExtractionFunction>,
    },
    In {
        dimension: DimensionSpec,
        values: Vec<String>,
    },
    Like {
        dimension: String,
        pattern: String,
        escape: Option<String>,
        extraction_fn: Option<ExtractionFunction>,
    },
    Bound {
        dimension: String,
        lower: Option<String>,
        upper: Option<String>,
        lower_strict: Option<bool>,
        upper_strict: Option<bool>,
        ordering: Option<String>, // must be one of "lexicographic", "alphanumeric", "numeric", "strlen", "version"
        extraction_fn: ExtractionFunction,
    },
    Interval {
        dimension: String,
        intervals: Vec<Interval>,
        extraction_fn: Option<ExtractionFunction>,
    },
    Expression {
        expression: String,
    },
    True,
    Spatial {
        dimension: String,
        bound: Bound,
    },
}

pub enum Bound {
    Rectangular {
        min_coords: Vec<f64>,
        max_coords: Vec<f64>,
    },
    Radius {
        coords: Vec<f64>,
        radius: f64,
    },
    Polygon {
        abscissa: Vec<f64>,
        ordinate: Vec<f64>,
    },
}
