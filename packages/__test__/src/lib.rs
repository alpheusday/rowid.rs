#[cfg(test)]
mod tests {
    use std::{io, time::SystemTime};

    use rowid::{
        decode, encode, generate, get_randomness, rowid,
        system_time_to_timestamp, timestamp_to_system_time, verify,
        GenerateResult, RowIDWithConfig, RowIDWithConfigResult, VerifyResult,
    };

    use rowid::errors::{
        DECODE_ECD_INVALID_ERR, DECODE_ECD_LENGTH_ERR,
        RWC_DONE_CHAR_LIST_LENGTH_ERR,
    };

    // system_time_to_timestamp

    #[test]
    fn test_system_time_to_timestamp() {
        let timestamp: usize = system_time_to_timestamp(SystemTime::UNIX_EPOCH);
        assert!(timestamp == 0);
    }

    // timestamp_to_system_time

    #[test]
    fn test_timestamp_to_system_time() {
        let system_time: SystemTime = timestamp_to_system_time(0);
        assert!(system_time == SystemTime::UNIX_EPOCH);
    }

    // timestamp_to_system_time + system_time_to_timestamp

    #[test]
    fn test_timestamp_system_time() {
        let ts: usize = 1_000_000_000;
        let system_time: SystemTime = timestamp_to_system_time(ts);
        let timestamp: usize = system_time_to_timestamp(system_time);
        assert!(ts == timestamp);
    }

    // get_randomness

    #[test]
    fn test_get_randomness_0() {
        let rds: String = get_randomness(0);
        assert!(rds.len() == 0);
    }

    #[test]
    fn test_get_randomness_6() {
        let rds: String = get_randomness(6);
        assert!(rds.len() == 6);
    }

    #[test]
    fn test_get_randomness_10() {
        let rds: String = get_randomness(10);
        assert!(rds.len() == 10);
    }

    #[test]
    fn test_get_randomness_20() {
        let rds: String = get_randomness(20);
        assert!(rds.len() == 20);
    }

    // encode

    #[test]
    fn test_encode_now() {
        let encoded: String = encode(SystemTime::now()).unwrap();
        assert!(encoded.len() == 10);
    }

    #[test]
    fn test_encode_0() {
        let encoded: String = encode(timestamp_to_system_time(0)).unwrap();
        assert!(encoded == "0000000000");
    }

    // rowid

    #[test]
    fn test_rowid() {
        let id: String = rowid();
        assert!(id.len() == 32);
    }

    // decode

    #[test]
    fn test_decode() {
        let current: SystemTime = SystemTime::now();
        let decoded: SystemTime = decode(encode(current).unwrap()).unwrap();
        assert!(
            system_time_to_timestamp(decoded)
                == system_time_to_timestamp(current)
        );
    }

    #[test]
    fn test_decode_length_error() {
        let result: io::Error = match decode("ABC123".to_string()) {
            | Ok(_) => return assert!(false),
            | Err(e) => e,
        };

        assert!(result.kind() == io::ErrorKind::InvalidInput);
        assert!(result.to_string() == DECODE_ECD_LENGTH_ERR);
    }

    #[test]
    fn test_decode_invalid_input_error() {
        let result: io::Error = match decode("ab^!@#$agastgyaSER".to_string()) {
            | Ok(_) => return assert!(false),
            | Err(e) => e,
        };

        assert!(result.kind() == io::ErrorKind::InvalidInput);
        assert!(result.to_string() == DECODE_ECD_INVALID_ERR);
    }

    // generate

    #[test]
    fn test_generate() {
        let current: SystemTime = SystemTime::now();
        let generated: GenerateResult = generate(current, Some(6));
        let id: String = generated.result.unwrap();
        assert!(generated.success == true);
        assert!(
            system_time_to_timestamp(decode(id.clone()).unwrap())
                == system_time_to_timestamp(current)
        );
        assert!(id.len() == 16);
    }

    // verify

    #[test]
    fn test_verify() {
        let current: SystemTime = SystemTime::now();
        let id: String = generate(current, Some(6)).result.unwrap();
        let verified: VerifyResult = verify(id);
        assert!(verified.success == true);
        assert!(match verified.result {
            | Some(r) =>
                system_time_to_timestamp(r) == system_time_to_timestamp(current),
            | None => false,
        });
        assert!(verified.natural == Some(true));
    }

    #[test]
    fn test_verify_length_error() {
        let verified: VerifyResult = verify("ABC123".to_string());

        assert!(verified.success == false);

        let error: io::Error = match verified.error {
            | None => return assert!(false),
            | Some(e) => e,
        };

        assert!(error.kind() == io::ErrorKind::InvalidInput);
        assert!(error.to_string() == DECODE_ECD_LENGTH_ERR);
    }

    #[test]
    fn test_verify_invalid_input_error() {
        let verified: VerifyResult = verify("ab^!@#$agastgyaSER".to_string());

        assert!(verified.success == false);

        let error: io::Error = match verified.error {
            | None => return assert!(false),
            | Some(e) => e,
        };

        assert!(error.kind() == io::ErrorKind::InvalidInput);
        assert!(error.to_string() == DECODE_ECD_INVALID_ERR);
    }

    // rowid_with_config

    #[test]
    fn test_rowid_with_config_char_list_length_error() {
        let err: io::Error =
            match RowIDWithConfig::new().char_list("ABC".to_string()).done() {
                | Ok(_) => return assert!(false),
                | Err(e) => e,
            };

        assert!(err.kind() == io::ErrorKind::InvalidInput);
        assert!(err.to_string() == RWC_DONE_CHAR_LIST_LENGTH_ERR);
    }

    #[test]
    fn test_rowid_with_config_encode() {
        let rwc: RowIDWithConfigResult =
            RowIDWithConfig::new().randomness_length(6).done().unwrap();

        let encoded: String = rwc.encode(SystemTime::now()).unwrap();

        assert!(encoded.len() == 10);
    }

    #[test]
    fn test_rowid_with_config_rowid() {
        let rwc: RowIDWithConfigResult =
            RowIDWithConfig::new().randomness_length(6).done().unwrap();

        let id: String = rwc.rowid();

        assert!(id.len() == (10 + 6));
    }

    #[test]
    fn test_rowid_with_config_decode() {
        let rwc: RowIDWithConfigResult =
            RowIDWithConfig::new().randomness_length(6).done().unwrap();

        let current: SystemTime = SystemTime::now();
        let decoded: SystemTime =
            rwc.decode(rwc.encode(current).unwrap()).unwrap();

        assert!(
            system_time_to_timestamp(decoded)
                == system_time_to_timestamp(current)
        );
    }

    #[test]
    fn test_rowid_with_config_generate() {
        let rwc: RowIDWithConfigResult =
            RowIDWithConfig::new().randomness_length(6).done().unwrap();

        let current: SystemTime = SystemTime::now();
        let generated: GenerateResult = rwc.generate(current, None);
        let id: String = generated.result.unwrap();

        assert!(generated.success == true);
        assert!(id.clone().len() == 16);
        assert!(
            system_time_to_timestamp(rwc.decode(id.clone()).unwrap())
                == system_time_to_timestamp(current)
        );
    }

    #[test]
    fn test_rowid_with_config_verify() {
        let rwc: RowIDWithConfigResult =
            RowIDWithConfig::new().randomness_length(6).done().unwrap();

        let current: SystemTime = SystemTime::now();
        let generated: GenerateResult = rwc.generate(current, None);
        let verified: VerifyResult = rwc.verify(generated.result.unwrap());

        assert!(verified.success == true);
        assert!(match verified.result {
            | Some(r) =>
                system_time_to_timestamp(r) == system_time_to_timestamp(current),
            | None => false,
        });
        assert!(verified.natural == Some(true));
    }
}
