/// A float number.
///
/// Due to the nature of numbers in JavaScript, it's hard to distinguish between integers and floats, so this type is used to represent
/// float numbers while any other JS number will be treated as an integer. (This is not the case for BigInts, which are always treated as BigInts).
#[napi]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Float {
  pub(crate) inner: f32,
}

impl From<f32> for Float {
  fn from(inner: f32) -> Self {
    Self { inner }
  }
}

impl From<&Float> for f32 {
  fn from(float: &Float) -> Self {
    float.inner
  }
}

#[napi]
impl Float {
  #[napi(constructor)]
  pub fn new_float(inner: f64) -> Float {
    Float::from(inner as f32)
  }

  #[napi]
  #[allow(clippy::inherent_to_string)]
  pub fn to_string(&self) -> String {
    self.inner.to_string()
  }
}
