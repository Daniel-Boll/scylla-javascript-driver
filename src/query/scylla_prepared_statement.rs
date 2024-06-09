use scylla::prepared_statement::PreparedStatement;

use crate::cluster::execution_profile::consistency::Consistency;

#[napi]
pub struct ScyllaPreparedStatement {
    pub (crate) prepared: PreparedStatement,
}

#[napi]
impl ScyllaPreparedStatement {

    pub fn new(prepared: PreparedStatement) -> Self {
        Self {
            prepared
        }
    }

    #[napi]
    pub fn set_consistency(&mut self, consistency: Consistency) {
        self.prepared.set_consistency(consistency.into());
    }
}

