use std::fmt::Display;

use napi::Either;
use scylla::batch::Batch;

use super::{scylla_prepared_statement::PreparedStatement, scylla_query::Query};

/// Batch statements
///
/// A batch statement allows to execute many data-modifying statements at once.
/// These statements can be simple or prepared.
/// Only INSERT, UPDATE and DELETE statements are allowed.
#[napi(js_name = "BatchStatement")]
pub struct ScyllaBatchStatement {
  pub(crate) batch: Batch,
}

impl Display for ScyllaBatchStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "ScyllaBatchStatement: {:?}",
      self
        .batch
        .statements
        .iter()
        .map(|s| match s {
          scylla::batch::BatchStatement::Query(q) => q.contents.clone(),
          scylla::batch::BatchStatement::PreparedStatement(p) => p.get_statement().to_string(),
        })
        .collect::<Vec<_>>()
    )
  }
}

#[napi]
impl ScyllaBatchStatement {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      batch: Default::default(),
    }
  }

  /// Appends a statement to the batch.
  ///
  /// _Warning_
  /// Using simple statements with bind markers in batches is strongly discouraged. For each simple statement with a non-empty list of values in the batch, the driver will send a prepare request, and it will be done sequentially. Results of preparation are not cached between `session.batch` calls. Consider preparing the statements before putting them into the batch.
  #[napi]
  pub fn append_statement(&mut self, statement: Either<&Query, &PreparedStatement>) {
    match statement {
      Either::A(simple_query) => self.batch.append_statement(simple_query.query.clone()),
      Either::B(prepared_statement) => self
        .batch
        .append_statement(prepared_statement.prepared.clone()),
    }
  }
}

impl Default for ScyllaBatchStatement {
  fn default() -> Self {
    Self::new()
  }
}
