use std::{collections::HashMap, net::IpAddr};

use napi::Either;
use serde::Serialize;

use crate::helpers::query_results::WithMapType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CqlTimestampWrapper(pub scylla::frame::value::CqlTimestamp);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CqlTimeuuidWrapper(pub scylla::frame::value::CqlTimeuuid);

impl Serialize for CqlTimestampWrapper {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_i64(self.0.0)
  }
}

impl Serialize for CqlTimeuuidWrapper {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(format!("{}", self.0).as_str())
  }
}

/// Tracing info retrieved from `system_traces.sessions`
/// with all events from `system_traces.events`
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TracingInfo {
  pub client: Option<IpAddr>,
  pub command: Option<String>,
  pub coordinator: Option<IpAddr>,
  pub duration: Option<i32>,
  pub parameters: Option<HashMap<String, String>>,
  pub request: Option<String>,
  /// started_at is a timestamp - time since unix epoch
  pub started_at: Option<CqlTimestampWrapper>,

  pub events: Vec<TracingEvent>,
}

/// A single event happening during a traced query
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TracingEvent {
  pub event_id: CqlTimeuuidWrapper,
  pub activity: Option<String>,
  pub source: Option<IpAddr>,
  pub source_elapsed: Option<i32>,
  pub thread: Option<String>,
}

impl From<TracingInfo> for serde_json::Value {
  fn from(info: TracingInfo) -> Self {
    serde_json::json!(info)
  }
}

impl From<scylla::tracing::TracingInfo> for TracingInfo {
  fn from(info: scylla::tracing::TracingInfo) -> Self {
    Self {
      client: info.client,
      command: info.command,
      coordinator: info.coordinator,
      duration: info.duration,
      parameters: info.parameters,
      request: info.request,
      started_at: info.started_at.map(CqlTimestampWrapper),
      events: info.events.into_iter().map(TracingEvent::from).collect(),
    }
  }
}

impl From<scylla::tracing::TracingEvent> for TracingEvent {
  fn from(event: scylla::tracing::TracingEvent) -> Self {
    Self {
      event_id: CqlTimeuuidWrapper(event.event_id),
      activity: event.activity,
      source: event.source,
      source_elapsed: event.source_elapsed,
      thread: event.thread,
    }
  }
}

pub type TracingReturn =
  HashMap<String, Either<Vec<HashMap<String, WithMapType>>, serde_json::Value>>;
