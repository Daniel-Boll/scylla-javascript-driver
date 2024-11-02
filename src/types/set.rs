use scylla::frame::response::result::CqlValue;

use crate::helpers::{cql_value_bridge::ParameterWithMapType, to_cql_value::ToCqlValue};

/// A list of any CqlType
#[napi]
#[derive(Debug, Clone, PartialEq)]
pub struct Set {
  pub(crate) inner: Vec<CqlValue>,
}

impl From<Vec<CqlValue>> for Set {
  fn from(inner: Vec<CqlValue>) -> Self {
    Self { inner }
  }
}

impl From<Set> for Vec<CqlValue> {
  fn from(list: Set) -> Self {
    list.inner
  }
}

impl From<&Set> for Vec<CqlValue> {
  fn from(list: &Set) -> Self {
    list.inner.clone()
  }
}

#[napi]
impl Set {
  #[napi(constructor, ts_args_type = "values: T[]")]
  pub fn new_set(values: Vec<ParameterWithMapType>) -> Set {
    let inner = values.into_iter().map(|v| v.to_cql_value()).collect();
    Set { inner }
  }
}
