// encode

/// `encode` fn -> `system_time` -> invalid error
pub const ENCODE_ST_INVALID_ERR: &str =
    "System time must not before the Unix epoch";

// decode

/// `decode` fn -> `encoded` -> length error
pub const DECODE_ECD_LENGTH_ERR: &str =
    "Encoded is not long enough to be decoded";

/// `decode` fn -> `encoded` -> invalid error
pub const DECODE_ECD_INVALID_ERR: &str = "Encoded is not valid";

// rowid with config

/// `RowIDWithConfig` struct -> `done` fn -> `char_list` length error
pub const RWC_DONE_CHAR_LIST_LENGTH_ERR: &str =
    "The length of char_list must be longer or equal to 28";
