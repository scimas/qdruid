#[derive(Debug, Clone)]
pub enum ToInclude {
    All,
    None,
    List { columns: Vec<String> },
}

impl ToInclude {
    pub fn all() -> Self {
        Self::All {}
    }

    pub fn none() -> Self {
        Self::None {}
    }

    pub fn list(columns: Vec<String>) -> Self {
        Self::List { columns }
    }
}
