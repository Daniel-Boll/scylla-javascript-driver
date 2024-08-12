use scylla::frame::response::result::{ColumnType, CqlValue};
pub struct QueryResult {
  pub(crate) result: scylla::QueryResult,
}

impl QueryResult {
  pub fn parser(result: scylla::QueryResult) -> serde_json::Value {
    if result.result_not_rows().is_ok() || result.rows.is_none() {
      return serde_json::json!([]);
    }

    let rows = result.rows.unwrap();
    let column_specs = result.col_specs;

    let mut result_json = serde_json::json!([]);

    for row in rows {
      let mut row_object = serde_json::Map::new();

      for (i, column) in row.columns.iter().enumerate() {
        let column_name = column_specs[i].name.clone();
        let column_value = Self::parse_value(column, &column_specs[i].typ);
        row_object.insert(column_name, column_value);
      }

      result_json
        .as_array_mut()
        .unwrap()
        .push(serde_json::Value::Object(row_object));
    }

    result_json
  }

  fn parse_value(column: &Option<CqlValue>, column_type: &ColumnType) -> serde_json::Value {
    match column {
      Some(column) => match column_type {
        ColumnType::Ascii => serde_json::Value::String(column.as_ascii().unwrap().to_string()),
        ColumnType::Text => serde_json::Value::String(column.as_text().unwrap().to_string()),
        ColumnType::Uuid => serde_json::Value::String(column.as_uuid().unwrap().to_string()),
        ColumnType::Int => serde_json::Value::Number(
          serde_json::Number::from_f64(column.as_int().unwrap() as f64).unwrap(),
        ),
        ColumnType::Float => serde_json::Value::Number(
          serde_json::Number::from_f64(column.as_float().unwrap() as f64).unwrap(),
        ),
        ColumnType::Timestamp | ColumnType::Date => {
          serde_json::Value::String(column.as_cql_date().unwrap().0.to_string())
        }
        ColumnType::UserDefinedType { field_types, .. } => {
          Self::parse_udt(column.as_udt().unwrap(), field_types)
        }
        ColumnType::Boolean => serde_json::Value::Bool(column.as_boolean().unwrap()),
        ColumnType::Inet => serde_json::Value::String(column.as_inet().unwrap().to_string()),
        ColumnType::Double => serde_json::Value::Number(
          serde_json::Number::from_f64(column.as_double().unwrap()).unwrap(),
        ),
        ColumnType::SmallInt => serde_json::Value::Number(
          serde_json::Number::from_f64(column.as_smallint().unwrap() as f64).unwrap(),
        ),
        ColumnType::TinyInt => serde_json::Value::Number(
          serde_json::Number::from_f64(column.as_tinyint().unwrap() as f64).unwrap(),
        ),
        ColumnType::BigInt => "ColumnType BigInt not supported yet".into(),
        ColumnType::Decimal => "ColumnType Decimal not supported yet".into(),
        ColumnType::Duration => "ColumnType Duration not supported yet".into(),
        ColumnType::Custom(_) => "ColumnType Custom not supported yet".into(),
        ColumnType::Blob => "ColumnType Blob not supported yet".into(),
        ColumnType::Counter => "ColumnType Counter not supported yet".into(),
        ColumnType::List(_) => "ColumnType List not supported yet".into(),
        ColumnType::Map(_, _) => "ColumnType Map not supported yet".into(),
        ColumnType::Set(_) => "ColumnType Set not supported yet".into(),
        ColumnType::Time => "ColumnType Time not supported yet".into(),
        ColumnType::Timeuuid => "ColumnType Timeuuid not supported yet".into(),
        ColumnType::Tuple(_) => "ColumnType Tuple not supported yet".into(),
        ColumnType::Varint => "ColumnType Varint not supported yet".into(),
      },
      None => serde_json::Value::Null,
    }
  }

  fn parse_udt(
    udt: &[(String, Option<CqlValue>)],
    field_types: &[(String, ColumnType)],
  ) -> serde_json::Value {
    let mut result = serde_json::Map::new();

    for (i, (field_name, field_value)) in udt.iter().enumerate() {
      let field_type = &field_types[i].1;
      let parsed_value = Self::parse_value(field_value, field_type);
      result.insert(field_name.clone(), parsed_value);
    }

    serde_json::Value::Object(result)
  }
}
