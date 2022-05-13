use serde::{Deserialize, Serialize};

use super::ordering::InvalidOrderingError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum TopNMetricSpec {
    Numeric {
        metric: String,
    },
    Dimension {
        ordering: Option<String>, // must be one of "lexicographic", "alphanumeric", "numeric" or "strlen"
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

    pub fn dimension(
        ordering: Option<String>,
        previous_stop: Option<String>,
    ) -> Result<Self, InvalidOrderingError> {
        if let Some(s) = &ordering {
            let s = s.to_lowercase();
            if s != "lexicographic" && s != "alphanumeric" && s != "strlen" && s != "numeric" {
                return Err(InvalidOrderingError::new(
                    s,
                    r#""lexicographic", "alphanumeric", "strlen" or "numeric""#.into(),
                ));
            }
        }
        Ok(Self::Dimension {
            ordering,
            previous_stop,
        })
    }

    pub fn inverted(metric: TopNMetricSpec) -> Self {
        Self::Inverted {
            metric: Box::new(metric),
        }
    }
}
