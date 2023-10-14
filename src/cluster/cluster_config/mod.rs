use crate::cluster::{
  cluster_config::compression::Compression, execution_profile::ExecutionProfile,
};

pub mod compression;

#[napi(object)]
pub struct ClusterConfig {
  pub nodes: Vec<String>,
  pub compression: Option<Compression>,
  pub default_execution_profile: Option<ExecutionProfile>,
}
