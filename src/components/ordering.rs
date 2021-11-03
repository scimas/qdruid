use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InvalidOrderingError {}

impl Display for InvalidOrderingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"ordering must be one of "lexicographic", "alphanumeric", "strlen" or "numeric""#)
    }
}

impl Error for InvalidOrderingError {}
