use std::collections::HashMap;
use std::sync::Arc;

use napi::bindgen_prelude::Either3;
use scylla::transport::ClusterData;
use scylla::transport::topology::{Keyspace, MaterializedView, Strategy, Table};

// ============= ClusterData ============= //
#[napi]
pub struct ScyllaClusterData {
  inner: Arc<ClusterData>,
}

impl From<Arc<ClusterData>> for ScyllaClusterData {
  fn from(cluster_data: Arc<ClusterData>) -> Self {
    ScyllaClusterData {
      inner: cluster_data,
    }
  }
}

#[napi]
impl ScyllaClusterData {
  #[napi]
  /// Access keyspaces details collected by the driver Driver collects various schema details like
  /// tables, partitioners, columns, types. They can be read using this method
  pub fn get_keyspace_info(&self) -> Option<HashMap<String, ScyllaKeyspace>> {
    let keyspaces_info = self.inner.get_keyspace_info();

    if keyspaces_info.is_empty() {
      None
    } else {
      Some(
        keyspaces_info
          .iter()
          .map(|(k, v)| (k.clone(), ScyllaKeyspace::from((*v).clone())))
          .collect(),
      )
    }
  }
}
// ======================================= //

// ============= Keyspace ============= //
#[napi(object)]
#[derive(Clone)]
pub struct ScyllaKeyspace {
  pub strategy: ScyllaStrategy,
  pub tables: HashMap<String, ScyllaTable>,
  pub views: HashMap<String, ScyllaMaterializedView>,
  // pub user_defined_types: HashMap<String, ScyllaUserDefinedType>,
}

impl From<Keyspace> for ScyllaKeyspace {
  fn from(keyspace: Keyspace) -> Self {
    ScyllaKeyspace {
      tables: keyspace
        .tables
        .into_iter()
        .map(|(k, v)| (k, ScyllaTable::from(v)))
        .collect(),
      views: keyspace
        .views
        .into_iter()
        .map(|(k, v)| (k, ScyllaMaterializedView::from(v)))
        .collect(),
      strategy: keyspace.strategy.into(),
      // TODO: Implement ScyllaUserDefinedType
      // user_defined_types: keyspace.user_defined_types.into_iter().map(|(k, v)| (k, ScyllaUserDefinedType::from(v))).collect(),
    }
  }
}
// ======================================= //

// ============= Strategy ============= //
#[napi(object)]
#[derive(Clone)]
pub struct ScyllaStrategy {
  pub kind: String,
  pub data: Option<Either3<SimpleStrategy, NetworkTopologyStrategy, Other>>,
}

#[napi(object)]
#[derive(Clone)]
pub struct SimpleStrategy {
  pub replication_factor: u32,
}

#[napi(object)]
#[derive(Clone)]
pub struct NetworkTopologyStrategy {
  pub datacenter_repfactors: HashMap<String, i32>,
}

#[napi(object)]
#[derive(Clone)]
pub struct Other {
  pub name: String,
  pub data: HashMap<String, String>,
}

impl From<Strategy> for ScyllaStrategy {
  fn from(strategy: Strategy) -> Self {
    match strategy {
      Strategy::SimpleStrategy { replication_factor } => ScyllaStrategy {
        kind: "SimpleStrategy".to_string(),
        data: Some(Either3::A(SimpleStrategy {
          replication_factor: replication_factor as u32,
        })),
      },
      Strategy::NetworkTopologyStrategy {
        datacenter_repfactors,
      } => ScyllaStrategy {
        kind: "NetworkTopologyStrategy".to_string(),
        data: Some(Either3::B(NetworkTopologyStrategy {
          datacenter_repfactors: datacenter_repfactors
            .into_iter()
            .map(|(k, v)| (k, v as i32))
            .collect(),
        })),
      },
      Strategy::Other { name, data } => ScyllaStrategy {
        kind: name.clone(),
        data: Some(Either3::C(Other {
          name: name.clone(),
          data,
        })),
      },
      Strategy::LocalStrategy => ScyllaStrategy {
        kind: "LocalStrategy".to_string(),
        data: None,
      },
    }
  }
}
// ======================================= //

// ============= Table ============= //
#[napi(object)]
#[derive(Clone)]
pub struct ScyllaTable {
  pub columns: Vec<String>,
  pub partition_key: Vec<String>,
  pub clustering_key: Vec<String>,
  pub partitioner: Option<String>,
}

impl From<Table> for ScyllaTable {
  fn from(table: Table) -> Self {
    ScyllaTable {
      columns: table.columns.clone().into_keys().collect::<Vec<String>>(),
      partition_key: table.partition_key.clone(),
      clustering_key: table.clustering_key.clone(),
      partitioner: table.partitioner.clone(),
    }
  }
}
// ======================================= //

// ============= MaterializedView ============= //
#[napi(object)]
#[derive(Clone)]
pub struct ScyllaMaterializedView {
  pub view_metadata: ScyllaTable,
  pub base_table_name: String,
}

impl From<MaterializedView> for ScyllaMaterializedView {
  fn from(view: MaterializedView) -> Self {
    ScyllaMaterializedView {
      view_metadata: ScyllaTable::from(view.view_metadata),
      base_table_name: view.base_table_name,
    }
  }
}
// ======================================= //
