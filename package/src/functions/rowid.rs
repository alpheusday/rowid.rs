use std::time::SystemTime;

use crate::functions::encode::{encode, EncodeOptions};
use crate::functions::get_randomness::{get_randomness, GetRandomnessOptions};

pub struct RowIDOptions<'a> {
    pub char_list: &'a str,
    pub randomness_length: usize,
}

pub fn rowid(opts: RowIDOptions) -> String {
    encode(EncodeOptions {
        char_list: opts.char_list,
        system_time: SystemTime::now(),
    })
    .unwrap()
        + &get_randomness(GetRandomnessOptions {
            char_list: opts.char_list,
            randomness_length: opts.randomness_length,
        })
}
