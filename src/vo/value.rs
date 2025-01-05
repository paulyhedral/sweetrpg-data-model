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
    #[serde(flatten)]
    pub auditable: AuditableVO,
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use sweetrpg_model_core::vo::auditable::AuditableVO;
    use sweetrpg_model_core::vo::tag::TagVO;

    #[test]
    fn serialize_value_vo() {
        let value1 = ValueVO {
            id: "12345".to_string(),
            store_id: "67890".to_string(),
            key_id: "abcde".to_string(),
            snapshot_id: "fghij".to_string(),
            value: "value".to_string(),
            notes: "notes".to_string(),
            tags: vec![ TagVO { name: "tag".to_string(), value: "value".to_string() } ],
            auditable: AuditableVO {
                created_at: 123456789,
                created_by: "user".to_string(),
                updated_at: 123456789,
                updated_by: "user".to_string(),
                deleted_at: None,
                deleted_by: None,
            },
        };

        let json = serde_json::to_string(&value1).unwrap();
        let value2: ValueVO = serde_json::from_str(&json).unwrap();

        assert_eq!(value1.id, value2.id);
        assert_eq!(value1.store_id, value2.store_id);
        assert_eq!(value1.key_id, value2.key_id);
        assert_eq!(value1.snapshot_id, value2.snapshot_id);
        assert_eq!(value1.value, value2.value);
        assert_eq!(value1.notes, value2.notes);
        assert_eq!(value1.auditable.created_at, value2.auditable.created_at);
        assert_eq!(value1.auditable.created_by, value2.auditable.created_by);
        assert_eq!(value1.auditable.updated_at, value2.auditable.updated_at);
        assert_eq!(value1.auditable.updated_by, value2.auditable.updated_by);
        assert_eq!(value1.auditable.deleted_at, value2.auditable.deleted_at);
        assert_eq!(value1.auditable.deleted_by, value2.auditable.deleted_by);
    }
}
