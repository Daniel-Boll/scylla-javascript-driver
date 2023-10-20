use std::sync::Arc;

use crate::error::pipe_error_from_string;

#[napi]
pub struct Metrics {
  metrics: Arc<scylla::Metrics>,
}

#[napi]
impl Metrics {
  pub fn new(metrics: Arc<scylla::Metrics>) -> Self {
    Self { metrics }
  }

  /// Returns counter for nonpaged queries
  #[napi]
  pub fn get_queries_num(&self) -> u64 {
    self.metrics.get_queries_num()
  }

  /// Returns counter for pages requested in paged queries
  #[napi]
  pub fn get_queries_iter_num(&self) -> u64 {
    self.metrics.get_queries_iter_num()
  }

  /// Returns counter for errors occurred in nonpaged queries
  #[napi]
  pub fn get_errors_num(&self) -> u64 {
    self.metrics.get_errors_num()
  }

  /// Returns counter for errors occurred in paged queries
  #[napi]
  pub fn get_errors_iter_num(&self) -> u64 {
    self.metrics.get_errors_iter_num()
  }

  /// Returns average latency in milliseconds
  #[napi]
  pub fn get_latency_avg_ms(&self) -> napi::Result<u64> {
    self
      .metrics
      .get_latency_avg_ms()
      .map_err(pipe_error_from_string)
  }

  /// Returns latency from histogram for a given percentile
  ///
  /// # Arguments
  ///
  /// * `percentile` - float value (0.0 - 100.0), value will be clamped to this range
  #[napi]
  pub fn get_latency_percentile_ms(&self, percentile: f64) -> napi::Result<u64> {
    self
      .metrics
      .get_latency_percentile_ms(percentile.clamp(0.0, 100.0))
      .map_err(pipe_error_from_string)
  }
}
