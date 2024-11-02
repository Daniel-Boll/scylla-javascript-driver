use scylla::serialize::{
  RowWriter, SerializationError,
  row::{RowSerializationContext, SerializeRow},
  value::SerializeCql,
};

use super::{cql_value_bridge::ParameterWithMapType, to_cql_value::ToCqlValue};

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
        parameter
          .to_cql_value()
          .serialize(&ctx.columns()[i].typ, writer.make_cell_writer())?;
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
