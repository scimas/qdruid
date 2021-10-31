#[derive(Debug, Clone)]
pub enum TopNMetricSpec {
    Numeric {
        metric: String,
    },
    Dimension {
        ordering: Option<String>,
        previous_stop: Option<String>,
    },
    Inverted {
        metric: Box<TopNMetricSpec>,
    },
}
