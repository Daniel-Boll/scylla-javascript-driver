use super::execution_profile::ExecutionProfile;

#[napi(object)]
pub struct ClusterConfig {
  pub nodes: Vec<String>,
  pub default_execution_profile: Option<ExecutionProfile>,
}
