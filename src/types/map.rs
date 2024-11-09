use scylla::frame::response::result::CqlValue;

use crate::helpers::{cql_value_bridge::ParameterWithMapType, to_cql_value::ToCqlValue};

/// A map of any CqlType to any CqlType
#[napi]
#[derive(Debug, Clone, PartialEq)]
pub struct Map {
  pub(crate) inner: Vec<(CqlValue, CqlValue)>,
}

impl From<Vec<(CqlValue, CqlValue)>> for Map {
  fn from(inner: Vec<(CqlValue, CqlValue)>) -> Self {
    Self { inner }
  }
}

impl From<Map> for Vec<(CqlValue, CqlValue)> {
  fn from(map: Map) -> Self {
    map.inner
  }
}

impl From<&Map> for Vec<(CqlValue, CqlValue)> {
  fn from(map: &Map) -> Self {
    map.inner.clone()
  }
}

#[napi]
impl Map {
  #[napi(constructor, ts_args_type = "values: Array<Array<T | U>>")]
  pub fn new_map(values: Vec<Vec<ParameterWithMapType>>) -> Map {
    Map {
      inner: values
        .into_iter()
        .map(|v| {
          let key = v[0].to_cql_value();
          let value = v[1].to_cql_value();
          (key, value)
        })
        .collect(),
    }
  }

  #[napi]
  #[allow(clippy::inherent_to_string)]
  pub fn to_string(&self) -> String {
    format!("{:?}", self.inner)
  }
}
