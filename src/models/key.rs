use sweetrpg_model_core::models::auditable::*;
use sweetrpg_model_core::models::tag::*;

/// Key model.
/// This model represents a key in a store of key-value information.
#[derive(Debug, Clone)]
pub struct Key {
    pub id: String,
    pub store_id: String,
    pub name: String,
    pub description: String,
    pub type_: String,
    pub encoding: String,
    pub expression: String,
    pub notes: String,
    pub tags: Vec<Tag>,
    pub auditable: Auditable,
}
