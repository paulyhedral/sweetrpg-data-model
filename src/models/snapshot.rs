/**
 * Snapshot model object.
 * @paulyhedral
 */
use sweetrpg_model_core::models::auditable::*;
use sweetrpg_model_core::models::tag::*;


/// Snapshot model.
/// This model represents a snapshot of keys and values in a store.
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub id: String,
    pub store_id: String,
    pub name: String,
    pub notes: String,
    pub tags: Vec<Tag>,
    pub auditable: Auditable,
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {}
