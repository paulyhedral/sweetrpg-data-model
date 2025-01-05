/**
 * Key value object.
 * @paulyhedral
 */
use serde::{Deserialize, Serialize};
use sweetrpg_model_core::vo::auditable::*;
use sweetrpg_model_core::vo::tag::*;


/// Key value object.
/// This value object is a serializable representation of the Key model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyVO {
    pub id: String,
    pub store_id: String,
    pub name: String,
    pub description: String,
    pub type_: String,
    pub encoding: String,
    pub expression: String,
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
    fn serialize_key_vo() {
        let key1 = KeyVO {
            id: "12345".to_string(),
            store_id: "67890".to_string(),
            name: "key".to_string(),
            description: "description".to_string(),
            type_: "type".to_string(),
            encoding: "encoding".to_string(),
            expression: "expression".to_string(),
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

        let json = serde_json::to_string(&key1).unwrap();
        let key2: KeyVO = serde_json::from_str(&json).unwrap();

        assert_eq!(key1.id, key2.id);
        assert_eq!(key1.store_id, key2.store_id);
        assert_eq!(key1.name, key2.name);
        assert_eq!(key1.description, key2.description);
        assert_eq!(key1.type_, key2.type_);
        assert_eq!(key1.encoding, key2.encoding);
        assert_eq!(key1.expression, key2.expression);
        assert_eq!(key1.notes, key2.notes);
        // assert_eq!(key1.tags, key2.tags);
        assert_eq!(key1.auditable.created_at, key2.auditable.created_at);
        assert_eq!(key1.auditable.created_by, key2.auditable.created_by);
        assert_eq!(key1.auditable.updated_at, key2.auditable.updated_at);
        assert_eq!(key1.auditable.updated_by, key2.auditable.updated_by);
        assert_eq!(key1.auditable.deleted_at, key2.auditable.deleted_at);
        assert_eq!(key1.auditable.deleted_by, key2.auditable.deleted_by);
    }
}
