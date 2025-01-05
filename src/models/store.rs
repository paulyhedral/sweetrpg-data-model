use serde::{Deserialize, Serialize};
/**
 * Store model object.
 * @paulyhedral
 */
use sweetrpg_model_core::models::auditable::*;
use sweetrpg_model_core::models::tag::*;


/// Store model.
/// This model represents a store of key-value information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Store {
    pub id: String,
    pub name: String,
    pub description: String,
    pub current_snapshot_id: String,
    pub notes: String,
    pub tags: Vec<Tag>,
    #[serde(flatten)]
    pub auditable: Auditable,
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {}
