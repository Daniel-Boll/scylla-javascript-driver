use scylla::frame::response::result::CqlValue;

use crate::helpers::{cql_value_bridge::ParameterWithMapType, to_cql_value::ToCqlValue};

/// A list of any CqlType
#[napi]
#[derive(Debug, Clone)]
pub struct List {
  pub(crate) inner: Vec<CqlValue>,
}

impl From<Vec<CqlValue>> for List {
  fn from(inner: Vec<CqlValue>) -> Self {
    Self { inner }
  }
}

impl From<List> for Vec<CqlValue> {
  fn from(list: List) -> Self {
    list.inner
  }
}

impl From<&List> for Vec<CqlValue> {
  fn from(list: &List) -> Self {
    list.inner.clone()
  }
}

#[napi]
impl List {
  #[napi(constructor, ts_args_type = "values: T[]")]
  pub fn new_list(values: Vec<ParameterWithMapType>) -> List {
    let inner = values.into_iter().map(|v| v.to_cql_value()).collect();
    List { inner }
  }
}
