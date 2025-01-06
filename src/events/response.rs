/**
 * Response object
 */
use serde::{Deserialize, Serialize};


/// Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub status: String,
    pub message: String,
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::constants::constants::RESPONSE_SUCCESS;

    #[test]
    fn serialize_response() {
        let response1 = super::Response {
            status: RESPONSE_SUCCESS.to_string(),
            message: "all good".to_string(),
        };

        let json = serde_json::to_string(&response1).unwrap();
        let response2: super::Response = serde_json::from_str(&json).unwrap();

        assert_eq!(response1.status, response2.status);
        assert_eq!(response1.message, response2.message);
    }
}
