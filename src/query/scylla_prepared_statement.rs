use scylla::prepared_statement::PreparedStatement;

use crate::cluster::execution_profile::{
  consistency::Consistency, serial_consistency::SerialConsistency,
};

#[napi]
pub struct ScyllaPreparedStatement {
  pub(crate) prepared: PreparedStatement,
}

#[napi]
impl ScyllaPreparedStatement {
  pub fn new(prepared: PreparedStatement) -> Self {
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
