/**
 * Expression event
 * @paulyhedral
 */
use crate::events::response::*;
use serde::{Deserialize, Serialize};


/// Expression event.
/// This object represents a change in a key's expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressionEvent {
    pub id: String,
    pub store_id: String,
    pub event: String,
    pub key: String,
    pub occurred_at: u64,
}

/// Expression response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressionResponse {
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
    fn serialize_expression_event() {
        let event1 = ExpressionEvent {
            id: "12345".to_string(),
            store_id: "12345".to_string(),
            event: EVENT_EXPRESSION_CHANGED.to_string(),
            key: "key".to_string(),
            occurred_at: 1234567890,
        };

        let json = serde_json::to_string(&event1).unwrap();
        let event2: ExpressionEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event2.id, "12345");
        assert_eq!(event2.store_id, "12345");
        assert_eq!(event2.event, EVENT_EXPRESSION_CHANGED);
        assert_eq!(event2.key, "key");
        assert_eq!(event2.occurred_at, 1234567890);
    }

    #[test]
    fn serialize_response() {
        let response1 = super::ExpressionResponse {
            base_response: Response {
                status: RESPONSE_SUCCESS.to_string(),
                message: "all good".to_string(),
            },
            // TODO
        };

        let json = serde_json::to_string(&response1).unwrap();
        let response2: super::ExpressionResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response1.base_response.status, response2.base_response.status);
        assert_eq!(response1.base_response.message, response2.base_response.message);
    }
}
