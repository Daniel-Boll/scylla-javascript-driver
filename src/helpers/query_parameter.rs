use std::collections::HashMap;

use crate::types::{
  decimal::Decimal, duration::Duration, float::Float, uuid::Uuid, varint::Varint,
};
use napi::bindgen_prelude::{BigInt, Either10, Either11};
use scylla::{
  frame::response::result::CqlValue,
  serialize::{
    RowWriter, SerializationError,
    row::{RowSerializationContext, SerializeRow},
    value::SerializeCql,
  },
};

macro_rules! define_expected_type {
    ($lifetime:lifetime, $($t:ty),+) => {
      pub type ParameterNativeTypes<$lifetime> = Either10<$($t),+>;
      pub type ParameterWithMapType<$lifetime> = Either11<$($t),+, HashMap<String, ParameterNativeTypes<$lifetime>>>;
      pub type JSQueryParameters<$lifetime> = napi::Result<Vec<HashMap<String, ParameterWithMapType<$lifetime>>>>;
    };
}

define_expected_type!('a, u32, String, &'a Uuid, BigInt, &'a Duration, &'a Decimal, bool, Vec<u32>, &'a Float, &'a Varint);

#[derive(Debug, Clone)]
pub struct QueryParameter<'a> {
  #[allow(clippy::type_complexity)]
  pub(crate) parameters: Option<Vec<ParameterWithMapType<'a>>>,
}

impl<'a> SerializeRow for QueryParameter<'a> {
  fn serialize(
    &self,
    ctx: &RowSerializationContext<'_>,
    writer: &mut RowWriter,
  ) -> Result<(), SerializationError> {
    if let Some(parameters) = &self.parameters {
      for (i, parameter) in parameters.iter().enumerate() {
        match parameter {
          ParameterWithMapType::A(num) => {
            CqlValue::Int(*num as i32)
              .serialize(&ctx.columns()[i].typ, writer.make_cell_writer())?;
          }
          ParameterWithMapType::B(str) => {
            CqlValue::Text(str.to_string())
              .serialize(&ctx.columns()[i].typ, writer.make_cell_writer())?;
          }
          ParameterWithMapType::C(uuid) => {
            CqlValue::Uuid(uuid.get_inner())
              .serialize(&ctx.columns()[i].typ, writer.make_cell_writer())?;
          }
          ParameterWithMapType::D(bigint) => {
            CqlValue::BigInt(bigint.get_i64().0)
              .serialize(&ctx.columns()[i].typ, writer.make_cell_writer())?;
          }
          ParameterWithMapType::E(duration) => {
            CqlValue::Duration((**duration).into())
              .serialize(&ctx.columns()[i].typ, writer.make_cell_writer())?;
          }
          ParameterWithMapType::F(decimal) => {
            CqlValue::Decimal((*decimal).into())
              .serialize(&ctx.columns()[i].typ, writer.make_cell_writer())?;
          }
          ParameterWithMapType::G(bool) => {
            CqlValue::Boolean(*bool).serialize(&ctx.columns()[i].typ, writer.make_cell_writer())?;
          }
          ParameterWithMapType::H(buffer) => {
            CqlValue::Blob(u32_vec_to_u8_vec(buffer))
              .serialize(&ctx.columns()[i].typ, writer.make_cell_writer())?;
          }
          ParameterWithMapType::I(float) => {
            CqlValue::Float((*float).into())
              .serialize(&ctx.columns()[i].typ, writer.make_cell_writer())?;
          }
          ParameterWithMapType::J(varint) => {
            CqlValue::Varint((*varint).into())
              .serialize(&ctx.columns()[i].typ, writer.make_cell_writer())?;
          }
          ParameterWithMapType::K(map) => {
            CqlValue::UserDefinedType {
              // FIXME: I'm not sure why this is even necessary tho, but if it's and makes sense we'll have to make it so we get the correct info
              keyspace: "keyspace".to_string(),
              type_name: "type_name".to_string(),
              fields: map
                .iter()
                .map(|(key, value)| match value {
                  ParameterNativeTypes::A(num) => {
                    (key.to_string(), Some(CqlValue::Int(*num as i32)))
                  }
                  ParameterNativeTypes::B(str) => {
                    (key.to_string(), Some(CqlValue::Text(str.to_string())))
                  }
                  ParameterNativeTypes::C(uuid) => {
                    (key.to_string(), Some(CqlValue::Uuid(uuid.get_inner())))
                  }
                  ParameterNativeTypes::D(bigint) => {
                    (key.to_string(), Some(CqlValue::BigInt(bigint.get_i64().0)))
                  }
                  ParameterNativeTypes::E(duration) => (
                    key.to_string(),
                    Some(CqlValue::Duration((**duration).into())),
                  ),
                  ParameterNativeTypes::F(decimal) => {
                    (key.to_string(), Some(CqlValue::Decimal((*decimal).into())))
                  }
                  ParameterNativeTypes::G(bool) => {
                    (key.to_string(), Some(CqlValue::Boolean(*bool)))
                  }
                  ParameterNativeTypes::H(buffer) => (
                    key.to_string(),
                    Some(CqlValue::Blob(u32_vec_to_u8_vec(buffer))),
                  ),
                  ParameterNativeTypes::J(varint) => {
                    (key.to_string(), Some(CqlValue::Varint((*varint).into())))
                  }
                  ParameterNativeTypes::I(float) => {
                    (key.to_string(), Some(CqlValue::Float((*float).into())))
                  }
                })
                .collect::<Vec<(String, Option<CqlValue>)>>(),
            }
            .serialize(&ctx.columns()[i].typ, writer.make_cell_writer())?;
          }
        }
      }
    }
    Ok(())
  }

  fn is_empty(&self) -> bool {
    self.parameters.is_none() || self.parameters.as_ref().unwrap().is_empty()
  }
}

impl<'a> QueryParameter<'a> {
  #[allow(clippy::type_complexity)]
  pub fn parser(parameters: Option<Vec<ParameterWithMapType<'a>>>) -> Option<Self> {
    if parameters.is_none() {
      return Some(QueryParameter { parameters: None });
    }

    let parameters = parameters.unwrap();

    let mut params = Vec::with_capacity(parameters.len());
    for parameter in parameters {
      params.push(parameter);
    }

    Some(QueryParameter {
      parameters: Some(params),
    })
  }
}

fn u32_vec_to_u8_vec(input: &[u32]) -> Vec<u8> {
  input.iter().map(|&num| num as u8).collect()
}
