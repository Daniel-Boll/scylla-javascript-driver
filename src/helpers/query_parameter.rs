use crate::types::uuid::Uuid;
use napi::bindgen_prelude::Either3;
use scylla::_macro_internal::SerializedValues;

pub struct QueryParameter {
  pub(crate) parameters: Option<Vec<Either3<u32, String, Uuid>>>,
}

impl QueryParameter {
  pub fn parser(parameters: Option<Vec<Either3<u32, String, &Uuid>>>) -> Option<SerializedValues> {
    parameters.map(|params| {
      let mut values = SerializedValues::with_capacity(params.len());
      for param in params {
        match param {
          Either3::A(number) => values.add_value(&(number as i32)).unwrap(),
          Either3::B(str) => values.add_value(&str).unwrap(),
          Either3::C(uuid) => values.add_value(&(uuid.uuid)).unwrap(),
        }
      }
      values
    })
  }
}
