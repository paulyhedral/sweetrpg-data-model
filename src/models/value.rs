use sweetrpg_model_core::models::auditable::*;
use sweetrpg_model_core::models::tag::*;

/// Value model.
/// This model represents a store of key-value information.
#[derive(Debug, Clone)]
pub struct Value {
    pub id: String,
    pub store_id: String,
    pub key_id: String,
    pub snapshot_id: String,
    pub value: String,
    pub notes: String,
    pub tags: Vec<Tag>,
    pub auditable: Auditable,
}
