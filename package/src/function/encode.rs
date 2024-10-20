use std::{
    io,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    common::{configs::TIMESTAMP_LENGTH, errors::RowIDError},
    time::system_time_to_timestamp,
};

pub struct EncodeOptions<'a> {
    pub char_list: &'a str,
    pub system_time: SystemTime,
}

fn _encode_(opts: EncodeOptions) -> String {
    let char_list: Vec<char> = opts.char_list.chars().collect();
    let char_list_length: usize = char_list.len();

    let mut index: usize = TIMESTAMP_LENGTH;
    let mut encoded: [char; TIMESTAMP_LENGTH] = ['\0'; TIMESTAMP_LENGTH];
    let mut remaining: usize = system_time_to_timestamp(opts.system_time);

    while index > 0 {
        index -= 1;
        encoded[index] = char_list[remaining % char_list_length];
        remaining /= char_list_length;
    }

    encoded.iter().collect()
}

pub fn encode_unsafe(opts: EncodeOptions) -> String {
    _encode_(opts)
}

pub fn _encode(opts: EncodeOptions) -> io::Result<String> {
    if opts.system_time < UNIX_EPOCH {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            RowIDError::InvalidEncoded.as_str(),
        ));
    }

    Ok(_encode_(opts))
}
