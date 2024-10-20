use std::{io, time::SystemTime};

use crate::{
    common::configs::{CHAR_LIST, RANDOMNESS_LENGTH},
    function::{
        decode::{DecodeOptions, _decode},
        encode::{EncodeOptions, _encode},
        generate::{GenerateOptions, _generate},
        get_randomness::{GetRandomnessOptions, _get_randomness},
        rowid::{RowIDOptions, _rowid},
        verify::{VerifyOptions, _verify},
    },
};

pub use crate::{
    common::errors::RowIDError,
    function::{generate::GenerateResult, verify::VerifyResult},
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
/// use rowid::base::encode;
///
/// let encoded: String = encode(SystemTime::now()).unwrap();
/// ```
pub fn encode(system_time: SystemTime) -> io::Result<String> {
    _encode(EncodeOptions { char_list: CHAR_LIST, system_time })
}

/// This function decodes the ID into a timestamp in milliseconds.
///
/// ## Example
///
/// ```no_run
/// use std::time::SystemTime;
/// use rowid::base::decode;
///
/// let decoded: SystemTime = decode("ABC123").unwrap();
/// ```
pub fn decode(encoded: &str) -> io::Result<SystemTime> {
    _decode(DecodeOptions { char_list: CHAR_LIST, encoded })
}

/// This function generates an ID based on the input.
///
/// ## Example
///
/// ```no_run
/// use std::time::SystemTime;
/// use rowid::base::{generate, GenerateResult};
///
/// let now: SystemTime = SystemTime::now();
/// let result: GenerateResult = generate(now, Some(22));
/// ```
pub fn generate(
    system_time: SystemTime,
    randomness_length: Option<usize>,
) -> GenerateResult {
    _generate(GenerateOptions {
        char_list: CHAR_LIST,
        system_time,
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
pub fn verify(encoded: &str) -> VerifyResult {
    _verify(VerifyOptions { char_list: CHAR_LIST, encoded })
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
