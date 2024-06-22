use std::time::SystemTime;

use crate::functions::encode::{encode, EncodeOptions};
use crate::functions::get_randomness::{get_randomness, GetRandomnessOptions};

pub struct RowIDOptions {
    pub char_list: String,
    pub randomness_length: usize,
}

pub fn rowid(opts: RowIDOptions) -> String {
    encode(EncodeOptions {
        char_list: opts.char_list.clone(),
        system_time: SystemTime::now(),
    })
    .unwrap()
        + &get_randomness(GetRandomnessOptions {
            char_list: opts.char_list.clone(),
            randomness_length: opts.randomness_length,
        })
}
