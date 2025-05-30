use std::{io, time::SystemTime};

use crate::{
    functions::decode::{_decode, DecodeOptions},
    time::timestamp_to_system_time,
};

pub struct VerifyOptions<CharList: AsRef<str>, Encoded: AsRef<str>>
where
    CharList: AsRef<str>,
{
    pub char_list: CharList,
    pub encoded: Encoded,
}

/// Result of the `verify` function.
#[derive(Debug)]
pub struct VerifyResult {
    /// Tells whether the verification is success or not.
    pub success: bool,
    /// Decoded timestamp based on the ID.
    pub result: Option<usize>,
    /// Tells whether the ID is natural or not.
    pub natural: Option<bool>,
    /// Error when the verification is failed.
    pub error: Option<io::Error>,
}

pub fn _verify<CharList: AsRef<str>, Encoded: AsRef<str>>(
    opts: VerifyOptions<CharList, Encoded>
) -> VerifyResult {
    let result: usize = match _decode(DecodeOptions {
        char_list: opts.char_list,
        encoded: opts.encoded,
    }) {
        | Ok(res) => res,
        | Err(e) => {
            return VerifyResult {
                success: false,
                result: None,
                natural: None,
                error: Some(e),
            };
        },
    };

    VerifyResult {
        success: true,
        result: Some(result),
        natural: Some(timestamp_to_system_time(result) < SystemTime::now()),
        error: None,
    }
}
