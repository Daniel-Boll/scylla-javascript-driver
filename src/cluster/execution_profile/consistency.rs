use napi::bindgen_prelude::*;

#[napi]
pub enum Consistency {
  Any = 0x0000,
  One = 0x0001,
  Two = 0x0002,
  Three = 0x0003,
  Quorum = 0x0004,
  All = 0x0005,
  LocalQuorum = 0x0006,
  EachQuorum = 0x0007,
  LocalOne = 0x000A,

  // Apparently, Consistency can be set to Serial or LocalSerial in SELECT statements
  // to make them use Paxos.
  Serial = 0x0008,
  LocalSerial = 0x0009,
}

impl From<Consistency> for scylla::statement::Consistency {
  fn from(value: Consistency) -> Self {
    match value {
      Consistency::Any => Self::Any,
      Consistency::One => Self::One,
      Consistency::Two => Self::Two,
      Consistency::Three => Self::Three,
      Consistency::Quorum => Self::Quorum,
      Consistency::All => Self::All,
      Consistency::LocalQuorum => Self::LocalQuorum,
      Consistency::EachQuorum => Self::EachQuorum,
      Consistency::LocalOne => Self::LocalOne,
      Consistency::Serial => Self::Serial,
      Consistency::LocalSerial => Self::LocalSerial,
    }
  }
}
