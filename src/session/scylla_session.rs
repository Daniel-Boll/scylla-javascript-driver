use crate::helpers::query_parameter::QueryParameter;
use crate::helpers::query_results::QueryResult;
use crate::query::scylla_prepared_statement::ScyllaPreparedStatement;
use crate::query::scylla_query::ScyllaQuery;
use crate::types::uuid::Uuid;
use napi::bindgen_prelude::{Either, Either3};

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
    query: Either<String, &ScyllaPreparedStatement>,
    parameters: Option<Vec<Either3<u32, String, &Uuid>>>,
  ) -> napi::Result<serde_json::Value> {
    let values = QueryParameter::parser(parameters.clone()).ok_or(napi::Error::new(
      napi::Status::InvalidArg,
      format!("Something went wrong with your query parameters. {parameters:?}"),
    ))?;

    let query_result = match query.clone() {
      Either::A(query) => self.session.query(query, values).await,
      Either::B(prepared) => self.session.execute(&prepared.prepared, values).await,
    }
    .map_err(|_| {
      let query = match query {
        Either::A(query) => query,
        Either::B(prepared) => prepared.prepared.get_statement().to_string(),
      };

      napi::Error::new(
        napi::Status::InvalidArg,
        format!("Something went wrong with your query. - [{query}] - {parameters:?}"),
      )
    })?;

    Ok(QueryResult::parser(query_result))
  }

  #[napi]
  pub async fn query(
    &self,
    scylla_query: &ScyllaQuery,
    parameters: Option<Vec<Either3<u32, String, &Uuid>>>,
  ) -> napi::Result<serde_json::Value> {
    let values = QueryParameter::parser(parameters.clone()).ok_or(napi::Error::new(
      napi::Status::InvalidArg,
      format!("Something went wrong with your query parameters. {parameters:?}"),
    ))?;

    let query_result = self
      .session
      .query(scylla_query.query.clone(), values)
      .await
      .map_err(|_| {
        napi::Error::new(
          napi::Status::InvalidArg,
          format!("Something went wrong with your query. - [{scylla_query}] - {parameters:?}"),
        )
      })?;

    Ok(QueryResult::parser(query_result))
  }

  #[napi]
  pub async fn prepare(&self, query: String) -> napi::Result<ScyllaPreparedStatement> {
    let prepared = self.session.prepare(query.clone()).await.map_err(|_| {
      napi::Error::new(
        napi::Status::InvalidArg,
        format!("Something went wrong with your prepared statement. - [{query}]"),
      )
    })?;

    Ok(ScyllaPreparedStatement::new(prepared))
  }
}
