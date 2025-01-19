/**
 * Value event object.
 * @paulyhedral
 */
use crate::events::response::*;
use serde::{Deserialize, Serialize};


/// Value event.
/// This object represents a change in a key's value.
/// The usual event type found in this object is {@EVENT_RECALCULATE}.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueEvent {
    pub id: String,
    pub store_id: String,
    pub event: String,
    pub key: String,
    pub occurred_at: u64,
}

/// Value response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueResponse {
    #[serde(flatten)]
    pub base_response: Response,
    // TODO
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::constants::*;
    use serde_json;

    #[test]
    fn serialize_value_event() {
        let event1 = ValueEvent {
            id: "12345".to_string(),
            store_id: "67890".to_string(),
            event: EVENT_RECALCULATE.to_string(),
            key: "key".to_string(),
            occurred_at: 123456789,
        };

        let json = serde_json::to_string(&event1).unwrap();
        let event2: ValueEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event1.id, event2.id);
        assert_eq!(event1.store_id, event2.store_id);
        assert_eq!(event1.event, event2.event);
        assert_eq!(event1.key, event2.key);
        assert_eq!(event1.occurred_at, event2.occurred_at);
    }

    #[test]
    fn serialize_response() {
        let response1 = super::ValueResponse {
            base_response: super::Response {
                status: RESPONSE_SUCCESS.to_string(),
                message: "all good".to_string(),
            },
            // TODO
        };

        let json = serde_json::to_string(&response1).unwrap();
        let response2: super::ValueResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response1.base_response.status, response2.base_response.status);
        assert_eq!(response1.base_response.message, response2.base_response.message);
    }
}
