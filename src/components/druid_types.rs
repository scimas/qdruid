pub enum DruidType<C> {
    String(String),
    Double(f64),
    Float(f32),
    Long(i64),
    Complex(C),
}

impl<C> DruidType<C> {
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

    fn complex(c: C) -> Self {
        Self::Complex(c)
    }
}
