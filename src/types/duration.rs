use scylla::frame::value::CqlDuration;

#[napi]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[napi]
impl Duration {
  #[napi(constructor)]
  pub fn new(months: i32, days: i32, nanoseconds: i64) -> Self {
    Self {
      months,
      days,
      nanoseconds,
    }
  }

  /// Returns the string representation of the Duration.
  // TODO: Check really how this is supposed to be displayed
  #[napi]
  #[allow(clippy::inherent_to_string)]
  pub fn to_string(&self) -> String {
    format!(
      "{} months, {} days, {} nanoseconds",
      self.months, self.days, self.nanoseconds
    )
  }
}
