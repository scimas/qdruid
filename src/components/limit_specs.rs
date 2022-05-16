use serde::{Deserialize, Serialize};

use super::ordering::Ordering;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum LimitSpec {
    Default {
        limit: Option<usize>,
        offset: Option<usize>,
        columns: Option<Vec<LimitSpec>>,
    },
    OrderByColumnSpec {
        dimension: String,
        direction: String,
        dimension_order: Option<Ordering>, // must be one of "lexicographic", "alphanumeric", "strlen" or "numeric"
    },
}

impl LimitSpec {
    pub fn default(
        limit: Option<usize>,
        offset: Option<usize>,
        columns: Option<Vec<LimitSpec>>,
    ) -> Self {
        Self::Default {
            limit,
            offset,
            columns,
        }
    }

    pub fn order_by_columns_spec(
        dimension: String,
        direction: String,
        dimension_order: Option<Ordering>,
    ) -> Self {
        Self::OrderByColumnSpec {
            dimension,
            direction,
            dimension_order,
        }
    }
}
