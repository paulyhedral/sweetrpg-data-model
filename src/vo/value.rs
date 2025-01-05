/**
 * Value value object.
 * @paulyhedral
 */
use serde::{Deserialize, Serialize};
use sweetrpg_model_core::vo::auditable::*;
use sweetrpg_model_core::vo::tag::*;


/// Value value object.
/// This value object is a serializable representation of the Value model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueVO {
    pub id: String,
    pub store_id: String,
    pub key_id: String,
    pub snapshot_id: String,
    pub value: String,
    pub notes: String,
    pub tags: Vec<TagVO>,
    pub auditable: AuditableVO,
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {}
