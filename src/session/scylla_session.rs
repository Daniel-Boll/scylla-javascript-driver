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

  /// Sends `USE <keyspace_name>` request on all connections\
  /// This allows to write `SELECT * FROM table` instead of `SELECT * FROM keyspace.table`\
  ///
  /// Note that even failed `useKeyspace` can change currently used keyspace - the request is sent on all connections and
  /// can overwrite previously used keyspace.
  ///
  /// Call only one `useKeyspace` at a time.\
  /// Trying to do two `useKeyspace` requests simultaneously with different names
  /// can end with some connections using one keyspace and the rest using the other.
  ///
  /// # Arguments
  ///
  /// * `keyspaceName` - keyspace name to use,
  /// keyspace names can have up to 48 alphanumeric characters and contain underscores
  /// * `caseSensitive` - if set to true the generated query will put keyspace name in quotes
  ///
  /// # Errors
  ///
  /// * `InvalidArg` - if the keyspace name is invalid
  ///
  /// # Example
  ///
  /// ```javascript
  /// import { Cluster } from ".";
  ///
  /// const cluster = new Cluster({
  ///   nodes: ["127.0.0.1:9042"],
  /// });
  ///
  /// const session = await cluster.connect();
  ///
  /// await session.useKeyspace("system_schema");
  ///
  /// const result = await session
  ///   .execute("SELECT * FROM scylla_tables limit ?", [1])
  ///   .catch((err) => console.error(err));
  /// ```
  #[napi]
  pub async fn use_keyspace(
    &self,
    keyspace_name: String,
    case_sensitive: Option<bool>,
  ) -> napi::Result<()> {
    self
      .session
      .use_keyspace(keyspace_name.clone(), case_sensitive.unwrap_or(false))
      .await
      .map_err(|e| {
        napi::Error::new(
          napi::Status::InvalidArg,
          format!("Something went wrong with your keyspace. - [{keyspace_name}]\n{e}"),
        )
      })?;

    Ok(())
  }

  /// session.awaitSchemaAgreement returns a Promise that can be awaited as long as schema is not in an agreement.
  /// However, it won’t wait forever; ClusterConfig defines a timeout that limits the time of waiting. If the timeout elapses,
  /// the return value is an error, otherwise it is the schema_version.
  ///
  /// # Returns
  ///
  /// * `Promise<Uuid>` - schema_version
  ///
  /// # Errors
  /// * `GenericFailure` - if the timeout elapses
  ///
  /// # Example
  /// ```javascript
  /// import { Cluster } from ".";
  ///
  /// const cluster = new Cluster({ nodes: ["127.0.0.1:9042"] });
  /// const session = await cluster.connect();
  ///
  /// const schemaVersion = await session.awaitSchemaAgreement().catch(console.error);
  /// console.log(schemaVersion);
  ///
  /// const isAgreed = await session.checkSchemaAgreement().catch(console.error);
  /// console.log(isAgreed);
  /// ```
  #[napi]
  pub async fn await_schema_agreement(&self) -> napi::Result<Uuid> {
    Ok(
      self
        .session
        .await_schema_agreement()
        .await
        .map_err(|e| {
          napi::Error::new(
            napi::Status::GenericFailure,
            format!("Something went wrong with your schema agreement. - {e}"),
          )
        })?
        .into(),
    )
  }

  #[napi]
  pub async fn check_schema_agreement(&self) -> napi::Result<bool> {
    Ok(
      self
        .session
        .check_schema_agreement()
        .await
        .map_err(|e| {
          napi::Error::new(
            napi::Status::GenericFailure,
            format!("Something went wrong with your schema agreement. - {e}"),
          )
        })?
        .is_some(),
    )
  }
}
