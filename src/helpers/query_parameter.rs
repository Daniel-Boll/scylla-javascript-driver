use crate::types::uuid::Uuid;
use napi::bindgen_prelude::Either3;
use scylla::_macro_internal::SerializedValues;

pub struct QueryParameter {
  pub(crate) parameters: Option<Vec<Either3<u32, String, Uuid>>>,
}

impl QueryParameter {
  pub fn parser(parameters: Option<Vec<Either3<u32, String, &Uuid>>>) -> Option<SerializedValues> {
    let mut values = None;

    if let Some(parameters) = parameters {
      values = Some(SerializedValues::with_capacity(parameters.len()));

      for parameter in parameters {
        match parameter {
          Either3::A(number) => values
            .as_mut()
            .unwrap()
            .add_value(&(number as i32))
            .unwrap(),
          Either3::B(str) => values.as_mut().unwrap().add_value(&str).unwrap(),
          Either3::C(uuid) => values.as_mut().unwrap().add_value(&(uuid.uuid)).unwrap(),
        }
      }
      return values;
    }
    values
  }
}
