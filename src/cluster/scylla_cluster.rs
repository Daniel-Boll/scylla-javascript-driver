use std::time::Duration;

use napi::Either;
use openssl::ssl::SslContextBuilder;

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
  auto_await_schema_agreement: Option<bool>,
  schema_agreement_interval: Option<Duration>,

  // connection fields
  connection: Option<ConnectionOptions>,
}

#[napi(object)]
struct ConnectionOptions {
  pub keyspace: Option<String>,
  pub auth: Option<Auth>,
  pub ssl: Option<Ssl>,
}

#[napi(object)]
#[derive(Clone, Debug)]
pub struct Auth {
  pub username: String,
  pub password: String,
}

#[napi(object)]
#[derive(Clone)]
pub struct Ssl {
  pub ca_filepath: String,
  pub verify_mode: Option<VerifyMode>,
}

#[napi]
pub enum VerifyMode {
  None,
  Peer,
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
      keyspace,
      auth,
      ssl,
      auto_await_schema_agreement,
      schema_agreement_interval,
    } = cluster_config;

    let uri = nodes.first().expect("at least one node is required");

    Self {
      uri: uri.to_string(),
      compression,
      default_execution_profile,
      connection: Some(ConnectionOptions {
        keyspace,
        auth,
        ssl,
      }),
      auto_await_schema_agreement,
      schema_agreement_interval: schema_agreement_interval.map(|d| Duration::from_secs(d as u64)),
    }
  }

  #[napi]
  /// Connect to the cluster
  pub async fn connect(
    &self,
    keyspace_or_options: Option<Either<String, ConnectionOptions>>,
    options: Option<ConnectionOptions>,
  ) -> napi::Result<ScyllaSession> {
    let mut builder = scylla::SessionBuilder::new().known_node(self.uri.as_str());

    // TODO: We need to think of a better way to deal with keyspace possibly being options
    let keyspace: Result<Option<String>, napi::Error> = match (&keyspace_or_options, &options) {
      (Some(Either::A(keyspace)), _) => Ok(Some(keyspace.clone())),
      (Some(Either::B(options)), _) => {
        if options.keyspace.is_none() {
          Ok(
            self
              .connection
              .as_ref()
              .and_then(|conn| conn.keyspace.clone()),
          )
        } else {
          Ok(options.keyspace.clone())
        }
      }
      (None, Some(options)) => {
        if options.keyspace.is_none() {
          Ok(
            self
              .connection
              .as_ref()
              .and_then(|conn| conn.keyspace.clone()),
          )
        } else {
          Ok(options.keyspace.clone())
        }
      }
      (None, None) => Ok(
        self
          .connection
          .as_ref()
          .and_then(|conn| conn.keyspace.clone()),
      ),
    };

    let auth = match (&keyspace_or_options, &options) {
      (Some(Either::A(_)), Some(options)) => Ok(options.auth.clone()), // when keyspace is provided as a string
      (Some(Either::A(_)), None) => Ok(self.connection.as_ref().and_then(|conn| conn.auth.clone())), // when keyspace is provided as a string and options is not provided
      (Some(Either::B(options)), None) => {
        if options.auth.is_none() {
          Ok(self.connection.as_ref().and_then(|conn| conn.auth.clone()))
        } else {
          Ok(options.auth.clone())
        }
      } // when keyspace is provided as an object
      (Some(Either::B(_)), Some(_)) => Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Options cannot be provided twice",
      )), // when keyspace is provided as an object and options is already provided
      (None, Some(options)) => {
        if options.auth.is_none() {
          Ok(self.connection.as_ref().and_then(|conn| conn.auth.clone()))
        } else {
          Ok(options.auth.clone())
        }
      } // when keyspace is not provided and options is provided (shouldn't happen)
      (None, None) => Ok(self.connection.as_ref().and_then(|conn| conn.auth.clone())), // when keyspace is not provided and options is not provided
    };

    let ssl = match (&keyspace_or_options, &options) {
      (Some(Either::A(_)), Some(options)) => {
        if options.ssl.is_none() {
          Ok(self.connection.as_ref().and_then(|conn| conn.ssl.clone()))
        } else {
          Ok(options.ssl.clone())
        }
      }
      (Some(Either::B(_)), Some(_)) => Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Options cannot be provided twice",
      )),
      (Some(Either::B(options)), None) => {
        if options.ssl.is_none() {
          Ok(self.connection.as_ref().and_then(|conn| conn.ssl.clone()))
        } else {
          Ok(options.ssl.clone())
        }
      }
      (None, Some(options)) => {
        if options.ssl.is_none() {
          Ok(self.connection.as_ref().and_then(|conn| conn.ssl.clone()))
        } else {
          Ok(options.ssl.clone())
        }
      }
      (None, None) => Ok(self.connection.as_ref().and_then(|conn| conn.ssl.clone())),
      (Some(Either::A(_)), None) => Ok(self.connection.as_ref().and_then(|conn| conn.ssl.clone())),
    };

    if let Some(keyspace) = keyspace.clone()? {
      builder = builder.use_keyspace(keyspace, false);
    }

    if let Some(auth) = auth? {
      builder = builder.user(auth.username, auth.password);
    }

    if let Some(ssl) = ssl? {
      let ssl_builder = SslContextBuilder::new(openssl::ssl::SslMethod::tls());

      if let Err(err) = ssl_builder {
        return Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Failed to create SSL context: {}", err),
        ));
      }

      // Safe to unwrap because we checked for Err above
      let mut ssl_builder = ssl_builder.unwrap();

      if let Some(verify_mode) = ssl.verify_mode {
        ssl_builder.set_verify(match verify_mode {
          VerifyMode::None => openssl::ssl::SslVerifyMode::NONE,
          VerifyMode::Peer => openssl::ssl::SslVerifyMode::PEER,
        });
      } else {
        ssl_builder.set_verify(openssl::ssl::SslVerifyMode::NONE);
      }

      if let Err(err) = ssl_builder.set_ca_file(ssl.ca_filepath) {
        return Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Failed to set CA file: {}", err),
        ));
      }

      if let Some(auto_await_schema_agreement) = self.auto_await_schema_agreement {
        builder = builder.auto_await_schema_agreement(auto_await_schema_agreement);
      }

      if let Some(schema_agreement_interval) = self.schema_agreement_interval {
        builder = builder.schema_agreement_interval(schema_agreement_interval);
      }

      builder = builder.ssl_context(Some(ssl_builder.build()));
    }

    if let Some(default_execution_profile) = &self.default_execution_profile {
      builder = builder.default_execution_profile_handle(default_execution_profile.into_handle());
    }

    if let Some(compression) = self.compression {
      builder = builder.compression(compression.into());
    }

    let session = builder.build().await;

    match session {
      Ok(session) => Ok(ScyllaSession::new(session)),
      Err(err) => Err(napi::Error::from_reason(format!(
        "Failed to connect to the database: {} - [{uri}] - Keyspace: {keyspace}",
        err,
        uri = self.uri,
        keyspace = keyspace
          .unwrap_or(Some("No keyspace provided".to_string()))
          .unwrap_or("No keyspace provided".to_string())
      ))),
    }
  }
}
