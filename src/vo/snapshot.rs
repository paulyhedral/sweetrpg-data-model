/**
 * Snapshot value object.
 * @paulyhedral
 */
use serde::{Deserialize, Serialize};
use sweetrpg_model_core::vo::auditable::*;
use sweetrpg_model_core::vo::tag::*;


/// Snapshot value object.
/// This value object is a serializable representation of the Snapshot model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotVO {
    pub id: String,
    pub store_id: String,
    pub name: String,
    pub notes: String,
    pub tags: Vec<TagVO>,
    #[serde(flatten)]
    pub auditable: AuditableVO,
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use sweetrpg_model_core::vo::auditable::AuditableVO;

    #[test]
    fn serialize_snapshot_vo() {
        let snapshot1 = SnapshotVO {
            id: "12345".to_string(),
            store_id: "67890".to_string(),
            name: "snapshot".to_string(),
            notes: "notes".to_string(),
            tags: vec![],
            auditable: AuditableVO {
                created_at: 123456789,
                created_by: "user".to_string(),
                updated_at: 123456789,
                updated_by: "user".to_string(),
                deleted_at: None,
                deleted_by: None,
            },
        };

        let json = serde_json::to_string(&snapshot1).unwrap();
        let snapshot2: SnapshotVO = serde_json::from_str(&json).unwrap();

        assert_eq!(snapshot1.id, snapshot2.id);
        assert_eq!(snapshot1.store_id, snapshot2.store_id);
        assert_eq!(snapshot1.name, snapshot2.name);
        assert_eq!(snapshot1.notes, snapshot2.notes);
        assert_eq!(snapshot1.auditable.created_at, snapshot2.auditable.created_at);
        assert_eq!(snapshot1.auditable.created_by, snapshot2.auditable.created_by);
        assert_eq!(snapshot1.auditable.updated_at, snapshot2.auditable.updated_at);
        assert_eq!(snapshot1.auditable.updated_by, snapshot2.auditable.updated_by);
        assert_eq!(snapshot1.auditable.deleted_at, snapshot2.auditable.deleted_at);
        assert_eq!(snapshot1.auditable.deleted_by, snapshot2.auditable.deleted_by);

    }
}
