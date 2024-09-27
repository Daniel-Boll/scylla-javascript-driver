use std::collections::HashMap;

use napi::bindgen_prelude::{BigInt, Either10, Either9};
use scylla::frame::response::result::{ColumnType, CqlValue};

use crate::types::{decimal::Decimal, duration::Duration, uuid::Uuid};
pub struct QueryResult {
  pub(crate) result: scylla::QueryResult,
}

macro_rules! define_return_type {
    ($($t:ty),+) => {
      type NativeTypes = Either9<$($t),+>;
      type WithMapType = Either10<$($t),+, HashMap<String, NativeTypes>>;
      type ReturnType = napi::Result<Option<WithMapType>>;
      pub type JSQueryResult = napi::Result<Vec<HashMap<String, WithMapType>>>;
    };
}

define_return_type!(
  String,
  i64,
  f64,
  bool,
  BigInt,
  Uuid,
  Duration,
  Decimal,
  Vec<u8>
);

impl QueryResult {
  pub fn parser(result: scylla::QueryResult) -> JSQueryResult {
    if result.result_not_rows().is_ok() || result.rows.is_none() {
      return Ok(Default::default());
    }

    let rows = result.rows.unwrap();
    let column_specs = result.col_specs;

    let mut result_json: Vec<HashMap<String, WithMapType>> = vec![];

    for row in rows {
      let mut row_object: HashMap<String, WithMapType> = HashMap::new();

      for (i, column) in row.columns.iter().enumerate() {
        let column_name = column_specs[i].name.clone();
        let column_value = Self::parse_value(column, &column_specs[i].typ)?;
        if let Some(column_value) = column_value {
          row_object.insert(column_name, column_value);
        }
      }

      result_json.push(row_object);
    }

    Ok(result_json)
  }

  fn parse_value(column: &Option<CqlValue>, column_type: &ColumnType) -> ReturnType {
    column
      .as_ref()
      .map(|column| match column_type {
        ColumnType::Ascii => Ok(WithMapType::A(column.as_ascii().unwrap().to_string())),
        ColumnType::Text => Ok(WithMapType::A(column.as_text().unwrap().to_string())),
        ColumnType::Uuid => Ok(WithMapType::F(Uuid {
          uuid: column.as_uuid().unwrap(),
        })),
        ColumnType::BigInt => Ok(WithMapType::E(column.as_bigint().unwrap().into())),
        ColumnType::Int => Ok(WithMapType::B(column.as_int().unwrap() as i64)),
        ColumnType::Float => Ok(WithMapType::C(column.as_float().unwrap() as f64)),
        ColumnType::Double => Ok(WithMapType::C(column.as_double().unwrap())),
        ColumnType::Boolean => Ok(WithMapType::D(column.as_boolean().unwrap())),
        ColumnType::SmallInt => Ok(WithMapType::B(column.as_smallint().unwrap() as i64)),
        ColumnType::TinyInt => Ok(WithMapType::B(column.as_tinyint().unwrap() as i64)),
        ColumnType::Date | ColumnType::Timestamp => {
          Ok(WithMapType::A(column.as_date().unwrap().to_string()))
        }
        ColumnType::Inet => Ok(WithMapType::A(column.as_inet().unwrap().to_string())),
        ColumnType::Duration => Ok(WithMapType::G(column.as_cql_duration().unwrap().into())),
        ColumnType::Decimal => Ok(WithMapType::H(
          column.clone().into_cql_decimal().unwrap().into(),
        )),
        ColumnType::Blob => Ok(WithMapType::I(column.as_blob().unwrap().clone())),
        ColumnType::Counter => Ok(WithMapType::B(column.as_counter().unwrap().0)),
        ColumnType::Varint => Ok(WithMapType::I(
          column
            .clone()
            .into_cql_varint()
            .unwrap()
            .as_signed_bytes_be_slice()
            .into(),
        )),
        ColumnType::Time => Ok(WithMapType::B(column.as_time().unwrap().nanosecond() as i64)),
        ColumnType::Timeuuid => Ok(WithMapType::F(column.as_timeuuid().unwrap().into())),
        ColumnType::Map(key, value) => {
          let map = column
            .as_map()
            .unwrap()
            .iter()
            .map(|(k, v)| {
              let key = Self::parse_value(&Some(k.clone()), key).unwrap();
              let value =
                Self::remove_map_from_type(Self::parse_value(&Some(v.clone()), value).unwrap())?
                  .unwrap();
              key
                .map(|key| match key {
                  WithMapType::A(key) => Ok((key, value)),
                  _ => Err(napi::Error::new(
                    napi::Status::GenericFailure,
                    "Map key must be a string",
                  )),
                })
                .transpose()
            })
            .collect::<napi::Result<Option<HashMap<String, NativeTypes>>>>();

          Ok(WithMapType::J(map?.unwrap()))
        }
        ColumnType::UserDefinedType { field_types, .. } => Ok(WithMapType::J(Self::parse_udt(
          column.as_udt().unwrap(),
          field_types,
        )?)),
        ColumnType::Custom(_) => Ok(WithMapType::A(
          "ColumnType Custom not supported yet".to_string(),
        )),
        ColumnType::List(_) => Ok(WithMapType::A(
          "ColumnType List not supported yet".to_string(),
        )),
        ColumnType::Set(_) => Ok(WithMapType::A(
          "ColumnType Set not supported yet".to_string(),
        )),
        ColumnType::Tuple(_) => Ok(WithMapType::A(
          "ColumnType Tuple not supported yet".to_string(),
        )),
      })
      .transpose()
  }

  fn parse_udt(
    udt: &[(String, Option<CqlValue>)],
    field_types: &[(String, ColumnType)],
  ) -> napi::Result<HashMap<String, NativeTypes>> {
    let mut result: HashMap<String, NativeTypes> = HashMap::new();

    for (i, (field_name, field_value)) in udt.iter().enumerate() {
      let field_type = &field_types[i].1;
      let parsed_value = Self::parse_value(field_value, field_type);
      if let Some(parsed_value) = Self::remove_map_from_type(parsed_value?)? {
        result.insert(field_name.clone(), parsed_value);
      }
    }

    Ok(result)
  }

  fn remove_map_from_type(a: Option<WithMapType>) -> napi::Result<Option<NativeTypes>> {
    a.map(|f| match f {
      WithMapType::A(a) => Ok(NativeTypes::A(a)),
      WithMapType::B(a) => Ok(NativeTypes::B(a)),
      WithMapType::C(a) => Ok(NativeTypes::C(a)),
      WithMapType::D(a) => Ok(NativeTypes::D(a)),
      WithMapType::E(a) => Ok(NativeTypes::E(a)),
      WithMapType::F(a) => Ok(NativeTypes::F(a)),
      WithMapType::G(a) => Ok(NativeTypes::G(a)),
      WithMapType::H(a) => Ok(NativeTypes::H(a)),
      WithMapType::I(a) => Ok(NativeTypes::I(a)),
      WithMapType::J(_) => Err(napi::Error::new(
        napi::Status::GenericFailure,
        "Map type is not supported in this context".to_string(),
      )),
    })
    .transpose()
  }
}
