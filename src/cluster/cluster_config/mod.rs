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

  // connection fields
  pub keyspace: Option<String>,
  pub auth: Option<Auth>,
  pub ssl: Option<Ssl>,
}
