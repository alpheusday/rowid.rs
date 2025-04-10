use std::{io, time::SystemTime};

use rowid::{
    base::{GenerateResult, RowIDError, VerifyResult},
    time::system_time_to_timestamp,
    with_config::{RowIDWithConfig, RowIDWithConfigResult},
};

#[test]
fn test_rowid_with_config_char_list_length_error() {
    let err: io::Error = match RowIDWithConfig::new().char_list("ABC").done() {
        | Ok(_) => return assert!(false),
        | Err(e) => e,
    };

    assert!(err.kind() == io::ErrorKind::InvalidInput);
    assert!(err.to_string() == RowIDError::CharListLength.as_str());
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
        rwc.decode(&rwc.encode(current).unwrap()).unwrap();

    assert!(
        system_time_to_timestamp(decoded) == system_time_to_timestamp(current)
    );
}

#[test]
fn test_rowid_with_config_encode_decode() {
    let rwc: RowIDWithConfigResult = RowIDWithConfig::new()
        .randomness_length(6)
        .char_list("0123456789acdefghjkmnpqrtvwxy")
        .done()
        .unwrap();

    let now: SystemTime = SystemTime::now();

    let encoded: String = rwc.encode(now).unwrap();

    let decoded: SystemTime = rwc.decode(encoded).unwrap();

    assert!(system_time_to_timestamp(decoded) == system_time_to_timestamp(now));
}

#[test]
fn test_rowid_with_config_generate() {
    let rwc: RowIDWithConfigResult =
        RowIDWithConfig::new().randomness_length(6).done().unwrap();

    let current: SystemTime = SystemTime::now();
    let generated: GenerateResult = rwc.generate(current, None);
    let id: String = generated.result.unwrap();

    assert!(generated.success == true);
    assert!(id.len() == 16);
    assert!(
        system_time_to_timestamp(rwc.decode(&id).unwrap())
            == system_time_to_timestamp(current)
    );
}

#[test]
fn test_rowid_with_config_verify() {
    let rwc: RowIDWithConfigResult =
        RowIDWithConfig::new().randomness_length(6).done().unwrap();

    let current: SystemTime = SystemTime::now();
    let generated: GenerateResult = rwc.generate(current, None);
    let verified: VerifyResult = rwc.verify(&generated.result.unwrap());

    assert!(verified.success == true);
    assert!(match verified.result {
        | Some(r) =>
            system_time_to_timestamp(r) == system_time_to_timestamp(current),
        | None => false,
    });
    assert!(verified.natural == Some(true));
}
