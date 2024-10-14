use std::time::SystemTime;

use crate::internal::functions::{
    encode::{encode_unsafe, EncodeOptions},
    get_randomness::{get_randomness, GetRandomnessOptions},
};

pub struct RowIDOptions<'a> {
    pub char_list: &'a str,
    pub randomness_length: usize,
}

pub fn rowid(opts: RowIDOptions) -> String {
    encode_unsafe(EncodeOptions {
        char_list: opts.char_list,
        system_time: SystemTime::now(),
    }) + &get_randomness(GetRandomnessOptions {
        char_list: opts.char_list,
        randomness_length: opts.randomness_length,
    })
}
