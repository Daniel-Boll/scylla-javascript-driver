use scylla::frame::value::CqlVarint;

/// Native CQL `varint` representation.
///
/// Represented as two's-complement binary in big-endian order.
///
/// This type is a raw representation in bytes. It's the default
/// implementation of `varint` type - independent of any
/// external crates and crate features.
///
/// # DB data format
/// Notice that constructors don't perform any normalization
/// on the provided data. This means that underlying bytes may
/// contain leading zeros.
///
/// Currently, Scylla and Cassandra support non-normalized `varint` values.
/// Bytes provided by the user via constructor are passed to DB as is.
#[napi]
#[derive(Debug, Clone)]
pub struct Varint {
  pub(crate) inner: Vec<u8>,
}

impl From<Vec<u8>> for Varint {
  fn from(inner: Vec<u8>) -> Self {
    Self { inner }
  }
}

impl From<Varint> for Vec<u8> {
  fn from(varint: Varint) -> Self {
    varint.inner
  }
}

impl From<&Varint> for CqlVarint {
  fn from(varint: &Varint) -> Self {
    CqlVarint::from_signed_bytes_be(varint.inner.clone())
  }
}

#[napi]
impl Varint {
  #[napi(constructor)]
  pub fn new_varint(inner: Vec<u8>) -> Varint {
    Varint::from(inner)
  }
}
