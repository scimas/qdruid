#[derive(Debug, Clone)]
pub enum ToInclude {
    All,
    None,
    List {
        columns: Vec<String>,
    }
}