#[napi]
pub enum Compression {
  None,
  Lz4,
  Snappy,
}

impl From<Compression> for Option<scylla::transport::Compression> {
  fn from(value: Compression) -> Self {
    match value {
      Compression::None => None,
      Compression::Lz4 => Some(scylla::transport::Compression::Lz4),
      Compression::Snappy => Some(scylla::transport::Compression::Snappy),
    }
  }
}
