use std::time::SystemTime;

use crate::{
    functions::{
        encode::{EncodeOptions, encode_unsafe},
        get_randomness::{_get_randomness, GetRandomnessOptions},
    },
    time::system_time_to_timestamp,
};

pub struct RowIDOptions<CharList: AsRef<str>> {
    pub char_list: CharList,
    pub randomness_length: usize,
}

pub fn _rowid<CharList: AsRef<str>>(opts: RowIDOptions<CharList>) -> String {
    let char_list: &str = opts.char_list.as_ref();

    encode_unsafe(EncodeOptions {
        char_list,
        timestamp: system_time_to_timestamp(SystemTime::now()),
    }) + &_get_randomness(GetRandomnessOptions {
        char_list,
        randomness_length: opts.randomness_length,
    })
}
