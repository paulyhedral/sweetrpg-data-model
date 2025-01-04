use crate::vo::store::StoreVO;
use serde::{Deserialize, Serialize};
use sweetrpg_model_core::vo::auditable::*;
use sweetrpg_model_core::vo::tag::*;

/// Key value object.
/// This value object is a serializable representation of the Key model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyVO {
    pub id: String,
    pub store: StoreVO,
    pub name: String,
    pub description: String,
    pub type_: String,
    pub encoding: String,
    pub expression: String,
    pub notes: String,
    pub tags: Vec<TagVO>,
    pub auditable: AuditableVO,
}
