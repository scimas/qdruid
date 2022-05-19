use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DruidNativeType {
    String(String),
    Array(Vec<String>),
    Long(i64),
    Float(f32),
    Double(f64),
}

impl DruidNativeType {
    pub fn string<T: Into<String>>(s: T) -> Self {
        Self::String(s.into())
    }

    pub fn double<T: Into<f64>>(d: T) -> Self {
        Self::Double(d.into())
    }

    pub fn float<T: Into<f32>>(f: T) -> Self {
        Self::Float(f.into())
    }

    pub fn long<T: Into<i64>>(l: T) -> Self {
        Self::Long(l.into())
    }

    pub fn as_str(&self) -> Option<&str> {
        if let Self::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<&[String]> {
        if let Self::Array(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        if let Self::Long(l) = self {
            Some(*l)
        } else {
            None
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        if let Self::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        if let Self::Double(d) = self {
            Some(*d)
        } else if let Self::Float(f) = self {
            Some(f64::from(*f))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "UPPERCASE")]
pub enum DruidSqlType {
    Tinyint(i64),
    Smallint(i64),
    Integer(i64),
    Bigint(i64),
    Boolean(bool),
    Float(f32),
    Double(f64),
    Decimal(f64),
    Real(f64),
    Timestamp(DateTime<Utc>),
    Date(DateTime<Utc>),
    Char(char),
    Varchar(String),
}

impl DruidSqlType {
    pub fn char<T: Into<char>>(c: T) -> Self {
        Self::Char(c.into())
    }

    pub fn varchar<T: Into<String>>(s: T) -> Self {
        Self::Varchar(s.into())
    }

    pub fn decimal<T: Into<f64>>(d: T) -> Self {
        Self::Decimal(d.into())
    }

    pub fn float<T: Into<f32>>(d: T) -> Self {
        Self::Float(d.into())
    }

    pub fn real<T: Into<f64>>(d: T) -> Self {
        Self::Real(d.into())
    }

    pub fn double<T: Into<f64>>(d: T) -> Self {
        Self::Double(d.into())
    }

    pub fn boolean<T: Into<bool>>(b: T) -> Self {
        Self::Boolean(b.into())
    }

    pub fn tinyint<T: Into<i64>>(d: T) -> Self {
        Self::Tinyint(d.into())
    }

    pub fn smallint<T: Into<i64>>(d: T) -> Self {
        Self::Smallint(d.into())
    }

    pub fn integer<T: Into<i64>>(d: T) -> Self {
        Self::Integer(d.into())
    }

    pub fn bigint<T: Into<i64>>(d: T) -> Self {
        Self::Bigint(d.into())
    }

    pub fn timestamp<T: Into<DateTime<Utc>>>(d: T) -> Self {
        Self::Timestamp(d.into())
    }

    pub fn date<T: Into<DateTime<Utc>>>(d: T) -> Self {
        Self::Date(d.into())
    }
}
