use scylla::frame::value::CqlDuration;

#[napi(object)]
#[derive(Debug, Clone, Copy)]
pub struct Duration {
  pub months: i32,
  pub days: i32,
  pub nanoseconds: i64,
}

impl From<CqlDuration> for Duration {
  fn from(value: CqlDuration) -> Self {
    Self {
      months: value.months,
      days: value.days,
      nanoseconds: value.nanoseconds,
    }
  }
}

impl From<Duration> for CqlDuration {
  fn from(value: Duration) -> Self {
    Self {
      months: value.months,
      days: value.days,
      nanoseconds: value.nanoseconds,
    }
  }
}
