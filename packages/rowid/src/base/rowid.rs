use std::io;
use std::time::SystemTime;

use crate::common::configs::{CHAR_LIST, RANDOMNESS_LENGTH};
use crate::functions::decode::{decode as _decode, DecodeOptions};
use crate::functions::encode::{encode as _encode, EncodeOptions};
use crate::functions::generate::{
    generate as _generate, GenerateOptions, GenerateResult,
};
use crate::functions::get_randomness::{
    get_randomness as _get_randomness, GetRandomnessOptions,
};
use crate::functions::rowid::{rowid as _rowid, RowIDOptions};
use crate::functions::verify::{
    verify as _verify, VerifyOptions, VerifyResult,
};

/// This function generates a 32-character unique ID
/// that is almost impossible to duplicate.
///
/// ## Example
///
/// ```no_run
/// use rowid::rowid;
///
/// let id: String = rowid();
/// ```
pub fn rowid() -> String {
    _rowid(RowIDOptions {
        char_list: CHAR_LIST.to_string(),
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
/// use rowid::encode;
///
/// let encoded: String = encode(SystemTime::now()).unwrap();
/// ```
pub fn encode(system_time: SystemTime) -> io::Result<String> {
    _encode(EncodeOptions { char_list: CHAR_LIST.to_string(), system_time })
}

/// This function decodes the ID into a timestamp in milliseconds.
///
/// ## Example
///
/// ```no_run
/// use std::time::SystemTime;
/// use rowid::decode;
///
/// let decoded: SystemTime = decode("ABC123".to_string()).unwrap();
/// ```
pub fn decode(encoded: String) -> io::Result<SystemTime> {
    _decode(DecodeOptions { char_list: CHAR_LIST.to_string(), encoded })
}

/// This function generates an ID based on the input.
///
/// ## Example
///
/// ```no_run
/// use std::time::SystemTime;
/// use rowid::{generate, GenerateResult};
///
/// let now: SystemTime = SystemTime::now();
/// let result: GenerateResult = generate(now, Some(22));
/// ```
pub fn generate(
    system_time: SystemTime,
    randomness_length: Option<usize>,
) -> GenerateResult {
    _generate(GenerateOptions {
        char_list: CHAR_LIST.to_string(),
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
/// use rowid::{verify, VerifyResult};
///
/// let result: VerifyResult = verify("ABC123".to_string());
/// ```
pub fn verify(encoded: String) -> VerifyResult {
    _verify(VerifyOptions { char_list: CHAR_LIST.to_string(), encoded })
}

/// This function generates randomness.
///
/// ## Example
///
/// ```no_run
/// use rowid::get_randomness;
///
/// let randomness: String = get_randomness(10);
/// ```
pub fn get_randomness(randomness_length: usize) -> String {
    _get_randomness(GetRandomnessOptions {
        char_list: CHAR_LIST.to_string(),
        randomness_length,
    })
}
