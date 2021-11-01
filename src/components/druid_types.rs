#[derive(Debug, Clone)]
pub enum DruidNativeType {
    String(String),
    Double(f64),
    Float(f32),
    Long(i64),
}

impl DruidNativeType {
    fn string<T: Into<String>>(s: T) -> Self {
        Self::String(s.into())
    }

    fn double<T: Into<f64>>(d: T) -> Self {
        Self::Double(d.into())
    }

    fn float<T: Into<f32>>(f: T) -> Self {
        Self::Float(f.into())
    }

    fn long<T: Into<i64>>(l: T) -> Self {
        Self::Long(l.into())
    }
}

#[derive(Debug, Clone)]
pub enum DruidSqlType {
    Char(char),
    Varchar(String),
    Decimal(f64),
    Float(f32),
    Real(f64),
    Double(f64),
    Boolean(bool),
    Tinyint(i64),
    Smallint(i64),
    Integer(i64),
    Bigint(i64),
    Timestamp(String),
    Date(String),
}

impl DruidSqlType {
    fn char<T: Into<char>>(c: T) -> Self {
        Self::Char(c.into())
    }

    fn varchar<T: Into<String>>(s: T) -> Self {
        Self::Varchar(s.into())
    }

    fn decimal<T: Into<f64>>(d: T) -> Self {
        Self::Decimal(d.into())
    }

    fn float<T: Into<f32>>(d: T) -> Self {
        Self::Float(d.into())
    }

    fn real<T: Into<f64>>(d: T) -> Self {
        Self::Real(d.into())
    }

    fn double<T: Into<f64>>(d: T) -> Self {
        Self::Double(d.into())
    }

    fn boolean<T: Into<bool>>(b: T) -> Self {
        Self::Boolean(b.into())
    }

    fn tinyint<T: Into<i64>>(d: T) -> Self {
        Self::Tinyint(d.into())
    }

    fn smallint<T: Into<i64>>(d: T) -> Self {
        Self::Smallint(d.into())
    }

    fn integer<T: Into<i64>>(d: T) -> Self {
        Self::Integer(d.into())
    }

    fn bigint<T: Into<i64>>(d: T) -> Self {
        Self::Bigint(d.into())
    }

    fn timestamp<T: Into<String>>(d: T) -> Self {
        Self::Timestamp(d.into())
    }

    fn date<T: Into<String>>(d: T) -> Self {
        Self::Date(d.into())
    }
}
