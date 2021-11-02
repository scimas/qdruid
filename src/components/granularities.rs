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
