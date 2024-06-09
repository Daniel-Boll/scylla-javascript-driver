use crate::helpers::query_parameter::QueryParameter;
use crate::helpers::query_results::QueryResult;
use crate::query::scylla_query::ScyllaQuery;
use crate::types::uuid::Uuid;
use napi::bindgen_prelude::{Either3, Reference};
use napi::Either;
use scylla::prepared_statement::PreparedStatement;
use scylla::transport::errors::QueryError;
use crate::query::scylla_prepared_statement::ScyllaPreparedStatement;

use super::metrics;

#[napi]
pub struct ScyllaSession {
  session: scylla::Session,
}

#[napi]
impl ScyllaSession {
  pub fn new(session: scylla::Session) -> Self {
    Self { session }
  }

  #[napi]
  pub fn metrics(&self) -> metrics::Metrics {
    metrics::Metrics::new(self.session.get_metrics())
  }

  #[napi]
  pub async fn execute(
    &self,
    query: String,
    parameters: Option<Vec<Either3<u32, String, &Uuid>>>,
  ) -> napi::Result<serde_json::Value> {
    let values = QueryParameter::parser(parameters).unwrap();

    let query_result = self.session.query(query, values).await;

    Ok(QueryResult::parser(query_result.unwrap()))
  }

  #[napi]
  pub async fn query(
    &self,
    scylla_query: &ScyllaQuery,
    parameters: Option<Vec<Either3<u32, String, &Uuid>>>,
  ) -> napi::Result<serde_json::Value> {
    let values = QueryParameter::parser(parameters).unwrap();

    let query_result = self.session.query(scylla_query.query.clone(), values).await;

    Ok(QueryResult::parser(query_result.unwrap()))
  }


  #[napi]
  pub async fn execute_prepare(
    &self,
    prepared_statement: Reference<ScyllaPreparedStatement>,
    parameters: Option<Vec<Either3<u32, String, &Uuid>>>,
  ) -> napi::Result<serde_json::Value> {
    let values = QueryParameter::parser(parameters).unwrap();

    let query_result = self.session.execute(&prepared_statement.prepared, values).await;
    // let query_result = match query {
    //   Either::A(query) => self.session.query(query, values).await,
    //   Either::B(prepared_statement) => self.session.execute(&prepared_statement.prepared, values).await
    // };

    Ok(QueryResult::parser(query_result.unwrap()))
  }
  #[napi]
  pub async fn prepare(
    &self,
    query: String
  ) -> napi::Result<ScyllaPreparedStatement> {
    let prepared = self.session.prepare(query).await;

    match prepared {
      Ok(prepared) => Ok(ScyllaPreparedStatement::new(prepared)),
      Err(_) => Err(napi::Error::new(napi::Status::InvalidArg, "Something went wrong with your prepared statement."))
    }
  }
}
