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
