use std::{io, time::SystemTime};

use crate::function::{
    encode::{EncodeOptions, _encode},
    get_randomness::{GetRandomnessOptions, _get_randomness},
};

pub struct GenerateOptions<'a> {
    pub char_list: &'a str,
    pub system_time: SystemTime,
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

pub fn _generate(opts: GenerateOptions) -> GenerateResult {
    let encoded: String = match _encode(EncodeOptions {
        char_list: opts.char_list,
        system_time: opts.system_time,
    }) {
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
            char_list: opts.char_list,
            randomness_length: opts.randomness_length,
        });

    GenerateResult {
        success: true,
        result: Some(format!("{}{}", encoded, extra_randomness_length)),
        error: None,
    }
}
