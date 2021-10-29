use std::{error::Error, fmt::Display, str::FromStr};

pub struct Interval {
    start: String,
    end: String,
}

impl Interval {
    fn new(start: &str, end: &str) -> Result<Self, InvalidISO8601> {
        if &start[..1] == "P" {
            if &end[..1] == "P" {
                return Err(InvalidISO8601);
            } else {
                return Ok(Self {
                    start: start.into(),
                    end: end.into(),
                });
            }
        } else {
            return Ok(Self {
                start: start.into(),
                end: end.into(),
            });
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
        return Err(Self::Err {});
    }
}

#[derive(Debug)]
struct InvalidISO8601;

impl Display for InvalidISO8601 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid ISO 8601 interval")
    }
}

impl Error for InvalidISO8601 {}

#[cfg(test)]
mod tests {
    use super::{Interval, InvalidISO8601};

    #[test]
    fn new_both_period_fails() -> Result<(), String> {
        match Interval::new("P1D", "PT3H") {
            Err(InvalidISO8601) => Ok(()),
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
