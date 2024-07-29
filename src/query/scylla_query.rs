use std::fmt::Display;

use crate::cluster::execution_profile::{
  consistency::Consistency, serial_consistency::SerialConsistency,
};
use scylla::query::Query;

#[napi(js_name = "Query")]
pub struct ScyllaQuery {
  pub(crate) query: Query,
}

impl Display for ScyllaQuery {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "ScyllaQuery: {}", self.query.contents)
  }
}

#[napi]
impl ScyllaQuery {
  #[napi(constructor)]
  pub fn new(query: String) -> Self {
    Self {
      query: Query::new(query),
    }
  }

  #[napi]
  pub fn set_consistency(&mut self, consistency: Consistency) {
    self.query.set_consistency(consistency.into());
  }

  #[napi]
  pub fn set_serial_consistency(&mut self, serial_consistency: SerialConsistency) {
    self
      .query
      .set_serial_consistency(Some(serial_consistency.into()));
  }

  #[napi]
  pub fn set_page_size(&mut self, page_size: i32) {
    self.query.set_page_size(page_size);
  }
}
