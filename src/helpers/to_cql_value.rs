use napi::bindgen_prelude::BigInt;
use scylla::frame::response::result::CqlValue;

use crate::types::{
  decimal::Decimal, duration::Duration, float::Float, list::List, set::Set, uuid::Uuid,
  varint::Varint,
};

// Trait to abstract the conversion to CqlValue
pub trait ToCqlValue {
  fn to_cql_value(&self) -> CqlValue;
}

// Implement ToCqlValue for various types
impl ToCqlValue for u32 {
  fn to_cql_value(&self) -> CqlValue {
    CqlValue::Int(*self as i32)
  }
}

impl ToCqlValue for String {
  fn to_cql_value(&self) -> CqlValue {
    CqlValue::Text(self.clone())
  }
}

impl ToCqlValue for &Uuid {
  fn to_cql_value(&self) -> CqlValue {
    CqlValue::Uuid(self.get_inner())
  }
}

impl ToCqlValue for BigInt {
  fn to_cql_value(&self) -> CqlValue {
    CqlValue::BigInt(self.get_i64().0)
  }
}

impl ToCqlValue for &Duration {
  fn to_cql_value(&self) -> CqlValue {
    CqlValue::Duration((**self).into())
  }
}

impl ToCqlValue for &Decimal {
  fn to_cql_value(&self) -> CqlValue {
    CqlValue::Decimal((*self).into())
  }
}

impl ToCqlValue for bool {
  fn to_cql_value(&self) -> CqlValue {
    CqlValue::Boolean(*self)
  }
}

impl ToCqlValue for &Float {
  fn to_cql_value(&self) -> CqlValue {
    CqlValue::Float((*self).into())
  }
}

impl ToCqlValue for &Varint {
  fn to_cql_value(&self) -> CqlValue {
    CqlValue::Varint((*self).into())
  }
}

impl ToCqlValue for &List {
  fn to_cql_value(&self) -> CqlValue {
    CqlValue::List(self.inner.clone())
  }
}

impl ToCqlValue for &Set {
  fn to_cql_value(&self) -> CqlValue {
    CqlValue::Set(self.inner.clone())
  }
}

// Helper function to convert u32 vector to u8 vector
fn u32_vec_to_u8_vec(input: &[u32]) -> Vec<u8> {
  input.iter().map(|&num| num as u8).collect()
}

impl ToCqlValue for Vec<u32> {
  fn to_cql_value(&self) -> CqlValue {
    CqlValue::Blob(u32_vec_to_u8_vec(self))
  }
}
