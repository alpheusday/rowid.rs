use std::io;

use crate::{
    common::configs::{CHAR_LIST, RANDOMNESS_LENGTH},
    functions::{
        decode::{_decode, DecodeOptions},
        encode::{_encode, EncodeOptions},
        generate::{_generate, GenerateOptions},
        get_randomness::{_get_randomness, GetRandomnessOptions},
        rowid::{_rowid, RowIDOptions},
        verify::{_verify, VerifyOptions},
    },
};

pub use crate::{
    common::errors::RowIDError,
    functions::{generate::GenerateResult, verify::VerifyResult},
};

/// This function generates a 32-character unique ID
/// that is almost impossible to duplicate.
///
/// ## Example
///
/// ```no_run
/// use rowid::base::rowid;
///
/// let id: String = rowid();
/// ```
pub fn rowid() -> String {
    _rowid(RowIDOptions {
        char_list: CHAR_LIST,
        randomness_length: RANDOMNESS_LENGTH,
    })
}

/// This function encodes the timestamp in milliseconds
/// into an ID without randomness.
///
/// ## Example
///
/// ```no_run
/// use std::time::SystemTime;
/// use rowid::{
///     base::encode,
///     time::system_time_to_timestamp,
/// };
///
/// let now: usize = system_time_to_timestamp(SystemTime::now());
///
/// let encoded: String = encode(now).unwrap();
/// ```
pub fn encode(timestamp: usize) -> io::Result<String> {
    _encode(EncodeOptions { char_list: CHAR_LIST, timestamp })
}

/// This function decodes the ID into a timestamp in milliseconds.
///
/// ## Example
///
/// ```no_run
/// use rowid::base::decode;
///
/// let decoded: usize = decode("ABC123").unwrap();
/// ```
pub fn decode<S: AsRef<str>>(encoded: S) -> io::Result<usize> {
    _decode(DecodeOptions { char_list: CHAR_LIST, encoded: encoded.as_ref() })
}

/// This function generates an ID based on the input.
///
/// ## Example
///
/// ```no_run
/// use std::time::SystemTime;
/// use rowid::{
///     base::{generate, GenerateResult},
///     time::system_time_to_timestamp,
/// };
///
/// let now: usize = system_time_to_timestamp(SystemTime::now());
///
/// let result: GenerateResult = generate(now, Some(22));
/// ```
pub fn generate(
    timestamp: usize,
    randomness_length: Option<usize>,
) -> GenerateResult {
    _generate(GenerateOptions {
        char_list: CHAR_LIST,
        timestamp,
        randomness_length: match randomness_length {
            | Some(l) => l,
            | None => RANDOMNESS_LENGTH,
        },
    })
}

/// This function verifies if the ID is valid and natural.
///
/// ## Example
///
/// ```no_run
/// use rowid::base::{verify, VerifyResult};
///
/// let result: VerifyResult = verify("ABC123");
/// ```
pub fn verify<S: AsRef<str>>(encoded: S) -> VerifyResult {
    _verify(VerifyOptions { char_list: CHAR_LIST, encoded: encoded.as_ref() })
}

/// This function generates randomness.
///
/// ## Example
///
/// ```no_run
/// use rowid::base::get_randomness;
///
/// let randomness: String = get_randomness(10);
/// ```
pub fn get_randomness(randomness_length: usize) -> String {
    _get_randomness(GetRandomnessOptions {
        char_list: CHAR_LIST,
        randomness_length,
    })
}
