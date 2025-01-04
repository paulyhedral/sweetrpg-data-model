use crate::vo::store::StoreVO;
use serde::{Deserialize, Serialize};
use sweetrpg_model_core::vo::auditable::*;
use sweetrpg_model_core::vo::tag::*;

/// Key value object.
/// This value object is a serializable representation of the Key model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyVO {
    ID: String,
    Store: StoreVO,
    Name: String,
    Description: String,
    Type: String,
    Encoding: String,
    Expression: String,
    Notes: String,
    Tags: Vec<TagVO>,
    pub auditable: AuditableVO,
}
