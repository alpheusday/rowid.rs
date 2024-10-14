use std::{collections::HashMap, io, time::SystemTime};

use crate::internal::{
    common::{configs::TIMESTAMP_LENGTH, errors::RowIDError},
    utils::time::timestamp_to_system_time,
};

pub struct DecodeOptions<'a> {
    pub char_list: &'a str,
    pub encoded: &'a str,
}

pub fn decode(opts: DecodeOptions) -> io::Result<SystemTime> {
    if opts.encoded.len() < TIMESTAMP_LENGTH {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            RowIDError::EncodedLength.as_str(),
        ));
    }

    let char_list: Vec<char> = opts.char_list.chars().collect();
    let char_list_length: usize = char_list.len();
    let char_index_map: HashMap<char, usize> =
        char_list.iter().enumerate().map(|(i, &c)| (c, i)).collect();

    let mut encoded_chars: [char; TIMESTAMP_LENGTH] = ['\0'; TIMESTAMP_LENGTH];

    for (i, c) in
        opts.encoded[..TIMESTAMP_LENGTH].to_uppercase().chars().enumerate()
    {
        encoded_chars[i] = c;
    }

    if !encoded_chars.iter().all(|c| char_index_map.contains_key(c)) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            RowIDError::InvalidEncoded.as_str(),
        ));
    }

    let mut timestamp: usize = 0;

    for c in encoded_chars {
        if let Some(index) = char_index_map.get(&c) {
            timestamp = timestamp * char_list_length + index;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                RowIDError::InvalidEncoded.as_str(),
            ));
        }
    }

    Ok(timestamp_to_system_time(timestamp))
}
