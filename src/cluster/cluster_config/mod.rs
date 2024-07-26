use crate::cluster::{
  cluster_config::compression::Compression,
  execution_profile::ExecutionProfile,
  scylla_cluster::{Auth, Ssl},
};

pub mod compression;

#[napi(object)]
pub struct ClusterConfig {
  pub nodes: Vec<String>,
  pub compression: Option<Compression>,
  pub default_execution_profile: Option<ExecutionProfile>,

  pub keyspace: Option<String>,
  pub auth: Option<Auth>,
  pub ssl: Option<Ssl>,

  /// The driver automatically awaits schema agreement after a schema-altering query is executed. Waiting for schema agreement more than necessary is never a bug, but might slow down applications which do a lot of schema changes (e.g. a migration). For instance, in case where somebody wishes to create a keyspace and then a lot of tables in it, it makes sense only to wait after creating a keyspace and after creating all the tables rather than after every query.
  pub auto_await_schema_agreement: Option<bool>,
  /// If the schema is not agreed upon, the driver sleeps for a duration in seconds before checking it again. The default value is 0.2 (200 milliseconds)
  pub schema_agreement_interval: Option<i32>,
}
