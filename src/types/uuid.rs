use napi::Result;
use scylla::frame::value::CqlTimeuuid;

#[napi()]
#[derive(Debug, Clone, Copy)]
pub struct Uuid {
  pub(crate) uuid: uuid::Uuid,
}

impl From<uuid::Uuid> for Uuid {
  fn from(uuid: uuid::Uuid) -> Self {
    Self { uuid }
  }
}

impl From<Uuid> for uuid::Uuid {
  fn from(uuid: Uuid) -> Self {
    uuid.uuid
  }
}

impl From<CqlTimeuuid> for Uuid {
  fn from(uuid: CqlTimeuuid) -> Self {
    Self {
      uuid: *uuid.as_ref(), // NOTE: not sure if this is the best way
    }
  }
}

impl Uuid {
  pub(crate) fn get_inner(&self) -> uuid::Uuid {
    self.uuid
  }
}

#[napi]
impl Uuid {
  /// Generates a random UUID v4.
  #[napi(js_name = "randomV4")]
  pub fn random_v4() -> Self {
    Self {
      uuid: uuid::Uuid::new_v4(),
    }
  }

  /// Parses a UUID from a string. It may fail if the string is not a valid UUID.
  #[napi]
  pub fn from_string(str: String) -> Result<Uuid> {
    let uuid = uuid::Uuid::parse_str(&str).map_err(|e| {
      napi::Error::new(
        napi::Status::GenericFailure,
        format!("Failed to parse UUID: {}", e),
      )
    })?;

    Ok(Self { uuid })
  }

  /// Returns the string representation of the UUID.
  #[napi]
  #[allow(clippy::inherent_to_string)]
  pub fn to_string(&self) -> String {
    self.uuid.to_string()
  }
}
