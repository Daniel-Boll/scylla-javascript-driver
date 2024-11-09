#[napi]
pub enum SerialConsistency {
  Serial = 0x0008,
  LocalSerial = 0x0009,
}

impl From<SerialConsistency> for scylla::statement::SerialConsistency {
  fn from(value: SerialConsistency) -> Self {
    match value {
      SerialConsistency::Serial => Self::Serial,
      SerialConsistency::LocalSerial => Self::LocalSerial,
    }
  }
}

impl From<scylla::statement::SerialConsistency> for SerialConsistency {
  fn from(value: scylla::statement::SerialConsistency) -> Self {
    match value {
      scylla::statement::SerialConsistency::Serial => Self::Serial,
      scylla::statement::SerialConsistency::LocalSerial => Self::LocalSerial,
    }
  }
}
