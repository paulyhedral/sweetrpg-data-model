/**
 * Key event object.
 * @paulyhedral
 */
use crate::events::response::*;
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

/// Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyResponse {
    #[serde(flatten)]
    pub base_response: Response ,
    pub count: u64,
    pub keys: Vec<String>,
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

    #[test]
    fn serialize_response() {
        let response1 = super::KeyResponse {
            base_response: Response {
                status: RESPONSE_SUCCESS.to_string(),
                message: "all good".to_string(),
            },
            count: 1,
            keys: vec!["key1".to_string(), "key2".to_string()],
        };

        let json = serde_json::to_string(&response1).unwrap();
        let response2: super::KeyResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response1.base_response.status, response2.base_response.status);
        assert_eq!(response1.base_response.message, response2.base_response.message);
        assert_eq!(response1.count, response2.count);
        assert_eq!(response1.keys, response2.keys);
    }
}
