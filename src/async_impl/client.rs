use std::collections::HashMap;

use crate::{
    components::druid_types::DruidNativeType,
    queries::{
        datasource_metadata::DataSourceMetadata,
        groupby::GroupBy,
        response::{
            DataSourceMetadataResult, GroupByResult, QueryError, ScanResult, SearchResult,
            SegmentMetadataResult, SqlResult, TimeBoundaryResult, TimeseriesResult, TopNResult,
        },
        scan::Scan,
        search::Search,
        segment_metadata::SegmentMetadata,
        sql::{ResultFormat, Sql},
        time_boundary::TimeBoundary,
        timeseries::Timeseries,
        topn::TopN,
    },
};

/// An HTTP connector to Druid.
///
/// It can hold two different URLs for using either the native queries or the
/// SQL query.
///
/// Submit a query to Druid and get a parsed response.
///
/// The client uses the appropriate native or SQL query URL depending on the
/// type of the query. It errors if the client is not created with the
/// respective URL.
///
/// A successful query can still result in an error if the received data is
/// corrupted and cannot be deserialized into a known query result format.
///
/// # Examples
///
/// ```no_run
/// # use std::error::Error;
/// use query_druid::prelude::{Client, Sql};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn Error>> {
/// let client = Client::sql_client("http://localhost:8888/druid/v2/sql/".to_string())?;
/// let query = Sql::new("SELECT * FROM wikipedia LIMIT 2");
/// let result = client.sql(query).await?;
/// # Ok(())
/// # }
/// ```
pub struct Client {
    inner: reqwest::Client,
    native_endpoint: Option<String>,
    sql_endpoint: Option<String>,
}

/// An error originating from this library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Client(String),
    #[error("connection error: {0}")]
    Connection(#[from] reqwest::Error),
    #[error("error during decoding response from druid: {0}")]
    ResponseDecode(String),
    #[error("error response from druid {0}")]
    QueryError(QueryError),
}

impl Client {
    /// Create a new `Client` for both native querying and SQL querying.
    pub fn new(native_endpoint: String, sql_endpoint: String) -> Result<Self, Error> {
        if let Ok(inner) = reqwest::Client::builder().gzip(true).build() {
            Ok(Self {
                inner,
                native_endpoint: Some(native_endpoint),
                sql_endpoint: Some(sql_endpoint),
            })
        } else {
            Err(Error::Client("could not create a client".to_string()))
        }
    }

    /// Create a new `Client` for native querying.
    pub fn native_client(native_endpoint: String) -> Result<Self, Error> {
        if let Ok(inner) = reqwest::Client::builder().gzip(true).build() {
            Ok(Self {
                inner,
                native_endpoint: Some(native_endpoint),
                sql_endpoint: None,
            })
        } else {
            Err(Error::Client("could not create a client".to_string()))
        }
    }

    /// Create a new `Client` for SQL querying.
    pub fn sql_client(sql_endpoint: String) -> Result<Self, Error> {
        if let Ok(inner) = reqwest::Client::builder().gzip(true).build() {
            Ok(Self {
                inner,
                native_endpoint: None,
                sql_endpoint: Some(sql_endpoint),
            })
        } else {
            Err(Error::Client("could not create a client".to_string()))
        }
    }

    fn is_native(&self) -> Result<(), Error> {
        self.native_endpoint
            .as_ref()
            .ok_or_else(|| Error::Client("not a native query client".to_string()))
            .map(|_| ())
    }

    fn is_sql(&self) -> Result<(), Error> {
        self.sql_endpoint
            .as_ref()
            .ok_or_else(|| Error::Client("not a SQL client".to_string()))
            .map(|_| ())
    }

    pub async fn datasource_metadata(
        &self,
        q: DataSourceMetadata,
    ) -> Result<Vec<DataSourceMetadataResult>, Error> {
        self.is_native()?;
        let resp = self
            .inner
            .post(self.native_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await?;
        let body = resp.bytes().await?;
        match serde_json::from_slice(&body) {
            Ok(r) => Ok(r),
            Err(_) => match serde_json::from_slice::<QueryError>(&body) {
                Ok(e) => Err(Error::QueryError(e)),
                Err(_) => Err(Error::ResponseDecode(
                    "response is not valid JSON".to_string(),
                )),
            },
        }
    }

    pub async fn groupby(&self, q: GroupBy) -> Result<Vec<GroupByResult>, Error> {
        self.is_native()?;
        let resp = self
            .inner
            .post(self.native_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await?;
        let body = resp.bytes().await?;
        match serde_json::from_slice(&body) {
            Ok(r) => Ok(r),
            Err(_) => match serde_json::from_slice::<QueryError>(&body) {
                Ok(e) => Err(Error::QueryError(e)),
                Err(_) => Err(Error::ResponseDecode(
                    "response is not valid JSON".to_string(),
                )),
            },
        }
    }

    pub async fn scan(&self, q: Scan) -> Result<Vec<ScanResult>, Error> {
        self.is_native()?;
        let resp = self
            .inner
            .post(self.native_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await?;
        let body = resp.bytes().await?;
        match serde_json::from_slice(&body) {
            Ok(r) => Ok(r),
            Err(_) => match serde_json::from_slice::<QueryError>(&body) {
                Ok(e) => Err(Error::QueryError(e)),
                Err(_) => Err(Error::ResponseDecode(
                    "response is not valid JSON".to_string(),
                )),
            },
        }
    }

    pub async fn search(&self, q: Search) -> Result<Vec<SearchResult>, Error> {
        self.is_native()?;
        let resp = self
            .inner
            .post(self.native_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await?;
        let body = resp.bytes().await?;
        match serde_json::from_slice(&body) {
            Ok(r) => Ok(r),
            Err(_) => match serde_json::from_slice::<QueryError>(&body) {
                Ok(e) => Err(Error::QueryError(e)),
                Err(_) => Err(Error::ResponseDecode(
                    "response is not valid JSON".to_string(),
                )),
            },
        }
    }

    pub async fn segment_metadata(
        &self,
        q: SegmentMetadata,
    ) -> Result<Vec<SegmentMetadataResult>, Error> {
        self.is_native()?;
        let resp = self
            .inner
            .post(self.native_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await?;
        let body = resp.bytes().await?;
        match serde_json::from_slice(&body) {
            Ok(r) => Ok(r),
            Err(_) => match serde_json::from_slice::<QueryError>(&body) {
                Ok(e) => Err(Error::QueryError(e)),
                Err(_) => Err(Error::ResponseDecode(
                    "response is not valid JSON".to_string(),
                )),
            },
        }
    }

    pub async fn time_boundary(&self, q: TimeBoundary) -> Result<Vec<TimeBoundaryResult>, Error> {
        self.is_native()?;
        let resp = self
            .inner
            .post(self.native_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await?;
        let body = resp.bytes().await?;
        match serde_json::from_slice(&body) {
            Ok(r) => Ok(r),
            Err(_) => match serde_json::from_slice::<QueryError>(&body) {
                Ok(e) => Err(Error::QueryError(e)),
                Err(_) => Err(Error::ResponseDecode(
                    "response is not valid JSON".to_string(),
                )),
            },
        }
    }

    pub async fn timeseries(&self, q: Timeseries) -> Result<Vec<TimeseriesResult>, Error> {
        self.is_native()?;
        let resp = self
            .inner
            .post(self.native_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await?;
        let body = resp.bytes().await?;
        match serde_json::from_slice(&body) {
            Ok(r) => Ok(r),
            Err(_) => match serde_json::from_slice::<QueryError>(&body) {
                Ok(e) => Err(Error::QueryError(e)),
                Err(_) => Err(Error::ResponseDecode(
                    "response is not valid JSON".to_string(),
                )),
            },
        }
    }

    pub async fn topn(&self, q: TopN) -> Result<Vec<TopNResult>, Error> {
        self.is_native()?;
        let resp = self
            .inner
            .post(self.native_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await?;
        let body = resp.bytes().await?;
        match serde_json::from_slice(&body) {
            Ok(r) => Ok(r),
            Err(_) => match serde_json::from_slice::<QueryError>(&body) {
                Ok(e) => Err(Error::QueryError(e)),
                Err(_) => Err(Error::ResponseDecode(
                    "response is not valid JSON".to_string(),
                )),
            },
        }
    }

    pub async fn sql(&self, q: Sql) -> Result<Vec<SqlResult>, Error> {
        self.is_sql()?;
        let resp = self
            .inner
            .post(self.sql_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await?;
        let text = match resp.text().await {
            Ok(s) => s,
            Err(_) => {
                return Err(Error::ResponseDecode(
                    "response is not valid utf-8".to_string(),
                ))
            }
        };
        match q.result_format {
            None | Some(ResultFormat::Object) | Some(ResultFormat::Array) => {
                match serde_json::from_str(&text) {
                    Ok(r) => Ok(r),
                    Err(_) => match serde_json::from_str::<QueryError>(&text) {
                        Ok(e) => Err(Error::QueryError(e)),
                        Err(_) => Err(Error::ResponseDecode(
                            "response is not valid JSON".to_string(),
                        )),
                    },
                }
            }
            Some(ResultFormat::ObjectLines) => {
                let maybe_objects: Result<Vec<HashMap<String, DruidNativeType>>, _> =
                    text.trim().lines().map(serde_json::from_str).collect();
                match maybe_objects {
                    Ok(v) => Ok(v.into_iter().map(SqlResult::Object).collect()),
                    Err(_) => Err(Error::ResponseDecode(
                        "part of the response is not valid JSON".to_string(),
                    )),
                }
            }
            Some(ResultFormat::ArrayLines) => {
                let maybe_arrays: Result<Vec<Vec<DruidNativeType>>, _> =
                    text.trim().lines().map(serde_json::from_str).collect();
                match maybe_arrays {
                    Ok(v) => Ok(v.into_iter().map(SqlResult::Array).collect()),
                    Err(_) => Err(Error::ResponseDecode(
                        "part of the response is not valid JSON".to_string(),
                    )),
                }
            }
            Some(ResultFormat::Csv) => Ok(text
                .lines()
                .map(|line| SqlResult::Csv(line.to_string()))
                .collect()),
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
