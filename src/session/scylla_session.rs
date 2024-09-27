use crate::helpers::query_parameter::QueryParameter;
use crate::helpers::query_results::{JSQueryResult, QueryResult};
use crate::query::batch_statement::ScyllaBatchStatement;
use crate::query::scylla_prepared_statement::PreparedStatement;
use crate::query::scylla_query::Query;
use crate::types::decimal::Decimal;
use crate::types::duration::Duration;
use crate::types::uuid::Uuid;
use napi::bindgen_prelude::{BigInt, Either3, Either6, Either7};
use napi::Either;
use std::collections::HashMap;

use super::metrics;
use super::topology::ScyllaClusterData;

#[napi(object)]
pub struct QueryOptions {
  pub prepare: Option<bool>,
}

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
  pub async fn get_cluster_data(&self) -> ScyllaClusterData {
    self
      .session
      .refresh_metadata()
      .await
      .expect("Failed to refresh metadata");

    let cluster_data = self.session.get_cluster_data();
    cluster_data.into()
  }

  /// Sends a query to the database and receives a response.\
  /// Returns only a single page of results, to receive multiple pages use (TODO: Not implemented yet)
  ///
  /// This is the easiest way to make a query, but performance is worse than that of prepared queries.
  ///
  /// It is discouraged to use this method with non-empty values argument. In such case, query first needs to be prepared (on a single connection), so
  /// driver will perform 2 round trips instead of 1. Please use `PreparedStatement` object or `{ prepared: true }` option instead.
  ///
  /// # Notes
  ///
  /// ## UDT
  /// Order of fields in the object must match the order of fields as defined in the UDT. The
  /// driver does not check it by itself, so incorrect data will be written if the order is
  /// wrong.
  #[allow(clippy::type_complexity)]
  #[napi]
  pub async fn execute(
    &self,
    query: Either3<String, &Query, &PreparedStatement>,
    parameters: Option<
      Vec<
        Either7<
          u32,
          String,
          &Uuid,
          BigInt,
          &Duration,
          &Decimal,
          HashMap<String, Either6<u32, String, &Uuid, BigInt, &Duration, &Decimal>>,
        >,
      >,
    >,
    options: Option<QueryOptions>,
  ) -> JSQueryResult {
    let values = QueryParameter::parser(parameters.clone()).ok_or_else(|| {
      napi::Error::new(
        napi::Status::InvalidArg,
        format!(
          "Something went wrong with your query parameters. {:?}",
          parameters
        ),
      )
    })?;

    let should_prepare = options.map_or(false, |options| options.prepare.unwrap_or(false));

    match query {
      Either3::A(ref query_str) if should_prepare => {
        let prepared = self.session.prepare(query_str.clone()).await.map_err(|e| {
          napi::Error::new(
            napi::Status::InvalidArg,
            format!(
              "Something went wrong preparing your statement. - [{}]\n{}",
              query_str, e
            ),
          )
        })?;
        self.execute_prepared(&prepared, values, query_str).await
      }
      Either3::A(query_str) => self.execute_query(Either::A(query_str), values).await,
      Either3::B(query_ref) => {
        self
          .execute_query(Either::B(query_ref.query.clone()), values)
          .await
      }
      Either3::C(prepared_ref) => {
        self
          .execute_prepared(
            &prepared_ref.prepared,
            values,
            prepared_ref.prepared.get_statement(),
          )
          .await
      }
    }
  }

  // Helper method to handle prepared statements
  async fn execute_prepared(
    &self,
    prepared: &scylla::prepared_statement::PreparedStatement,
    values: QueryParameter<'_>,
    query: &str,
  ) -> JSQueryResult {
    let query_result = self.session.execute(prepared, values).await.map_err(|e| {
      napi::Error::new(
        napi::Status::InvalidArg,
        format!(
          "Something went wrong with your prepared statement. - [{}]\n{}",
          query, e
        ),
      )
    })?;
    QueryResult::parser(query_result)
  }

  // Helper method to handle direct queries
  async fn execute_query(
    &self,
    query: Either<String, scylla::query::Query>,
    values: QueryParameter<'_>,
  ) -> JSQueryResult {
    let query_result = match &query {
      Either::A(query_str) => self.session.query(query_str.clone(), values).await,
      Either::B(query_ref) => self.session.query(query_ref.clone(), values).await,
    }
    .map_err(|e| {
      let query_str = match query {
        Either::A(query_str) => query_str,
        Either::B(query_ref) => query_ref.contents.clone(),
      };
      napi::Error::new(
        napi::Status::InvalidArg,
        format!(
          "Something went wrong with your query. - [{}]\n{}",
          query_str, e
        ),
      )
    })?;

    QueryResult::parser(query_result)
  }

  #[allow(clippy::type_complexity)]
  #[napi]
  pub async fn query(
    &self,
    scylla_query: &Query,
    parameters: Option<
      Vec<
        Either7<
          u32,
          String,
          &Uuid,
          BigInt,
          &Duration,
          &Decimal,
          HashMap<String, Either6<u32, String, &Uuid, BigInt, &Duration, &Decimal>>,
        >,
      >,
    >,
  ) -> JSQueryResult {
    let values = QueryParameter::parser(parameters.clone()).ok_or(napi::Error::new(
      napi::Status::InvalidArg,
      format!("Something went wrong with your query parameters. {parameters:?}"),
    ))?;

    let query_result = self
      .session
      .query(scylla_query.query.clone(), values)
      .await
      .map_err(|e| {
        napi::Error::new(
          napi::Status::InvalidArg,
          format!("Something went wrong with your query. - [{scylla_query}] - {parameters:?}\n{e}"),
        )
      })?;

    QueryResult::parser(query_result)
  }

  #[napi]
  pub async fn prepare(&self, query: String) -> napi::Result<PreparedStatement> {
    let prepared = self.session.prepare(query.clone()).await.map_err(|e| {
      napi::Error::new(
        napi::Status::InvalidArg,
        format!("Something went wrong with your prepared statement. - [{query}]\n{e}"),
      )
    })?;

    Ok(PreparedStatement::new(prepared))
  }

  /// Perform a batch query\
  /// Batch contains many `simple` or `prepared` queries which are executed at once\
  /// Batch doesn't return any rows
  ///
  /// Batch values must contain values for each of the queries
  ///
  /// See [the book](https://rust-driver.docs.scylladb.com/stable/queries/batch.html) for more information
  ///
  /// # Arguments
  /// * `batch` - Batch to be performed
  /// * `values` - List of values for each query, it's the easiest to use an array of arrays
  ///
  /// # Example
  /// ```javascript
  /// const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];
  ///
  /// const cluster = new Cluster({ nodes });
  /// const session = await cluster.connect();
  ///
  /// const batch = new BatchStatement();
  ///
  /// await session.execute("CREATE KEYSPACE IF NOT EXISTS batch_statements WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 }");
  /// await session.useKeyspace("batch_statements");
  /// await session.execute("CREATE TABLE IF NOT EXISTS users (id UUID PRIMARY KEY, name TEXT)");
  ///
  /// const simpleStatement = new Query("INSERT INTO users (id, name) VALUES (?, ?)");
  /// const preparedStatement = await session.prepare("INSERT INTO users (id, name) VALUES (?, ?)");
  ///
  /// batch.appendStatement(simpleStatement);
  /// batch.appendStatement(preparedStatement);
  ///
  /// await session.batch(batch, [[Uuid.randomV4(), "Alice"], [Uuid.randomV4(), "Bob"]]);
  ///
  /// console.log(await session.execute("SELECT * FROM users"));
  /// ```
  #[napi]
  #[allow(clippy::type_complexity)]
  pub async fn batch(
    &self,
    batch: &ScyllaBatchStatement,
    parameters: Vec<
      Option<
        Vec<
          Either7<
            u32,
            String,
            &Uuid,
            BigInt,
            &Duration,
            &Decimal,
            HashMap<String, Either6<u32, String, &Uuid, BigInt, &Duration, &Decimal>>,
          >,
        >,
      >,
    >,
  ) -> JSQueryResult {
    let values = parameters
      .iter()
      .map(|params| {
        QueryParameter::parser(params.clone()).ok_or(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Something went wrong with your batch parameters. {parameters:?}"),
        ))
      })
      .collect::<napi::Result<Vec<_>>>()?;

    let query_result = self
      .session
      .batch(&batch.batch, values)
      .await
      .map_err(|e| {
        napi::Error::new(
          napi::Status::InvalidArg,
          format!("Something went wrong with your batch. - [{batch}] - {parameters:?}\n{e}"),
        )
      })?;

    QueryResult::parser(query_result)
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
  ///   .catch(console.error);
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
