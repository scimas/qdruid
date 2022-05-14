use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct Interval {
    start: String,
    end: String,
}

impl Interval {
    pub fn new(start: &str, end: &str) -> Result<Self, InvalidISO8601> {
        if &start[..1] == "P" {
            if &end[..1] == "P" {
                Err(InvalidISO8601(format!("{start}/{end}")))
            } else {
                Ok(Self {
                    start: start.into(),
                    end: end.into(),
                })
            }
        } else {
            Ok(Self {
                start: start.into(),
                end: end.into(),
            })
        }
    }
}

impl ToString for Interval {
    fn to_string(&self) -> String {
        let mut s = self.start.clone();
        s.push('/');
        s.push_str(&self.end);
        s
    }
}

impl FromStr for Interval {
    type Err = InvalidISO8601;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('/');
        let start = parts.next();
        if let Some(start) = start {
            let end = parts.next();
            if let Some(end) = end {
                return Self::new(start, end);
            }
        }
        Err(InvalidISO8601(s.to_string()))
    }
}

impl Serialize for Interval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Interval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        let mut parts = s.split('/');
        let start = if let Some(start) = parts.next() {
            start.to_string()
        } else {
            return Err(de::Error::custom(r#"interval does not have a start"#));
        };
        let end = if let Some(end) = parts.next() {
            end.to_string()
        } else {
            return Err(de::Error::custom(r#"interval does not have an end"#));
        };
        Ok(Self { start, end })
    }
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("invalid ISO 8601 interval {0}")]
pub struct InvalidISO8601(String);

#[cfg(test)]
mod tests {
    use super::{Interval, InvalidISO8601};

    #[test]
    fn new_both_period_fails() -> Result<(), String> {
        match Interval::new("P1D", "PT3H") {
            Err(InvalidISO8601(_)) => Ok(()),
            _ => Err("did not receive expected error".into()),
        }
    }

    #[test]
    fn new_proper_succeeds() {
        assert!(Interval::new("1", "2").is_ok());
    }

    #[test]
    fn expected_string_repr() {
        let interval = Interval::new("2021-01-01", "2021-01-02T22:00:03").unwrap();
        assert_eq!(
            interval.to_string(),
            "2021-01-01/2021-01-02T22:00:03".to_string()
        );
    }
}
