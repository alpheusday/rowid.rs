use std::{io, time::UNIX_EPOCH};

use crate::{
    common::{configs::TIMESTAMP_LENGTH, errors::RowIDError},
    time::timestamp_to_system_time,
};

pub struct EncodeOptions<CharList: AsRef<str>> {
    pub char_list: CharList,
    pub timestamp: usize,
}

fn __encode<CharList: AsRef<str>>(opts: EncodeOptions<CharList>) -> String {
    let char_list: Vec<char> = opts.char_list.as_ref().chars().collect();
    let char_list_length: usize = char_list.len();

    let mut index: usize = TIMESTAMP_LENGTH;
    let mut encoded: [char; TIMESTAMP_LENGTH] = ['\0'; TIMESTAMP_LENGTH];
    let mut remaining: usize = opts.timestamp;

    while index > 0 {
        index -= 1;
        encoded[index] = char_list[remaining % char_list_length];
        remaining /= char_list_length;
    }

    encoded.iter().collect()
}

pub fn encode_unsafe<CharList: AsRef<str>>(
    opts: EncodeOptions<CharList>
) -> String {
    __encode(opts)
}

pub fn _encode<CharList: AsRef<str>>(
    opts: EncodeOptions<CharList>
) -> io::Result<String> {
    if timestamp_to_system_time(opts.timestamp) < UNIX_EPOCH {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            RowIDError::InvalidEncoded.as_str(),
        ));
    }

    Ok(__encode(opts))
}
