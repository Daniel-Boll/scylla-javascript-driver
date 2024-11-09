/// A double precision float number.
///
/// Due to the nature of numbers in JavaScript, it's hard to distinguish between integers and floats, so this type is used to represent
/// double precision float numbers while any other JS number will be treated as an integer. (This is not the case for BigInts, which are always treated as BigInts).
#[napi]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Double {
  pub(crate) inner: f64,
}

impl From<f64> for Double {
  fn from(inner: f64) -> Self {
    Self { inner }
  }
}

impl From<&Double> for f64 {
  fn from(float: &Double) -> Self {
    float.inner
  }
}

#[napi]
impl Double {
  #[napi(constructor)]
  pub fn new_float(inner: f64) -> Double {
    Double::from(inner)
  }

  #[napi]
  #[allow(clippy::inherent_to_string)]
  pub fn to_string(&self) -> String {
    self.inner.to_string()
  }
}
