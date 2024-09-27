use std::fmt::Debug;

use napi::bindgen_prelude::Uint8Array;
use scylla::frame::value::CqlDecimal;

#[napi]
#[derive(Clone)]
pub struct Decimal {
  int_val: Uint8Array,
  scale: i32,
}

impl From<CqlDecimal> for Decimal {
  fn from(value: CqlDecimal) -> Self {
    let (int_val, scale) = value.as_signed_be_bytes_slice_and_exponent();

    Self {
      int_val: int_val.into(),
      scale,
    }
  }
}

impl From<Decimal> for CqlDecimal {
  fn from(value: Decimal) -> Self {
    CqlDecimal::from_signed_be_bytes_slice_and_exponent(value.int_val.as_ref(), value.scale)
  }
}

impl Debug for Decimal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Decimal")
      .field("int_val", &self.int_val.into_iter().collect::<Vec<_>>())
      .field("scale", &self.scale)
      .finish()
  }
}

// TODO: implement operations for this wrapper
