use crate::cluster::{execution_profile::ExecutionProfile, config::compression::Compression};

#[napi(object)]
pub struct ClusterConfig {
  pub nodes: Vec<String>,
  pub compression: Option<Compression>,
  pub default_execution_profile: Option<ExecutionProfile>,
}
