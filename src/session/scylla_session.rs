use napi::bindgen_prelude::Either3;
use scylla::_macro_internal::SerializedValues;
use scylla::frame::response::result::ColumnType;

use crate::types::uuid::Uuid;

#[napi]
pub struct ScyllaSession {
  session: scylla::Session,
}

#[napi]
impl ScyllaSession {
  pub fn new(session: scylla::Session) -> Self {
    Self { session }
  }

  #[napi]
  pub async fn execute(
    &self,
    query: String,
    parameters: Option<Vec<Either3<u32, String, &Uuid>>>,
  ) -> napi::Result<serde_json::Value> {
    let query_result = if let Some(parameters) = parameters {
      let mut values = SerializedValues::with_capacity(parameters.len());

      for parameter in parameters {
        match parameter {
          Either3::A(number) => values.add_value(&(number as i32)).unwrap(),
          Either3::B(str) => values.add_value(&str).unwrap(),
          Either3::C(uuid) => values.add_value(&(uuid.uuid)).unwrap(),
        }
      }

      self.session.query(query.clone(), values).await
    } else {
      self.session.query(query.clone(), &[]).await
    }
    .unwrap();

    // If no rows were found return an empty array
    if query_result.result_not_rows().is_ok() {
      return Ok(serde_json::json!([]));
    }

    // Empty results
    if query_result.rows.is_none() {
      return Ok(serde_json::json!([]));
    }

    let rows = query_result.rows.unwrap();
    let column_specs = query_result.col_specs;

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

    Ok(result)
  }
}
