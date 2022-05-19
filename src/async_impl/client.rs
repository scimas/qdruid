use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    components::druid_types::DruidNativeType,
    queries::{
        datasource_metadata::DataSourceMetadata,
        groupby::GroupBy,
        response::{
            DataSourceMetadataResult, DruidResponse, GroupByResult, ScanResult, SearchResult,
            SegmentMetadataResult, SqlResult, TimeBoundaryResult, TimeseriesResult, TopNResult,
        },
        scan::Scan,
        search::Search,
        segment_metadata::SegmentMetadata,
        sql::{ResultFormat, Sql},
        time_boundary::TimeBoundary,
        timeseries::Timeseries,
        topn::TopN,
        Query,
    },
};

/// An HTTP connector to Druid.
///
/// It can hold two different URLs for using either the native queries or the
/// SQL query.
pub struct Client {
    inner: reqwest::Client,
    native_endpoint: Option<String>,
    sql_endpoint: Option<String>,
}

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Error {
    #[error("{0}")]
    Client(String),
    #[error("error in network communication: {0}")]
    Connection(String),
    #[error("error during decoding response from druid: {0}")]
    DecodeResponse(String),
    #[error("error response from druid {0:?}")]
    QueryError(DruidResponse),
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
    /// # use query_druid::prelude::DruidResponse;
    /// use query_druid::prelude::{Client, Sql};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// let client = Client::sql_client("http://localhost:8888/druid/v2/sql/".to_string())?;
    /// let query = Sql::new("SELECT * FROM wikipedia LIMIT 2");
    /// let result = client.execute(query.into()).await?;
    /// assert!(matches!(result, DruidResponse::Sql(..)));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute(&self, query: Query) -> Result<DruidResponse, Error> {
        let result = match query {
            Query::Sql(q) => self.sql_execute(q).await,
            Query::DataSourceMetadata(q) => self.datasource_metadata_execute(q).await,
            Query::GroupBy(q) => self.groupby_execute(*q).await,
            Query::Scan(q) => self.scan_execute(q).await,
            Query::Search(q) => self.search_execute(q).await,
            Query::SegmentMetadata(q) => self.segment_metadata_execute(q).await,
            Query::TimeBoundary(q) => self.time_boundary_execute(q).await,
            Query::Timeseries(q) => self.timeseries_execute(q).await,
            Query::TopN(q) => self.topn_execute(*q).await,
        };
        match result {
            Err(e) => Err(e),
            Ok(err @ DruidResponse::QueryError { .. }) => Err(Error::QueryError(err)),
            Ok(dr) => Ok(dr),
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

    fn handle_reqwest_connection_error(e: reqwest::Error) -> Error {
        if let Some(url) = e.url() {
            Error::Connection(format!("could not connect to {:?}", url.as_str()))
        } else {
            Error::Connection("could not connect to druid".to_string())
        }
    }

    async fn datasource_metadata_execute(
        &self,
        q: DataSourceMetadata,
    ) -> Result<DruidResponse, Error> {
        self.is_native()?;
        match self
            .inner
            .post(self.native_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await
        {
            Ok(resp) => match resp.json::<Vec<DataSourceMetadataResult>>().await {
                Ok(j) => Ok(DruidResponse::DataSourceMetadata(j)),
                Err(_) => Err(Error::DecodeResponse(
                    "response is not valid JSON".to_string(),
                )),
            },
            Err(e) => Err(Self::handle_reqwest_connection_error(e)),
        }
    }

    async fn groupby_execute(&self, q: GroupBy) -> Result<DruidResponse, Error> {
        self.is_native()?;
        match self
            .inner
            .post(self.native_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await
        {
            Ok(resp) => match resp.json::<Vec<GroupByResult>>().await {
                Ok(j) => Ok(DruidResponse::GroupBy(j)),
                Err(_) => Err(Error::DecodeResponse(
                    "response is not valid JSON".to_string(),
                )),
            },
            Err(e) => Err(Self::handle_reqwest_connection_error(e)),
        }
    }

    async fn scan_execute(&self, q: Scan) -> Result<DruidResponse, Error> {
        self.is_native()?;
        match self
            .inner
            .post(self.native_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await
        {
            Ok(resp) => match resp.json::<Vec<ScanResult>>().await {
                Ok(j) => Ok(DruidResponse::Scan(j)),
                Err(_) => Err(Error::DecodeResponse(
                    "response is not valid JSON".to_string(),
                )),
            },
            Err(e) => Err(Self::handle_reqwest_connection_error(e)),
        }
    }

    async fn search_execute(&self, q: Search) -> Result<DruidResponse, Error> {
        self.is_native()?;
        match self
            .inner
            .post(self.native_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await
        {
            Ok(resp) => match resp.json::<Vec<SearchResult>>().await {
                Ok(j) => Ok(DruidResponse::Search(j)),
                Err(_) => Err(Error::DecodeResponse(
                    "response is not valid JSON".to_string(),
                )),
            },
            Err(e) => Err(Self::handle_reqwest_connection_error(e)),
        }
    }

    async fn segment_metadata_execute(&self, q: SegmentMetadata) -> Result<DruidResponse, Error> {
        self.is_native()?;
        match self
            .inner
            .post(self.native_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await
        {
            Ok(resp) => match resp.json::<Vec<SegmentMetadataResult>>().await {
                Ok(j) => Ok(DruidResponse::SegmentMetadata(j)),
                Err(_) => Err(Error::DecodeResponse(
                    "response is not valid JSON".to_string(),
                )),
            },
            Err(e) => Err(Self::handle_reqwest_connection_error(e)),
        }
    }

    async fn time_boundary_execute(&self, q: TimeBoundary) -> Result<DruidResponse, Error> {
        self.is_native()?;
        match self
            .inner
            .post(self.native_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await
        {
            Ok(resp) => match resp.json::<Vec<TimeBoundaryResult>>().await {
                Ok(j) => Ok(DruidResponse::TimeBoundary(j)),
                Err(_) => Err(Error::DecodeResponse(
                    "response is not valid JSON".to_string(),
                )),
            },
            Err(e) => Err(Self::handle_reqwest_connection_error(e)),
        }
    }

    async fn timeseries_execute(&self, q: Timeseries) -> Result<DruidResponse, Error> {
        self.is_native()?;
        match self
            .inner
            .post(self.native_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await
        {
            Ok(resp) => match resp.json::<Vec<TimeseriesResult>>().await {
                Ok(j) => Ok(DruidResponse::Timeseries(j)),
                Err(_) => Err(Error::DecodeResponse(
                    "response is not valid JSON".to_string(),
                )),
            },
            Err(e) => Err(Self::handle_reqwest_connection_error(e)),
        }
    }

    async fn topn_execute(&self, q: TopN) -> Result<DruidResponse, Error> {
        self.is_native()?;
        match self
            .inner
            .post(self.native_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await
        {
            Ok(resp) => match resp.json::<Vec<TopNResult>>().await {
                Ok(j) => Ok(DruidResponse::TopN(j)),
                Err(_) => Err(Error::DecodeResponse(
                    "response is not valid JSON".to_string(),
                )),
            },
            Err(e) => Err(Self::handle_reqwest_connection_error(e)),
        }
    }

    async fn sql_execute(&self, q: Sql) -> Result<DruidResponse, Error> {
        self.is_sql()?;
        match self
            .inner
            .post(self.sql_endpoint.as_ref().unwrap())
            .json(&q)
            .send()
            .await
        {
            Ok(resp) => match q.result_format {
                None | Some(ResultFormat::Object) | Some(ResultFormat::Array) => {
                    match resp.json::<Vec<SqlResult>>().await {
                        Ok(j) => Ok(DruidResponse::Sql(j)),
                        Err(_) => Err(Error::DecodeResponse(
                            "response is not valid JSON".to_string(),
                        )),
                    }
                }
                Some(ResultFormat::ObjectLines) => match resp.text().await {
                    Ok(s) => {
                        let maybe_objects: Result<Vec<HashMap<String, DruidNativeType>>, _> =
                            s.trim().lines().map(serde_json::from_str).collect();
                        match maybe_objects {
                            Ok(v) => Ok(DruidResponse::Sql(
                                v.into_iter().map(SqlResult::Object).collect(),
                            )),
                            Err(_) => Err(Error::DecodeResponse(
                                "part of the response is not valid JSON".to_string(),
                            )),
                        }
                    }
                    Err(_) => Err(Error::DecodeResponse(
                        "response is not valid utf-8".to_string(),
                    )),
                },
                Some(ResultFormat::ArrayLines) => match resp.text().await {
                    Ok(s) => {
                        let maybe_arrays: Result<Vec<Vec<DruidNativeType>>, _> =
                            s.trim().lines().map(serde_json::from_str).collect();
                        match maybe_arrays {
                            Ok(v) => Ok(DruidResponse::Sql(
                                v.into_iter().map(SqlResult::Array).collect(),
                            )),
                            Err(_) => Err(Error::DecodeResponse(
                                "part of the response is not valid JSON".to_string(),
                            )),
                        }
                    }
                    Err(_) => Err(Error::DecodeResponse(
                        "response is not valid utf-8".to_string(),
                    )),
                },
                Some(ResultFormat::Csv) => match resp.text().await {
                    Ok(csv) => Ok(DruidResponse::Sql(
                        csv.lines()
                            .map(|line| SqlResult::Csv(line.to_string()))
                            .collect(),
                    )),
                    Err(_) => Err(Error::DecodeResponse(
                        "response is not valid utf-8".to_string(),
                    )),
                },
            },
            Err(e) => Err(Self::handle_reqwest_connection_error(e)),
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
