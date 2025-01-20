/**
 * Constants
 * @paulyhedral
 */

// Event types
pub const EVENT_EXPRESSION_CHANGED: &str = "expression_changed";
pub const EVENT_RECALCULATE: &str = "recalculate";
pub const EVENT_VALUE_CHANGED: &str = "value_changed";
pub const EVENT_PING: &str = "ping";

// Response values
pub const RESPONSE_SUCCESS: &str = "success";
pub const RESPONSE_PONG: &str = "pong";
pub const RESPONSE_IGNORED: &str = "ignored";
pub const RESPONSE_ERROR: &str = "error";

// Key types
pub const KEY_TYPE_INTEGER: &str = "number";
pub const KEY_TYPE_STRING: &str = "string";
pub const KEY_TYPE_BOOLEAN: &str = "boolean";
pub const KEY_TYPE_DATA: &str = "data";

// Key encodings
pub const KEY_ENCODING_PLAIN: &str = "plain";
pub const KEY_ENCODING_BASE64: &str = "base64";
