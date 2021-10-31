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

impl Granularity {
    fn simple(granulaity: &str) -> Self {
        Self::Simple(granulaity.into())
    }

    fn duration(duration: u64) -> Self {
        Self::Duration {
            duration,
            origin: None,
        }
    }

    fn duration_with_origin(duration: u64, origin: &str) -> Self {
        Self::Duration {
            duration,
            origin: Some(origin.into()),
        }
    }

    fn period(period: &str) -> Self {
        Self::Period {
            period: period.into(),
            origin: None,
            time_zone: None,
        }
    }

    fn period_with_origin_tz(
        period: &str,
        origin: Option<String>,
        time_zone: Option<String>,
    ) -> Self {
        Self::Period {
            period: period.into(),
            origin,
            time_zone,
        }
    }
}
