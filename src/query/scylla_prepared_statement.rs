use scylla::prepared_statement;

use crate::cluster::execution_profile::{
  consistency::Consistency, serial_consistency::SerialConsistency,
};

#[napi]
pub struct PreparedStatement {
  pub(crate) prepared: prepared_statement::PreparedStatement,
}

#[napi]
impl PreparedStatement {
  pub fn new(prepared: prepared_statement::PreparedStatement) -> Self {
    Self { prepared }
  }

  #[napi]
  pub fn set_consistency(&mut self, consistency: Consistency) {
    self.prepared.set_consistency(consistency.into());
  }

  #[napi]
  pub fn set_serial_consistency(&mut self, serial_consistency: SerialConsistency) {
    self
      .prepared
      .set_serial_consistency(Some(serial_consistency.into()));
  }
}
