# Apache Druid Query Library

A simple library for constructing and executing queries and parsing responses
from Apache Druid.

`use query_druid::prelude::*` for a quick-start with the library.

[`Client`](async_impl::client::Client) provides an HTTP connection to Druid. It
can be used to execute queries.

The library is arranged in two modules, [`components`] and [`queries`].
`components` has all of the Druid native query building blocks like aggregations
and filters in their own modules. `queries` has all types of queries, including
the SQL query, and a [`DruidResponse`](queries::response::DruidResponse) enum
for representing different query and error responses from Druid.

## Usage

### SQL Query
```rust
use std::error::Error;

use query_druid::prelude::{Client, Sql};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::sql_client("http://localhost:8888/druid/v2/sql/".to_string())?;

    let query = Sql::new("SELECT * FROM wikipedia LIMIT 2");

    let result = client.execute(query.into()).await?;

    Ok(())
}
```

### Native Query
```rust
use std::error::Error;

use query_druid::prelude::{Client, Timeseries, Aggregator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::native_client("http://localhost:8888/druid/v2/".to_string())?;

    let aggs = vec![Aggregator::count("num_records".into())];
    let query = Timeseries::new(
        "wikipedia".into(),
        &["2015-09-12/P1D".parse()?],
        "hour".parse()?,
    )
    .aggregations(&aggs);

    let result = client.execute(query.into()).await?;

    Ok(())
}
```
