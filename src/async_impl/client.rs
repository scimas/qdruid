use std::error::Error;

use serde::Serialize;
use serde_json::Value;

use crate::Query;

pub struct Client {
    inner: reqwest::Client,
    native_endpoint: Option<String>,
    sql_endpoint: Option<String>,
}

impl Client {
    pub fn new(native_endpoint: String, sql_endpoint: String) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            inner: reqwest::Client::builder().gzip(true).build()?,
            native_endpoint: Some(native_endpoint),
            sql_endpoint: Some(sql_endpoint),
        })
    }

    pub fn native_client(native_endpoint: String) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            inner: reqwest::Client::builder().gzip(true).build()?,
            native_endpoint: Some(native_endpoint),
            sql_endpoint: None,
        })
    }

    pub fn sql_client(sql_endpoint: String) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            inner: reqwest::Client::builder().gzip(true).build()?,
            native_endpoint: None,
            sql_endpoint: Some(sql_endpoint),
        })
    }

    pub async fn execute(&self, query: Query) -> Result<Value, Box<dyn Error>> {
        match query {
            Query::Sql(q) => match &self.sql_endpoint {
                Some(url) => Ok(self.inner.post(url).json(&q).send().await?.json().await?),
                None => Err("not a SQL client".into()),
            },
            Query::DataSourceMetadata(q) => self.native_execute(q).await,
            Query::GroupBy(q) => self.native_execute(q).await,
            Query::Scan(q) => self.native_execute(q).await,
            Query::Search(q) => self.native_execute(q).await,
            Query::SegmentMetadata(q) => self.native_execute(q).await,
            Query::TimeBoundary(q) => self.native_execute(q).await,
            Query::Timeseries(q) => self.native_execute(q).await,
            Query::TopN(q) => self.native_execute(q).await,
        }
    }

    async fn native_execute<Q: Serialize>(&self, q: Q) -> Result<Value, Box<dyn Error>> {
        match &self.native_endpoint {
            Some(url) => Ok(self.inner.post(url).json(&q).send().await?.json().await?),
            None => Err("not a native query client".into()),
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
        )
        .expect("couldn't create a client");
        let client = Client {
            inner: reqwest::Client::new(),
            native_endpoint: Some("http://localhost:8888/druid/v2".to_string()),
            sql_endpoint: Some("http://localhost:8888/druid/v2/sql".to_string()),
        };
        assert_eq!(new_client.native_endpoint, client.native_endpoint);
        assert_eq!(new_client.sql_endpoint, client.sql_endpoint);
    }
}
