use std::str::FromStr;

use serde::{
    de,
    ser::{SerializeMap, Serializer},
    Deserialize, Deserializer, Serialize,
};
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum Granularity {
    Simple(String),
    Duration {
        duration: u64,
        origin: Option<String>,
    },
    Period {
        period: String,
        origin: Option<String>,
        time_zone: Option<String>,
    },
}

impl Serialize for Granularity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::Simple(ref s) => serializer.serialize_str(s),
            Self::Duration {
                ref duration,
                ref origin,
            } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("type", "duration")?;
                map.serialize_entry("duration", duration)?;
                map.serialize_entry("origin", origin)?;
                map.end()
            }
            Self::Period {
                ref period,
                ref origin,
                ref time_zone,
            } => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("type", "period")?;
                map.serialize_entry("period", period)?;
                map.serialize_entry("origin", origin)?;
                map.serialize_entry("timeZone", time_zone)?;
                map.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for Granularity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        let val: Value = serde_json::to_value(s).map_err(de::Error::custom)?;
        match val {
            Value::String(s) => Ok(Self::Simple(s)),
            Value::Object(map) => match map.get("type") {
                None => Err(de::Error::custom(r#"expected field "type" not found"#)),
                Some(val) => match val {
                    Value::String(s) if s == "duration" => {
                        let duration = map.get("duration").ok_or_else(|| {
                            de::Error::custom(r#"expected field "duration" not found"#)
                        })?;
                        let origin = if let Some(val) = map.get("origin") {
                            Some(
                                val.as_str()
                                    .ok_or_else(|| {
                                        de::Error::custom(r#"field "origin" is not a string"#)
                                    })?
                                    .to_string(),
                            )
                        } else {
                            None
                        };
                        Ok(Granularity::Duration {
                            duration: duration.as_u64().ok_or_else(|| {
                                de::Error::custom(r#"field "duration" is not u64"#)
                            })?,
                            origin,
                        })
                    }
                    Value::String(s) if s == "period" => {
                        let period = map.get("period").ok_or_else(|| {
                            de::Error::custom(r#"expected field "period not found"#)
                        })?;
                        let origin = if let Some(val) = map.get("origin") {
                            Some(
                                val.as_str()
                                    .ok_or_else(|| {
                                        de::Error::custom(r#"field "origin" is not a string"#)
                                    })?
                                    .to_string(),
                            )
                        } else {
                            None
                        };
                        let time_zone = if let Some(val) = map.get("timeZone") {
                            Some(
                                val.as_str()
                                    .ok_or_else(|| {
                                        de::Error::custom(r#"field "timeZone" is not a string"#)
                                    })?
                                    .to_string(),
                            )
                        } else {
                            None
                        };
                        Ok(Granularity::Period {
                            period: period
                                .as_str()
                                .ok_or_else(|| {
                                    de::Error::custom(r#"field "period" is not a string"#)
                                })?
                                .to_string(),
                            origin,
                            time_zone,
                        })
                    }
                    _ => Err(de::Error::custom(
                        r#"field "type" does not have a recognized value"#,
                    )),
                },
            },
            _ => Err(de::Error::custom(
                r#"value does not have a recognized form"#,
            )),
        }
    }
}

impl Granularity {
    pub fn simple(granulaity: String) -> Self {
        Self::Simple(granulaity)
    }

    pub fn duration(duration: u64) -> Self {
        Self::Duration {
            duration,
            origin: None,
        }
    }

    pub fn duration_with_origin(duration: u64, origin: String) -> Self {
        Self::Duration {
            duration,
            origin: Some(origin),
        }
    }

    pub fn period(period: String) -> Self {
        Self::Period {
            period,
            origin: None,
            time_zone: None,
        }
    }

    pub fn period_with_origin_tz(
        period: String,
        origin: Option<String>,
        time_zone: Option<String>,
    ) -> Self {
        Self::Period {
            period,
            origin,
            time_zone,
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("invalid simple granularity string {0}")]
pub struct InvalidGranularity(String);

impl FromStr for Granularity {
    type Err = InvalidGranularity;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        let g_str = match lower.as_str() {
            "none" | "all" | "second" | "minute" | "fifteen_minute" | "thirty_minute" | "hour"
            | "day" | "week" | "month" | "quarter" | "year" => s.to_string(),
            _ => return Err(InvalidGranularity(s.to_string())),
        };
        Ok(Granularity::Simple(g_str))
    }
}
