use reqwest::IntoUrl;

pub struct Client<T: IntoUrl, U: IntoUrl> {
    inner: reqwest::Client,
    native_endpoint: Option<T>,
    sql_endpoint: Option<U>,
}

impl<T: IntoUrl, U: IntoUrl> Client<T, U> {
    fn new(native_endpoint: T, sql_endpoint: U) -> Self {
        Self {
            inner: reqwest::Client::new(),
            native_endpoint: Some(native_endpoint),
            sql_endpoint: Some(sql_endpoint),
        }
    }

    fn new_with_native_endpoint(native_endpoint: T) -> Self {
        Self {
            inner: reqwest::Client::new(),
            native_endpoint: Some(native_endpoint),
            sql_endpoint: None,
        }
    }

    fn new_with_sql_endpoint(sql_endpoint: U) -> Self {
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
        let new_client = Client::new("http://localhost:8888/druid/v2", "http://localhost:8888/druid/v2/sql");
        let client = Client {
            inner: reqwest::Client::new(),
            native_endpoint: Some("http://localhost:8888/druid/v2"),
            sql_endpoint: Some("http://localhost:8888/druid/v2/sql"),
        };
        assert_eq!(new_client.native_endpoint, client.native_endpoint);
        assert_eq!(new_client.sql_endpoint, client.sql_endpoint);
    }
}
