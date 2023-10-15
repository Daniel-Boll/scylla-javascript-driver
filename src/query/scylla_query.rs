use scylla::query::Query;
use scylla::statement::Consistency;

#[napi(js_name = "Query")]
pub struct ScyllaQuery {
  pub(crate) query: Query,
}

#[napi]
impl ScyllaQuery {
  /// Object config is in the format:
  #[napi(constructor)]
  pub fn new(query: String) -> Self {
    Self {
      query: Query::new(query),
    }
  }

  pub fn set_consistency(&mut self, consistency: Consistency) {
    self.query.set_consistency(Consistency::from(consistency));
  }

  pub fn set_page_size(&mut self, page_size: i32) {
    self.query.set_page_size(page_size);
  }
}
