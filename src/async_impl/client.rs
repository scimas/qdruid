pub struct Client {
    inner: reqwest::Client,
    native_endpoint: Option<String>,
    sql_endpoint: Option<String>,
}

impl Client {
    pub fn new(native_endpoint: String, sql_endpoint: String) -> Self {
        Self {
            inner: reqwest::Client::new(),
            native_endpoint: Some(native_endpoint),
            sql_endpoint: Some(sql_endpoint),
        }
    }

    pub fn native_client(native_endpoint: String) -> Self {
        Self {
            inner: reqwest::Client::new(),
            native_endpoint: Some(native_endpoint),
            sql_endpoint: None,
        }
    }

    pub fn sql_client(sql_endpoint: String) -> Self {
        Self {
            inner: reqwest::Client::new(),
            native_endpoint: None,
            sql_endpoint: Some(sql_endpoint),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Client;

    #[test]
    fn new_test() {
        let new_client = Client::new(
            "http://localhost:8888/druid/v2".to_string(),
            "http://localhost:8888/druid/v2/sql".to_string(),
        );
        let client = Client {
            inner: reqwest::Client::new(),
            native_endpoint: Some("http://localhost:8888/druid/v2".to_string()),
            sql_endpoint: Some("http://localhost:8888/druid/v2/sql".to_string()),
        };
        assert_eq!(new_client.native_endpoint, client.native_endpoint);
        assert_eq!(new_client.sql_endpoint, client.sql_endpoint);
    }
}
