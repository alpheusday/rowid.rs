use std::io;
use std::time::SystemTime;

use crate::common::configs::{CHAR_LIST, RANDOMNESS_LENGTH};
use crate::common::errors::RWC_DONE_CHAR_LIST_LENGTH_ERR;
use crate::functions::decode::{decode as _decode, DecodeOptions};
use crate::functions::encode::{encode as _encode, EncodeOptions};
use crate::functions::generate::{
    generate as _generate, GenerateOptions, GenerateResult,
};
use crate::functions::get_randomness::{
    get_randomness as _get_randomness, GetRandomnessOptions,
};
use crate::functions::rowid::{rowid as _rowid, RowIDOptions};
use crate::functions::verify::{
    verify as _verify, VerifyOptions, VerifyResult,
};

/// This struct contains the state of the `RowIDWithConfig` struct.
pub struct RowIDWithConfigState {
    /// The list of characters used in the current function.
    pub char_list: String,
    /// The length of randomness used in the current function.
    pub randomness_length: usize,
}

/// This struct contains different modified functions
/// from the `RowIDWithConfig` struct.
///
/// ## Example
///
/// ```no_run
/// use rowid::{RowIDWithConfig, RowIDWithConfigResult};
///
/// let rwc: RowIDWithConfigResult =
///     RowIDWithConfig::new().done().unwrap();
/// ```
pub struct RowIDWithConfigResult {
    /// Represents function's customization configurations.
    pub state: RowIDWithConfigState,
}

impl RowIDWithConfigResult {
    /// This function generates a unique ID
    /// that is almost impossible to duplicate.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use rowid::{RowIDWithConfig, RowIDWithConfigResult};
    ///
    /// let rwc: RowIDWithConfigResult =
    ///     RowIDWithConfig::new().done().unwrap();
    /// let id: String = rwc.rowid();
    /// ```
    pub fn rowid(&self) -> String {
        _rowid(RowIDOptions {
            char_list: self.state.char_list.clone(),
            randomness_length: self.state.randomness_length,
        })
    }

    /// This function encodes the timestamp in milliseconds
    /// into an ID without randomness.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use std::time::SystemTime;
    /// use rowid::{RowIDWithConfig, RowIDWithConfigResult};
    ///
    /// let rwc: RowIDWithConfigResult =
    ///     RowIDWithConfig::new().done().unwrap();
    /// let encoded: String = rwc.encode(SystemTime::now()).unwrap();
    /// ```
    pub fn encode(
        &self,
        system_time: SystemTime,
    ) -> io::Result<String> {
        _encode(EncodeOptions {
            char_list: self.state.char_list.clone(),
            system_time,
        })
    }

    /// This function decodes the ID into a timestamp in milliseconds.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use std::time::SystemTime;
    /// use rowid::{RowIDWithConfig, RowIDWithConfigResult};
    ///
    /// let rwc: RowIDWithConfigResult =
    ///     RowIDWithConfig::new().done().unwrap();
    /// let id: String = "ABC123".to_string();
    /// let decoded: SystemTime = rwc.decode(id).unwrap();
    /// ```
    pub fn decode(
        &self,
        encoded: String,
    ) -> io::Result<SystemTime> {
        _decode(DecodeOptions {
            char_list: self.state.char_list.clone(),
            encoded,
        })
    }

    /// This function generates an ID based on the input.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use std::time::SystemTime;
    /// use rowid::{RowIDWithConfig, RowIDWithConfigResult, GenerateResult};
    ///
    /// let rwc: RowIDWithConfigResult =
    ///     RowIDWithConfig::new().done().unwrap();
    /// let now: SystemTime = SystemTime::now();
    /// let result: GenerateResult = rwc.generate(now, Some(22));
    /// ```
    pub fn generate(
        &self,
        system_time: SystemTime,
        randomness_length: Option<usize>,
    ) -> GenerateResult {
        _generate(GenerateOptions {
            char_list: self.state.char_list.clone(),
            system_time,
            randomness_length: match randomness_length {
                | Some(l) => l,
                | None => self.state.randomness_length,
            },
        })
    }

    /// This function verifies if the ID is valid and natural.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use rowid::{RowIDWithConfig, RowIDWithConfigResult, VerifyResult};
    ///
    /// let rwc: RowIDWithConfigResult =
    ///     RowIDWithConfig::new().done().unwrap();
    /// let id: String = "ABC123".to_string();
    /// let result: VerifyResult = rwc.verify(id);
    /// ```
    pub fn verify(
        &self,
        encoded: String,
    ) -> VerifyResult {
        _verify(VerifyOptions {
            char_list: self.state.char_list.clone(),
            encoded,
        })
    }

    /// This function generates randomness.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use rowid::{RowIDWithConfig, RowIDWithConfigResult};
    ///
    /// let rwc: RowIDWithConfigResult =
    ///     RowIDWithConfig::new().done().unwrap();
    /// let randomness: String = rwc.get_randomness(10);
    /// ```
    pub fn get_randomness(
        &self,
        randomness_length: usize,
    ) -> String {
        _get_randomness(GetRandomnessOptions {
            char_list: self.state.char_list.clone(),
            randomness_length,
        })
    }
}

/// This struct allows you to create a new instance
/// that contains different RowID functions
/// and add different configuration functions
/// to customize how RowID works.
///
/// ## Example
///
/// ```no_run
/// use rowid::{RowIDWithConfig, RowIDWithConfigResult};
///
/// let rwc: RowIDWithConfigResult =
///     RowIDWithConfig::new().done().unwrap();
/// ```
pub struct RowIDWithConfig {
    state: RowIDWithConfigState,
}

impl RowIDWithConfig {
    /// Creates a new `RowIDWithConfig` with default values.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use rowid::{RowIDWithConfig, RowIDWithConfigResult};
    ///
    /// let rwc: RowIDWithConfigResult =
    ///     RowIDWithConfig::new().done().unwrap();
    /// ```
    pub fn new() -> Self {
        Self {
            state: RowIDWithConfigState {
                char_list: CHAR_LIST.to_string(),
                randomness_length: RANDOMNESS_LENGTH,
            },
        }
    }

    /// The list of characters that can be used in the RowID,
    /// it must be longer or equal to `28`.
    ///
    /// default: `0123456789ABCDEFGHJKMNPQRSTVWXYZ`
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use rowid::{RowIDWithConfig, RowIDWithConfigResult};
    ///
    /// let rwc: RowIDWithConfigResult = RowIDWithConfig::new()
    ///     .char_list("0123456789ABCDEFGHJKMNPQRSTVWXYZ".to_string())
    ///     .done()
    ///     .unwrap();
    ///
    /// let id: String = rwc.rowid();
    /// ```
    pub fn char_list(
        mut self,
        list: String,
    ) -> Self {
        self.state.char_list = list;
        self
    }

    /// The default length of randomness in the RowID,
    /// it's recommended to be longer or equal to `6`
    /// to avoid collision in the same timestamp.
    ///
    /// default: `22`
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use rowid::{RowIDWithConfig, RowIDWithConfigResult};
    ///
    /// let rwc: RowIDWithConfigResult = RowIDWithConfig::new()
    ///     .randomness_length(22)
    ///     .done()
    ///     .unwrap();
    ///
    /// let id: String = rwc.rowid();
    /// ```
    pub fn randomness_length(
        mut self,
        length: usize,
    ) -> Self {
        self.state.randomness_length = length;
        self
    }

    /// This function ends the configuration of the `rowid_with_config` function,
    /// and returns different modified functions based on the parameters.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use rowid::{RowIDWithConfig, RowIDWithConfigResult};
    ///
    /// let rwc: RowIDWithConfigResult =
    ///     RowIDWithConfig::new().done().unwrap();
    /// let id: String = rwc.rowid();
    /// ```
    pub fn done(self) -> io::Result<RowIDWithConfigResult> {
        if self.state.char_list.len() < 28 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                RWC_DONE_CHAR_LIST_LENGTH_ERR,
            ));
        }

        Ok(RowIDWithConfigResult {
            state: RowIDWithConfigState {
                char_list: self.state.char_list.clone(),
                randomness_length: self.state.randomness_length,
            },
        })
    }
}

impl Default for RowIDWithConfig {
    fn default() -> Self {
        Self::new()
    }
}
