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
mod tests {
    use super::*;
    use sweetrpg_model_core::vo::auditable::AuditableVO;

    #[test]
    fn serialize_store_vo() {
        let store1 = StoreVO {
            id: "12345".to_string(),
            name: "store".to_string(),
            description: "description".to_string(),
            current_snapshot_id: "67890".to_string(),
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

        let json = serde_json::to_string(&store1).unwrap();
        let store2: StoreVO = serde_json::from_str(&json).unwrap();

        assert_eq!(store1.id, store2.id);
        assert_eq!(store1.name, store2.name);
        assert_eq!(store1.description, store2.description);
        assert_eq!(store1.current_snapshot_id, store2.current_snapshot_id);
        assert_eq!(store1.notes, store2.notes);
        assert_eq!(store1.auditable.created_at, store2.auditable.created_at);
        assert_eq!(store1.auditable.created_by, store2.auditable.created_by);
        assert_eq!(store1.auditable.updated_at, store2.auditable.updated_at);
        assert_eq!(store1.auditable.updated_by, store2.auditable.updated_by);
        assert_eq!(store1.auditable.deleted_at, store2.auditable.deleted_at);
        assert_eq!(store1.auditable.deleted_by, store2.auditable.deleted_by);
    }
}
