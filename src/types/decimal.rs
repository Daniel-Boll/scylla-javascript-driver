use std::fmt::Debug;

use scylla::frame::value::CqlDecimal;

#[napi]
#[derive(Clone, PartialEq, Eq)]
pub struct Decimal {
  int_val: Vec<u8>,
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

impl From<&Decimal> for CqlDecimal {
  fn from(value: &Decimal) -> Self {
    CqlDecimal::from_signed_be_bytes_slice_and_exponent(value.int_val.as_ref(), value.scale)
  }
}

impl Debug for Decimal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Decimal")
      .field("int_val", &self.int_val)
      .field("scale", &self.scale)
      .finish()
  }
}

// TODO: implement operations for this wrapper
#[napi]
impl Decimal {
  #[napi(constructor)]
  pub fn new(int_val: Vec<u8>, scale: i32) -> Self {
    Self { int_val, scale }
  }

  /// Returns the string representation of the Decimal.
  // TODO: Check really how this is supposed to be displayed
  #[napi]
  #[allow(clippy::inherent_to_string)]
  pub fn to_string(&self) -> String {
    let mut result = String::new();
    for b in &self.int_val {
      result.push_str(&format!("{:02x}", b));
    }
    result.push('e');
    result.push_str(&self.scale.to_string());
    result
  }
}
