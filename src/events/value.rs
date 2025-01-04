use serde::{Deserialize, Serialize};

/// Value event.
/// This object represents a change in a key's value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueEvent {
    pub id: String,
    pub store_id: String,
    pub event: String,
    pub key: String,
    pub occurred_at: u64,
}
