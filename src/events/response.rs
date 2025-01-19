/**
 * Generic response
 * @paulyhedral
 */
use serde::{Deserialize, Serialize};


/// Generic response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub status: String,
    pub message: String,
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::constants::constants::RESPONSE_PONG;
    use serde_json;

    #[test]
    fn serialize_response() {
        let response1 = super::Response {
            status: RESPONSE_PONG.to_string(),
            message: "pong".to_string(),
        };

        let json = serde_json::to_string(&response1).unwrap();
        let response2: super::Response = serde_json::from_str(&json).unwrap();

        assert_eq!(response1.status, response2.status);
        assert_eq!(response1.message, response2.message);
    }
}
