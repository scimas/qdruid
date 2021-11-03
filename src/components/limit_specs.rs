use super::ordering::InvalidOrderingError;

#[derive(Debug, Clone)]
pub enum LimitSpec {
    Default {
        limit: Option<usize>,
        offset: Option<usize>,
        columns: Option<Vec<LimitSpec>>,
    },
    OrderByColumnSpec {
        dimension: String,
        direction: String,
        dimension_order: Option<String>, // must be one of "lexicographic", "alphanumeric", "strlen" or "numeric"
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
        dimension_order: Option<String>,
    ) -> Result<Self, InvalidOrderingError> {
        if let Some(s) = &dimension_order {
            let s = s.to_lowercase();
            if s != "lexicographic" && s != "alphanumeric" && s != "strlen" && s != "numeric" {
                return Err(InvalidOrderingError::new(
                    s,
                    r#""lexicographic", "alphanumeric", "strlen" or "numeric""#.into(),
                ));
            }
        }
        Ok(Self::OrderByColumnSpec {
            dimension,
            direction,
            dimension_order,
        })
    }
}
