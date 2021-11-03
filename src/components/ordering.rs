use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InvalidOrderingError {
    what: String,
    should_be: String,
}

impl InvalidOrderingError {
    pub fn new(what: String, should_be: String) -> Self {
        Self { what, should_be }
    }
}
impl Display for InvalidOrderingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"{} is not a valid ordering, should be {}"#,
            self.what, self.should_be
        )
    }
}

impl Error for InvalidOrderingError {}
