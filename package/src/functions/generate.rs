use std::io;

use crate::functions::{
    encode::{_encode, EncodeOptions},
    get_randomness::{_get_randomness, GetRandomnessOptions},
};

pub struct GenerateOptions<CharList: AsRef<str>> {
    pub char_list: CharList,
    pub timestamp: usize,
    pub randomness_length: usize,
}

/// Result of the `generate` function.
#[derive(Debug)]
pub struct GenerateResult {
    /// Tells whether the generation is success or not.
    pub success: bool,
    /// Encoded timestamp and randomness based on the input.
    pub result: Option<String>,
    /// Error when the generation is failed.
    pub error: Option<io::Error>,
}

pub fn _generate<CharList: AsRef<str>>(
    opts: GenerateOptions<CharList>
) -> GenerateResult {
    let char_list: &str = opts.char_list.as_ref();

    let encoded: String =
        match _encode(EncodeOptions { char_list, timestamp: opts.timestamp }) {
            | Ok(res) => res,
            | Err(e) => {
                return GenerateResult {
                    success: false,
                    result: None,
                    error: Some(e),
                };
            },
        };

    let extra_randomness_length: String =
        _get_randomness(GetRandomnessOptions {
            char_list,
            randomness_length: opts.randomness_length,
        });

    GenerateResult {
        success: true,
        result: Some(format!("{}{}", encoded, extra_randomness_length)),
        error: None,
    }
}
