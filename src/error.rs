// https://github.com/surrealdb/surrealdb.node/blob/main/src/error.rs
pub fn pipe_error(err: impl std::error::Error) -> napi::Error {
  napi::Error::from_reason(err.to_string())
}

pub fn pipe_error_from_string(err: impl std::string::ToString) -> napi::Error {
  napi::Error::from_reason(err.to_string())
}
