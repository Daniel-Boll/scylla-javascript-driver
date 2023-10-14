#[napi(object)]
pub struct ClusterConfig {
  pub nodes: Vec<String>,
}
