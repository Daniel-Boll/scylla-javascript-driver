use crate::session::scylla_session::ScyllaSession;

use super::cluster_config::ClusterConfig;

#[napi(js_name = "Cluster")]
struct ScyllaCluster {
  uri: String,
}

#[napi]
impl ScyllaCluster {
  /// Object config is in the format:
  /// {
  ///     nodes: Array<string>,
  /// }
  #[napi(constructor)]
  pub fn new(object_config: ClusterConfig) -> Self {
    let nodes = object_config.nodes;

    let uri = nodes.get(0).expect("at least one node is required");

    Self {
      uri: uri.to_string(),
    }
  }

  /// Connect to the cluster
  #[napi]
  pub async fn connect(&self, keyspace: Option<String>) -> ScyllaSession {
    let mut builder = scylla::SessionBuilder::new().known_node(self.uri.as_str());

    if let Some(keyspace) = keyspace {
      builder = builder.use_keyspace(keyspace, false);
    }

    ScyllaSession::new(builder.build().await.unwrap())
  }
}
