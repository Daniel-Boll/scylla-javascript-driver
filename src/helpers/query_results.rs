use std::collections::HashMap;

use napi::bindgen_prelude::{BigInt, Either4};
use scylla::frame::response::result::{ColumnType, CqlValue};

use crate::types::uuid::Uuid;
pub struct QueryResult {
  pub(crate) result: scylla::QueryResult,
}

impl QueryResult {
  pub fn parser(
    result: scylla::QueryResult,
  ) -> Vec<HashMap<String, Either4<String, i32, BigInt, Uuid>>> {
    if result.result_not_rows().is_ok() || result.rows.is_none() {
      return Default::default();
    }

    let rows = result.rows.unwrap();
    let column_specs = result.col_specs;

    let mut result_json: Vec<HashMap<String, Either4<String, i32, BigInt, Uuid>>> = vec![];

    for row in rows {
      let mut row_object: HashMap<String, Either4<String, i32, BigInt, Uuid>> = HashMap::new();

      for (i, column) in row.columns.iter().enumerate() {
        let column_name = column_specs[i].name.clone();
        let column_value = Self::parse_value(column, &column_specs[i].typ);
        if let Some(column_value) = column_value {
          row_object.insert(column_name, column_value);
        }
      }

      result_json.push(row_object);
    }

    result_json
  }

  fn parse_value(
    column: &Option<CqlValue>,
    column_type: &ColumnType,
  ) -> Option<Either4<String, i32, BigInt, Uuid>> {
    match column {
      Some(column) => Some(match column_type {
        ColumnType::Ascii => Either4::A(column.as_ascii().unwrap().to_string()),
        ColumnType::Text => Either4::A(column.as_text().unwrap().to_string()),
        ColumnType::Uuid => Either4::D(Uuid {
          uuid: column.as_uuid().unwrap(),
        }),
        ColumnType::BigInt => Either4::C(column.as_bigint().unwrap().into()),
        ColumnType::Int => Either4::B(column.as_int().unwrap()),
        // ColumnType::Int => serde_json::Value::Number(
        //   serde_json::Number::from_f64(column.as_int().unwrap() as f64).unwrap(),
        // ),
        // ColumnType::Float => serde_json::Value::Number(
        //   serde_json::Number::from_f64(column.as_float().unwrap() as f64).unwrap(),
        // ),
        // ColumnType::Timestamp | ColumnType::Date => {
        //   serde_json::Value::String(column.as_cql_date().unwrap().0.to_string())
        // }
        // ColumnType::UserDefinedType { field_types, .. } => {
        //   Self::parse_udt(column.as_udt().unwrap(), field_types)
        // }
        // ColumnType::Boolean => serde_json::Value::Bool(column.as_boolean().unwrap()),
        // ColumnType::Inet => serde_json::Value::String(column.as_inet().unwrap().to_string()),
        // ColumnType::Double => serde_json::Value::Number(
        //   serde_json::Number::from_f64(column.as_double().unwrap()).unwrap(),
        // ),
        // ColumnType::SmallInt => serde_json::Value::Number(
        //   serde_json::Number::from_f64(column.as_smallint().unwrap() as f64).unwrap(),
        // ),
        // ColumnType::TinyInt => serde_json::Value::Number(
        //   serde_json::Number::from_f64(column.as_tinyint().unwrap() as f64).unwrap(),
        // ),
        ColumnType::Decimal => Either4::A("ColumnType Decimal not supported yet".to_string()),
        ColumnType::Duration => Either4::A("ColumnType Duration not supported yet".to_string()),
        ColumnType::Custom(_) => Either4::A("ColumnType Custom not supported yet".to_string()),
        ColumnType::Blob => Either4::A("ColumnType Blob not supported yet".to_string()),
        ColumnType::Counter => Either4::A("ColumnType Counter not supported yet".to_string()),
        ColumnType::List(_) => Either4::A("ColumnType List not supported yet".to_string()),
        ColumnType::Map(_, _) => Either4::A("ColumnType Map not supported yet".to_string()),
        ColumnType::Set(_) => Either4::A("ColumnType Set not supported yet".to_string()),
        ColumnType::Time => Either4::A("ColumnType Time not supported yet".to_string()),
        ColumnType::Timeuuid => Either4::A("ColumnType Timeuuid not supported yet".to_string()),
        ColumnType::Tuple(_) => Either4::A("ColumnType Tuple not supported yet".to_string()),
        ColumnType::Varint => Either4::A("ColumnType Varint not supported yet".to_string()),
        _ => todo!(),
      }),
      None => None,
    }
  }

  fn parse_udt(
    udt: &[(String, Option<CqlValue>)],
    field_types: &[(String, ColumnType)],
  ) -> HashMap<String, Either4<String, i32, BigInt, Uuid>> {
    let mut result: HashMap<String, Either4<String, i32, BigInt, Uuid>> = HashMap::new();

    for (i, (field_name, field_value)) in udt.iter().enumerate() {
      let field_type = &field_types[i].1;
      let parsed_value = Self::parse_value(field_value, field_type);
      if let Some(parsed_value) = parsed_value {
        result.insert(field_name.clone(), parsed_value);
      }
    }

    result
  }
}
