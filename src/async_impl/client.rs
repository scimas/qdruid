use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    components::druid_types::DruidNativeType,
    queries::{
        response::{DruidResponse, SqlResult},
        sql::{ResultFormat, Sql},
        Query,
    },
};

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
}

impl Client {
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

    pub async fn execute(&self, query: Query) -> Result<DruidResponse, Error> {
        match query {
            Query::Sql(q) => self.sql_execute(q).await,
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

    async fn native_execute<Q: Serialize>(&self, q: Q) -> Result<DruidResponse, Error> {
        match &self.native_endpoint {
            Some(url) => match self.inner.post(url).json(&q).send().await {
                Ok(resp) => match resp.json().await {
                    Ok(j) => Ok(j),
                    Err(_) => Err(Error::DecodeResponse(
                        "response is not valid JSON".to_string(),
                    )),
                },
                Err(e) => {
                    if let Some(url) = e.url() {
                        Err(Error::Connection(format!(
                            "could not connect to {:?}",
                            url.as_str()
                        )))
                    } else {
                        Err(Error::Connection("could not connect to druid".to_string()))
                    }
                }
            },
            None => Err(Error::Client("not a native query client".to_string())),
        }
    }

    async fn sql_execute(&self, q: Sql) -> Result<DruidResponse, Error> {
        match &self.sql_endpoint {
            Some(url) => match self.inner.post(url).json(&q).send().await {
                Ok(resp) => match q.result_format {
                    None | Some(ResultFormat::Object) | Some(ResultFormat::Array) => {
                        match resp.json().await {
                            Ok(j) => Ok(j),
                            Err(_) => Err(Error::DecodeResponse(
                                "response is not valid JSON".to_string(),
                            )),
                        }
                    }
                    Some(ResultFormat::ObjectLines) => match resp.text().await {
                        Ok(s) => {
                            let maybe_objects: Result<Vec<HashMap<String, DruidNativeType>>, _> = s
                                .trim()
                                .lines()
                                .map(|line| serde_json::from_str(line))
                                .collect();
                            match maybe_objects {
                                Ok(v) => Ok(DruidResponse::Sql(
                                    v.into_iter().map(|h| SqlResult::Object(h)).collect(),
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
                            let maybe_arrays: Result<Vec<Vec<DruidNativeType>>, _> = s
                                .trim()
                                .lines()
                                .map(|line| serde_json::from_str(line))
                                .collect();
                            match maybe_arrays {
                                Ok(v) => Ok(DruidResponse::Sql(
                                    v.into_iter().map(|val| SqlResult::Array(val)).collect(),
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
                Err(e) => {
                    if let Some(url) = e.url() {
                        Err(Error::Connection(format!(
                            "could not connect to {:?}",
                            url.as_str()
                        )))
                    } else {
                        Err(Error::Connection("could not connect to druid".to_string()))
                    }
                }
            },
            None => Err(Error::Client("not a SQL client".to_string())),
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
