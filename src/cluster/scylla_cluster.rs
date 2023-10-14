use crate::session::scylla_session::ScyllaSession;
use crate::cluster::{
  config::{cluster_config::ClusterConfig, compression::Compression}, 
  execution_profile::ExecutionProfile
};

#[napi(js_name = "Cluster")]
struct ScyllaCluster {
  uri: String,
  compression: Option<Compression>,
  default_execution_profile: Option<ExecutionProfile>,
}

#[napi]
impl ScyllaCluster {
  /// Object config is in the format:
  /// {
  ///     nodes: Array<string>,
  /// }
  #[napi(constructor)]
  pub fn new(cluster_config: ClusterConfig) -> Self {
    let ClusterConfig {
      nodes,
      compression,
      default_execution_profile,
    } = cluster_config;

    let uri = nodes.get(0).expect("at least one node is required");

    Self {
      uri: uri.to_string(),
      compression,
      default_execution_profile,
    }
  }

  /// Connect to the cluster
  #[napi]
  pub async fn connect(&self, keyspace: Option<String>) -> ScyllaSession {
    let mut builder = scylla::SessionBuilder::new().known_node(self.uri.as_str());

    if let Some(keyspace) = keyspace {
      builder = builder.use_keyspace(keyspace, false);
    }

    if let Some(default_execution_profile) = &self.default_execution_profile {
      builder = builder.default_execution_profile_handle(default_execution_profile.into_handle());
    }

    if let Some(compression) = self.compression {
      builder = builder.compression(compression.into());
    }

    ScyllaSession::new(builder.build().await.unwrap())
  }
}
