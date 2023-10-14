use napi::Either;

use crate::{
  cluster::{
    cluster_config::{compression::Compression, ClusterConfig},
    execution_profile::ExecutionProfile,
  },
  session::scylla_session::ScyllaSession,
};

#[napi(js_name = "Cluster")]
struct ScyllaCluster {
  uri: String,
  compression: Option<Compression>,
  default_execution_profile: Option<ExecutionProfile>,
}

#[napi(object)]
struct ConnectionOptions {
  pub keyspace: Option<String>,
  pub auth: Option<Auth>,
}

#[napi(object)]
struct Auth {
  pub username: String,
  pub password: String,
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
  pub async fn connect(
    &self,
    keyspace_or_options: Option<Either<String, ConnectionOptions>>,
    options: Option<ConnectionOptions>,
  ) -> napi::Result<ScyllaSession> {
    let mut builder = scylla::SessionBuilder::new().known_node(self.uri.as_str());

    let keyspace = match (&keyspace_or_options, &options) {
      (Some(Either::A(keyspace)), _) => Ok(Some(keyspace.clone())),
      (Some(Either::B(_)), Some(_)) => Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Options cannot be provided twice",
      )),
      (Some(Either::B(options)), _) => Ok(options.keyspace.clone()),
      (None, Some(options)) => Ok(options.keyspace.clone()),
      (None, None) => Ok(None),
    };

    let auth = match (keyspace_or_options, options) {
      (Some(Either::A(_)), Some(options)) => Ok(options.auth),
      (Some(Either::B(_)), Some(_)) => Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Options cannot be provided twice",
      )),
      (Some(Either::B(options)), None) => Ok(options.auth),
      (None, Some(options)) => Ok(options.auth),
      (None, None) => Ok(None),
      (Some(Either::A(_)), None) => Ok(None),
    };

    if let Some(keyspace) = keyspace? {
      builder = builder.use_keyspace(keyspace, false);
    }

    if let Some(auth) = auth? {
      builder = builder.user(auth.username, auth.password);
    }

    if let Some(default_execution_profile) = &self.default_execution_profile {
      builder = builder.default_execution_profile_handle(default_execution_profile.into_handle());
    }

    if let Some(compression) = self.compression {
      builder = builder.compression(compression.into());
    }

    Ok(ScyllaSession::new(builder.build().await.unwrap()))
  }
}
