use serde::{Deserialize, Serialize};

use super::druid_types::DruidNativeType;
use crate::queries::Query;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DataSource {
    Table {
        name: String,
    },
    Lookup {
        lookup: String,
    },
    Union {
        data_sources: Vec<String>,
    },
    Inline {
        column_names: Vec<String>,
        rows: Vec<Vec<DruidNativeType>>,
    },
    Query(Box<Query>),
    Join {
        left: Box<DataSource>,  // must not be union
        right: Box<DataSource>, // must not be table, union or join
        right_prefix: String,   // must not be empty
        condition: String,
        join_type: String, // must be INNER or LEFT
    },
}

impl DataSource {
    pub fn datasource_type(&self) -> String {
        match self {
            DataSource::Table { .. } => "table",
            DataSource::Lookup { .. } => "lookup",
            DataSource::Union { .. } => "union",
            DataSource::Inline { .. } => "inline",
            DataSource::Query { .. } => "query",
            DataSource::Join { .. } => "join",
        }
        .into()
    }

    pub fn table(name: String) -> Self {
        Self::Table { name }
    }

    pub fn lookup(lookup: String) -> Self {
        Self::Lookup { lookup }
    }

    pub fn union(data_sources: Vec<String>) -> Self {
        Self::Union { data_sources }
    }

    pub fn inline(column_names: Vec<String>, rows: Vec<Vec<DruidNativeType>>) -> Self {
        Self::Inline { column_names, rows }
    }

    pub fn query(q: Query) -> Self {
        Self::Query(Box::new(q))
    }

    pub fn join(
        left: DataSource,
        right: DataSource,
        right_prefix: String,
        condition: String,
        join_type: String,
    ) -> Result<Self, JoinDataSourceError> {
        match left {
            DataSource::Union { .. } => Err(JoinDataSourceError::InvalidJoinSource {
                which: "left".into(),
                datasource_type: left.datasource_type(),
            }),
            _ => match right {
                DataSource::Table { .. } | DataSource::Union { .. } | DataSource::Join { .. } => {
                    Err(JoinDataSourceError::InvalidJoinSource {
                        which: "right".into(),
                        datasource_type: right.datasource_type(),
                    })
                }
                _ => {
                    if right_prefix.is_empty() {
                        return Err(JoinDataSourceError::EmptyRightPrefix);
                    }
                    let join_type = join_type.to_uppercase();
                    if &join_type != "INNER" && &join_type != "LEFT" {
                        return Err(JoinDataSourceError::InvalidJoinType(join_type));
                    }
                    if !condition.contains("==") {
                        return Err(JoinDataSourceError::ConditionNotEquality);
                    }
                    Ok(Self::Join {
                        left: Box::new(left),
                        right: Box::new(right),
                        right_prefix,
                        condition,
                        join_type,
                    })
                }
            },
        }
    }
}

#[derive(Debug)]
pub enum JoinDataSourceError {
    InvalidJoinSource {
        which: String, // left or right
        datasource_type: String,
    },
    InvalidJoinType(String),
    EmptyRightPrefix,
    ConditionNotEquality,
}

impl Display for JoinDataSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidJoinSource {
                which,
                datasource_type,
            } => write!(f, "invalid {} join source {}", which, datasource_type),
            Self::InvalidJoinType(join_type) => write!(f, "invalid join type {}", join_type),
            Self::EmptyRightPrefix => write!(f, "right_prefix is empty"),
            Self::ConditionNotEquality => write!(f, "condition is not equality"),
        }
    }
}

impl Error for JoinDataSourceError {}

#[cfg(test)]
mod tests {
    use super::{DataSource, JoinDataSourceError};
    use std::error::Error;

    #[test]
    fn invalid_left_fails() -> Result<(), Box<dyn Error>> {
        let invalid_left = DataSource::union(vec!["a".into(), "b".into()]);
        let right = DataSource::lookup("l".to_owned());
        let right_prefix = "r".to_owned();
        let condition = "c == d".to_owned();
        let join_type = "inner".to_owned();
        match DataSource::join(invalid_left, right, right_prefix, condition, join_type) {
            Err(JoinDataSourceError::InvalidJoinSource { which, .. }) if which == "left" => Ok(()),
            Err(e) => Err(Box::new(e)),
            _ => Err("did not receive the expected error".into()),
        }
    }

    #[test]
    fn invalid_right_fails() -> Result<(), Box<dyn Error>> {
        let left = DataSource::table("t".to_owned());
        let invalid_right = DataSource::table("t2".to_owned());
        let right_prefix = "r".to_owned();
        let condition = "c == d".to_owned();
        let join_type = "left".to_owned();
        match DataSource::join(left, invalid_right, right_prefix, condition, join_type) {
            Err(JoinDataSourceError::InvalidJoinSource { which, .. }) if which == "right" => Ok(()),
            Err(e) => Err(Box::new(e)),
            _ => Err("did not receive the expected error".into()),
        }
    }

    #[test]
    fn empty_right_prefix_fails() -> Result<(), Box<dyn Error>> {
        let left = DataSource::table("t".to_owned());
        let right = DataSource::lookup("l".to_owned());
        let right_prefix = "".to_owned();
        let condition = "c == d".to_owned();
        let join_type = "inner".to_owned();
        match DataSource::join(left, right, right_prefix, condition, join_type) {
            Err(JoinDataSourceError::EmptyRightPrefix) => Ok(()),
            Err(e) => Err(Box::new(e)),
            _ => Err("did not receive the expected error".into()),
        }
    }

    #[test]
    fn invalid_join_type_fails() -> Result<(), Box<dyn Error>> {
        let left = DataSource::table("t".to_owned());
        let right = DataSource::lookup("l".to_owned());
        let right_prefix = "r".to_owned();
        let condition = "c == d".to_owned();
        let join_type = "outer".to_owned();
        match DataSource::join(left, right, right_prefix, condition, join_type) {
            Err(JoinDataSourceError::InvalidJoinType(join_type)) if join_type == "OUTER" => Ok(()),
            Err(e) => Err(Box::new(e)),
            _ => Err("did not receive the expected error".into()),
        }
    }

    #[test]
    fn invalid_condition_fails() -> Result<(), Box<dyn Error>> {
        let left = DataSource::table("t".to_owned());
        let right = DataSource::lookup("l".to_owned());
        let right_prefix = "r".to_owned();
        let condition = "c != d".to_owned();
        let join_type = "left".to_owned();
        match DataSource::join(left, right, right_prefix, condition, join_type) {
            Err(JoinDataSourceError::ConditionNotEquality) => Ok(()),
            Err(e) => Err(Box::new(e)),
            _ => Err("did not receive the expected error".into()),
        }
    }
}
