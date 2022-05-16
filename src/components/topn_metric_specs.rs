use serde::{Deserialize, Serialize};

use super::ordering::Ordering;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum TopNMetricSpec {
    Numeric {
        metric: String,
    },
    Dimension {
        ordering: Option<Ordering>, // must be one of "lexicographic", "alphanumeric", "numeric" or "strlen"
        previous_stop: Option<String>,
    },
    Inverted {
        metric: Box<TopNMetricSpec>,
    },
}

impl TopNMetricSpec {
    pub fn numeric(metric: String) -> Self {
        Self::Numeric { metric }
    }

    pub fn dimension(ordering: Option<Ordering>, previous_stop: Option<String>) -> Self {
        Self::Dimension {
            ordering,
            previous_stop,
        }
    }

    pub fn inverted(metric: TopNMetricSpec) -> Self {
        Self::Inverted {
            metric: Box::new(metric),
        }
    }
}
