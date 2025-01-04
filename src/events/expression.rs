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
