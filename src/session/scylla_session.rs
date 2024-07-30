use std::collections::HashMap;
use std::sync::Arc;

use crate::helpers::query_parameter::QueryParameter;
use crate::helpers::query_results::QueryResult;
use crate::query::batch_statement::ScyllaBatchStatement;
use crate::query::scylla_prepared_statement::PreparedStatement;
use crate::query::scylla_query::Query;
use crate::types::uuid::Uuid;
use napi::bindgen_prelude::{Either3, FromNapiValue};
use scylla::transport::topology::{Column, Keyspace, MaterializedView, Table, UserDefinedType};
use scylla::transport::ClusterData;

use super::metrics;

#[napi]
pub struct ScyllaSession {
  session: scylla::Session,
}

#[napi]
pub struct ClusterDataSimplified {
  inner: Arc<ClusterData>,
}

impl From<Arc<ClusterData>> for ClusterDataSimplified {
  fn from(cluster_data: Arc<ClusterData>) -> Self {
    ClusterDataSimplified {
      inner: cluster_data,
    }
  }
}

#[napi]
impl ClusterDataSimplified {
  #[napi]
  pub fn get_keyspace_info(&self) -> Option<HashMap<String, KeyspaceSimplified>> {
    let keyspaces_info = self.inner.get_keyspace_info();

    if keyspaces_info.is_empty() {
      None
    } else {
      Some(
        keyspaces_info
          .into_iter()
          .map(|(k, v)| (k.clone(), KeyspaceSimplified::from((*v).clone())))
          .collect(),
      )
    }
  }
}

impl From<Keyspace> for KeyspaceSimplified {
  fn from(keyspace: Keyspace) -> Self {
    // filter to have only the table basic
    let mut keyspace_tables = HashMap::new();

    for (table_name, table_info) in keyspace.tables.iter() {
      if table_name != "basic" {
        continue;
      }
      keyspace_tables.insert(table_name.clone(), table_info.clone());
    }

    keyspace_tables.iter().for_each(|(table_name, table_info)| {
      println!("  Table: {}", table_name);
      // table_info
      //   .columns
      //   .iter()
      //   .for_each(|(column_name, column_info)| {
      //     println!("    Column: {}", column_name);
      //     println!("      Type: {:?}", column_info);
      //   });
    });

    KeyspaceSimplified {
      // strategy: format!("{:?}", keyspace.strategy),
      tables: keyspace_tables
        .into_iter()
        .map(|(k, v)| (k, TableSimplified::from(v)))
        .collect(),
      views: keyspace
        .views
        .into_iter()
        .map(|(k, v)| (k, MaterializedViewSimplified::from(v)))
        .collect(),
      // user_defined_types: keyspace.user_defined_types.into_iter().map(|(k, v)| (k, UserDefinedTypeSimplified::from(v))).collect(),
    }
  }
}

#[napi]
#[derive(Clone)]
pub struct KeyspaceSimplified {
  // pub strategy: String,
  pub tables: HashMap<String, TableSimplified>,
  pub views: HashMap<String, MaterializedViewSimplified>,
  // pub user_defined_types: HashMap<String, UserDefinedTypeSimplified>,
}

impl FromNapiValue for MaterializedViewSimplified {
  unsafe fn from_napi_value(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<Self> {
    panic!("Not implemented")
  }
}

// impl From<Keyspace> for KeyspaceSimplified {
//   fn from(keyspace: Keyspace) -> Self {
//     KeyspaceSimplified {
//       // strategy: format!("{:?}", keyspace.strategy),
//       tables: keyspace
//         .tables
//         .into_iter()
//         .map(|(k, v)| (k, TableSimplified::from(v)))
//         .collect(),
//       views: keyspace
//         .views
//         .into_iter()
//         .map(|(k, v)| (k, ViewSimplified::from(v)))
//         .collect(),
//       // user_defined_types: keyspace.user_defined_types.into_iter().map(|(k, v)| (k, UserDefinedTypeSimplified::from(v))).collect(),
//     }
//   }
// }

#[napi]
#[derive(Clone)]
pub struct TableSimplified {
  pub columns: Vec<String>,
  pub partition_key: Vec<String>,
  pub clustering_key: Vec<String>,
  pub partitioner: Option<String>,
}

impl FromNapiValue for TableSimplified {
  unsafe fn from_napi_value(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<Self> {
    panic!("Not implemented")
  }
}

impl From<Table> for TableSimplified {
  fn from(table: Table) -> Self {
    println!("Table: {:?}", table);
    println!("Columns: {:?}", table.columns.clone().into_iter().map(|(k, _)| k.clone()).collect::<Vec<String>>());
    
    TableSimplified {
      columns: table.columns.clone().into_iter().map(|(k, _)| (k.clone())).collect::<Vec<String>>(),
      partition_key: table.partition_key.clone(),
      clustering_key: table.clustering_key.clone(),
      partitioner: table.partitioner.clone(),
    }
  }
}

// #[napi]
// #[derive(Clone)]
// pub struct ColumnSimplified {
//   pub type_: String,
//   pub kind: String,
// }

// impl From<Column> for ColumnSimplified {
//   fn from(column: Column) -> Self {
//     ColumnSimplified {
//       type_: format!("{:?}", column.type_),
//       kind: format!("{:?}", column.kind),
//     }
//   }
// }

#[napi]
#[derive(Clone)]
pub struct MaterializedViewSimplified {
  pub view_metadata: TableSimplified,
  pub base_table_name: String,
}

impl From<MaterializedView> for MaterializedViewSimplified {
  fn from(view: MaterializedView) -> Self {
    MaterializedViewSimplified {
      view_metadata: TableSimplified::from(view.view_metadata),
      base_table_name: view.base_table_name,
    }
  }
}

// #[napi]
// pub struct UserDefinedTypeSimplified {
//     pub name: String,
//     pub keyspace: String,
//     // pub field_types: Vec<(String, String)>, // Simplified for the example
// }

// impl From<Arc<UserDefinedType>> for UserDefinedTypeSimplified {
//     fn from(udt: Arc<UserDefinedType>) -> Self {
//         UserDefinedTypeSimplified {
//             name: udt.name.clone(),
//             keyspace: udt.keyspace.clone(),
//             // field_types: udt.field_types.iter().map(|(k, v)| (k.clone(), format!("{:?}", v))).collect(),
//         }
//     }
// }

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
  pub async fn get_cluster_data(&self) -> ClusterDataSimplified {
    self
      .session
      .refresh_metadata()
      .await
      .expect("Failed to refresh metadata");

    let cluster_data: Arc<ClusterData> = self.session.get_cluster_data();
    // ClusterDataSimplified::from(cluster_data)
    cluster_data.into()
  }

  #[napi]
  pub async fn execute(
    &self,
    query: Either3<String, &Query, &PreparedStatement>,
    parameters: Option<Vec<Either3<u32, String, &Uuid>>>,
  ) -> napi::Result<serde_json::Value> {
    let values = QueryParameter::parser(parameters.clone()).ok_or(napi::Error::new(
      napi::Status::InvalidArg,
      format!("Something went wrong with your query parameters. {parameters:?}"),
    ))?;

    let query_result = match query.clone() {
      Either3::A(query) => self.session.query(query, values).await,
      Either3::B(query) => self.session.query(query.query.clone(), values).await,
      Either3::C(prepared) => self.session.execute(&prepared.prepared, values).await,
    }
    .map_err(|_| {
      let query = match query {
        Either3::A(query) => query,
        Either3::B(query) => query.query.contents.clone(),
        Either3::C(prepared) => prepared.prepared.get_statement().to_string(),
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
    scylla_query: &Query,
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
  pub async fn prepare(&self, query: String) -> napi::Result<PreparedStatement> {
    let prepared = self.session.prepare(query.clone()).await.map_err(|_| {
      napi::Error::new(
        napi::Status::InvalidArg,
        format!("Something went wrong with your prepared statement. - [{query}]"),
      )
    })?;

    Ok(PreparedStatement::new(prepared))
  }

  #[napi]
  #[allow(clippy::type_complexity)]
  pub async fn batch(
    &self,
    batch: &ScyllaBatchStatement,
    parameters: Vec<Option<Vec<Either3<u32, String, &Uuid>>>>,
  ) -> napi::Result<serde_json::Value> {
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

    Ok(QueryResult::parser(query_result))
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
