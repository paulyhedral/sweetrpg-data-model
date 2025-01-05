/**
 * Key event object.
 * @paulyhedral
 */
use serde::{Deserialize, Serialize};


/// Key event.
/// This object represents an event on a store's key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyEvent {
    pub id: String,
    pub store_id: String,
    pub event: String,
    pub key: String,
    pub occurred_at: u64,
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::constants::EVENT_EXPRESSION_CHANGED;
    use serde_json;

    #[test]
    fn serialize_key_event() {
        let event1 = KeyEvent {
            id: "12345".to_string(),
            store_id: "67890".to_string(),
            event: EVENT_EXPRESSION_CHANGED.to_string(),
            key: "key".to_string(),
            occurred_at: 12345,
        };

        let json = serde_json::to_string(&event1).unwrap();
        let event2: KeyEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event1.id, event2.id);
        assert_eq!(event1.store_id, event2.store_id);
        assert_eq!(event1.event, event2.event);
        assert_eq!(event1.key, event2.key);
        assert_eq!(event1.occurred_at, event2.occurred_at);
    }
}
