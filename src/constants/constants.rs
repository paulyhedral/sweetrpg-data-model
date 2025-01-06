/**
 * Constants
 * @paulyhedral
 */

// Event types
pub static EVENT_EXPRESSION_CHANGED: &str = "expression_changed";
pub static EVENT_RECALCULATE: &str = "recalculate";
pub static EVENT_VALUE_CHANGED: &str = "value_changed";
pub static EVENT_PING: &str = "ping";

// Response values 
pub static RESPONSE_SUCCESS: &str = "success";
pub static RESPONSE_PONG: &str = "pong";
pub static RESPONSE_ERROR: &str = "error";

// Key types
pub static KEY_TYPE_INTEGER: &str = "number";
pub static KEY_TYPE_STRING: &str = "string";
pub static KEY_TYPE_BOOLEAN: &str = "boolean";
pub static KEY_TYPE_DATA: &str = "data";

// Key encodings
pub static KEY_ENCODING_PLAIN: &str = "plain";
pub static KEY_ENCODING_BASE64: &str = "base64";
