use std::{io, time::SystemTime};

use crate::functions::decode::{DecodeOptions, _decode};

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
    /// Decoded system time based on the ID.
    pub result: Option<SystemTime>,
    /// Tells whether the ID is natural or not.
    pub natural: Option<bool>,
    /// Error when the verification is failed.
    pub error: Option<io::Error>,
}

pub fn _verify<CharList: AsRef<str>, Encoded: AsRef<str>>(
    opts: VerifyOptions<CharList, Encoded>
) -> VerifyResult {
    let result: SystemTime = match _decode(DecodeOptions {
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
        natural: Some(result < SystemTime::now()),
        error: None,
    }
}
