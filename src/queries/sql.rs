use crate::components::{context::Context, druid_types::DruidSqlType};

#[derive(Debug, Clone)]
pub struct Sql {
    query: String,
    result_format: Option<String>,
    header: Option<bool>,
    parameters: Option<Vec<DruidSqlType>>,
    context: Option<Context>,
}

impl Sql {
    fn new(query: &str) -> Self {
        Self {
            query: query.into(),
            result_format: None,
            header: None,
            parameters: None,
            context: None,
        }
    }

    fn result_format(&mut self, result_format: &str) -> &mut Self {
        self.result_format = Some(result_format.into());
        self
    }

    fn header(&mut self, header: bool) -> &mut Self {
        self.header = Some(header);
        self
    }

    fn parameters(&mut self, parameters: &[DruidSqlType]) -> &mut Self {
        self.parameters = Some(parameters.to_vec());
        self
    }

    fn context(&mut self, context: Context) -> &mut Self {
        self.context = Some(context);
        self
    }
}
