use crate::vo::snapshot::SnapshotVO;
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
    pub current_snapshot: SnapshotVO,
    pub notes: String,
    pub tags: Vec<TagVO>,
    pub auditable: AuditableVO,
}
