use std::collections::HashMap;

use crate::types::uuid::Uuid;
use napi::bindgen_prelude::{BigInt, Either4, Either5};
use scylla::{
  frame::response::result::CqlValue,
  serialize::{
    row::{RowSerializationContext, SerializeRow},
    value::SerializeCql,
    RowWriter, SerializationError,
  },
};

pub struct QueryParameter<'a> {
  #[allow(clippy::type_complexity)]
  pub(crate) parameters: Option<
    Vec<
      Either5<
        u32,
        String,
        &'a Uuid,
        BigInt,
        HashMap<String, Either4<u32, String, &'a Uuid, BigInt>>,
      >,
    >,
  >,
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
          Either5::A(num) => {
            CqlValue::Int(*num as i32)
              .serialize(&ctx.columns()[i].typ, writer.make_cell_writer())?;
          }
          Either5::B(str) => {
            CqlValue::Text(str.to_string())
              .serialize(&ctx.columns()[i].typ, writer.make_cell_writer())?;
          }
          Either5::C(uuid) => {
            CqlValue::Uuid(uuid.get_inner())
              .serialize(&ctx.columns()[i].typ, writer.make_cell_writer())?;
          }
          Either5::D(bigint) => {
            CqlValue::BigInt(bigint.get_i64().0)
              .serialize(&ctx.columns()[i].typ, writer.make_cell_writer())?;
          }
          Either5::E(map) => {
            CqlValue::UserDefinedType {
              // FIXME: I'm not sure why this is even necessary tho, but if it's and makes sense we'll have to make it so we get the correct info
              keyspace: "keyspace".to_string(),
              type_name: "type_name".to_string(),
              fields: map
                .iter()
                .map(|(key, value)| match value {
                  Either4::A(num) => (key.to_string(), Some(CqlValue::Int(*num as i32))),
                  Either4::B(str) => (key.to_string(), Some(CqlValue::Text(str.to_string()))),
                  Either4::C(uuid) => (key.to_string(), Some(CqlValue::Uuid(uuid.get_inner()))),
                  Either4::D(bigint) => {
                    (key.to_string(), Some(CqlValue::BigInt(bigint.get_i64().0)))
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
  pub fn parser(
    parameters: Option<
      Vec<
        Either5<
          u32,
          String,
          &'a Uuid,
          BigInt,
          HashMap<String, Either4<u32, String, &'a Uuid, BigInt>>,
        >,
      >,
    >,
  ) -> Option<Self> {
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
