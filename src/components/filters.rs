use serde::{Deserialize, Serialize};

use super::{
    dimension_specs::DimensionSpec, extraction_functions::ExtractionFunction, intervals::Interval,
    ordering::Ordering, search_query_specs::SearchQuerySpec,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
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
        ordering: Option<Ordering>,
        extraction_fn: Option<ExtractionFunction>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
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

impl Filter {
    pub fn selector(
        dimension: DimensionSpec,
        value: String,
        extraction_fn: Option<ExtractionFunction>,
    ) -> Self {
        Self::Selector {
            dimension,
            value,
            extraction_fn,
        }
    }

    pub fn column_comparison(dimensions: Vec<DimensionSpec>) -> Self {
        Self::ColumnComparison { dimensions }
    }

    pub fn regex(
        dimension: DimensionSpec,
        pattern: String,
        extraction_fn: Option<ExtractionFunction>,
    ) -> Self {
        Self::Regex {
            dimension,
            pattern,
            extraction_fn,
        }
    }

    pub fn and(fields: Vec<Filter>) -> Self {
        Self::And { fields }
    }

    pub fn or(fields: Vec<Filter>) -> Self {
        Self::Or { fields }
    }

    pub fn not(field: Filter) -> Self {
        Self::Not {
            field: Box::new(field),
        }
    }

    pub fn javascript(
        dimension: DimensionSpec,
        function: String,
        extraction_fn: Option<ExtractionFunction>,
    ) -> Self {
        Self::Javascript {
            dimension,
            function,
            extraction_fn,
        }
    }

    pub fn extraction(
        dimension: DimensionSpec,
        value: String,
        extraction_fn: ExtractionFunction,
    ) -> Self {
        Self::Extraction {
            dimension,
            value,
            extraction_fn,
        }
    }

    pub fn search(
        dimension: DimensionSpec,
        query: SearchQuerySpec,
        extraction_fn: Option<ExtractionFunction>,
    ) -> Self {
        Self::Search {
            dimension,
            query,
            extraction_fn,
        }
    }

    pub fn in_filter(dimension: DimensionSpec, values: Vec<String>) -> Self {
        Self::In { dimension, values }
    }

    pub fn like(
        dimension: String,
        pattern: String,
        escape: Option<String>,
        extraction_fn: Option<ExtractionFunction>,
    ) -> Self {
        Self::Like {
            dimension,
            pattern,
            escape,
            extraction_fn,
        }
    }

    pub fn bound(
        dimension: String,
        lower: Option<String>,
        upper: Option<String>,
        lower_strict: Option<bool>,
        upper_strict: Option<bool>,
        ordering: Option<Ordering>,
        extraction_fn: Option<ExtractionFunction>,
    ) -> Self {
        Self::Bound {
            dimension,
            lower,
            upper,
            lower_strict,
            upper_strict,
            ordering,
            extraction_fn,
        }
    }

    pub fn interval(
        dimension: String,
        intervals: Vec<Interval>,
        extraction_fn: Option<ExtractionFunction>,
    ) -> Self {
        Self::Interval {
            dimension,
            intervals,
            extraction_fn,
        }
    }

    pub fn expression(expression: String) -> Self {
        Self::Expression { expression }
    }

    pub fn true_filter() -> Self {
        Self::True {}
    }

    pub fn spatial(dimension: String, bound: Bound) -> Self {
        Self::Spatial { dimension, bound }
    }
}

impl Bound {
    pub fn rectangular(min_coords: Vec<f64>, max_coords: Vec<f64>) -> Self {
        Self::Rectangular {
            min_coords,
            max_coords,
        }
    }

    pub fn radius(coords: Vec<f64>, radius: f64) -> Self {
        Self::Radius { coords, radius }
    }

    pub fn polygon(abscissa: Vec<f64>, ordinate: Vec<f64>) -> Self {
        Self::Polygon { abscissa, ordinate }
    }
}
