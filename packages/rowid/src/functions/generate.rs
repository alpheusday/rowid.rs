use std::io;
use std::time::SystemTime;

use crate::functions::encode::{encode, EncodeOptions};
use crate::functions::get_randomness::{get_randomness, GetRandomnessOptions};

pub struct GenerateOptions {
    pub char_list: String,
    pub system_time: SystemTime,
    pub randomness_length: usize,
}

/// Result of the `generate` function.
pub struct GenerateResult {
    /// Tells whether the generation is success or not.
    pub success: bool,
    /// Encoded timestamp and randomness based on the input.
    pub result: Option<String>,
    /// Error when the generation is failed.
    pub error: Option<io::Error>,
}

pub fn generate(opts: GenerateOptions) -> GenerateResult {
    let encoded: String = match encode(EncodeOptions {
        char_list: opts.char_list.clone(),
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
        get_randomness(GetRandomnessOptions {
            char_list: opts.char_list.clone(),
            randomness_length: opts.randomness_length,
        });

    GenerateResult {
        success: true,
        result: Some(format!("{}{}", encoded, extra_randomness_length)),
        error: None,
    }
}
