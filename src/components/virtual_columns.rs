#[derive(Debug, Clone)]
pub enum VirtaulColumn {
    Expression {
        name: String,
        expression: String,
        output_type: Option<String>,
    },
}
