pub mod consistency;

use self::consistency::Consistency;

#[napi(object)]
#[derive(Copy, Clone)]
pub struct ExecutionProfile {
  pub consistency: Option<Consistency>,
  pub request_timeout: Option<u32>,
}

impl ExecutionProfile {
  fn create_execution_profile(self) -> scylla::ExecutionProfile {
    let mut ec_builder = scylla::transport::ExecutionProfile::builder();

    if let Some(consistency) = self.consistency {
      ec_builder = ec_builder.consistency(consistency.into());
    }

    if let Some(request_timeout) = self.request_timeout {
      ec_builder =
        ec_builder.request_timeout(Some(std::time::Duration::from_secs(request_timeout.into())));
    }

    ec_builder.build()
  }

  pub(crate) fn into_handle(self) -> scylla::execution_profile::ExecutionProfileHandle {
    self.create_execution_profile().into_handle()
  }

  pub(crate) fn into_handle_with_label(
    self,
    label: String,
  ) -> scylla::execution_profile::ExecutionProfileHandle {
    self
      .create_execution_profile()
      .into_handle_with_label(label)
  }
}
