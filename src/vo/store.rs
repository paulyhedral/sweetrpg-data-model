/**
 * Store value object.
 * @paulyhedral
 */
use serde::{Deserialize, Serialize};
use sweetrpg_model_core::vo::auditable::*;
use sweetrpg_model_core::vo::tag::*;


/// Store value object.
/// This value object is a serializable representation of the Store model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreVO {
    pub id: String,
    pub name: String,
    pub description: String,
    pub current_snapshot_id: String,
    pub notes: String,
    pub tags: Vec<TagVO>,
    pub auditable: AuditableVO,
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {}
