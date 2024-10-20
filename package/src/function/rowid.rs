use std::time::SystemTime;

use crate::function::{
    encode::{encode_unsafe, EncodeOptions},
    get_randomness::{GetRandomnessOptions, _get_randomness},
};

pub struct RowIDOptions<'a> {
    pub char_list: &'a str,
    pub randomness_length: usize,
}

pub fn _rowid(opts: RowIDOptions) -> String {
    encode_unsafe(EncodeOptions {
        char_list: opts.char_list,
        system_time: SystemTime::now(),
    }) + &_get_randomness(GetRandomnessOptions {
        char_list: opts.char_list,
        randomness_length: opts.randomness_length,
    })
}
