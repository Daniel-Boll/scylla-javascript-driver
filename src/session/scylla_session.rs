use crate::helpers::query_parameter::QueryParameter;
use crate::helpers::query_results::QueryResult;
use crate::query::scylla_query::ScyllaQuery;
use crate::types::uuid::Uuid;
use napi::bindgen_prelude::Either3;

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
}
