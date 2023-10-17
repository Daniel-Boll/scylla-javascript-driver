use scylla::frame::response::result::ColumnType;
pub struct QueryResult {
  pub(crate) result: scylla::QueryResult,
}

impl QueryResult {
  pub fn parser(result: scylla::QueryResult) -> serde_json::Value {
    if result.result_not_rows().is_ok() {
      return serde_json::json!([]);
    }

    if result.rows.is_none() {
      return serde_json::json!([]);
    }

    let rows = result.rows.unwrap();
    let column_specs = result.col_specs;

    let mut result = serde_json::json!([]);

    for row in rows {
      let mut row_object = serde_json::Map::new();

      for (i, column) in row.columns.iter().enumerate() {
        let column_name = column_specs[i].name.clone();

        let column_value = match column {
          Some(column) => match column_specs[i].typ {
            ColumnType::Ascii => serde_json::Value::String(column.as_ascii().unwrap().to_string()),
            ColumnType::Text => serde_json::Value::String(column.as_text().unwrap().to_string()),
            ColumnType::Uuid => serde_json::Value::String(column.as_uuid().unwrap().to_string()),
            ColumnType::Int => serde_json::Value::Number(
              serde_json::Number::from_f64(column.as_int().unwrap() as f64).unwrap(),
            ),
            _ => "Not implemented".into(),
          },
          None => serde_json::Value::Null,
        };

        row_object.insert(column_name, column_value);
      }

      result
        .as_array_mut()
        .unwrap()
        .push(serde_json::Value::Object(row_object));
    }

    result
  }
}
