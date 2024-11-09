use std::{io, time::SystemTime};

use crate::{
    common::{
        configs::{CHAR_LIST, RANDOMNESS_LENGTH},
        errors::RowIDError,
    },
    functions::{
        decode::{DecodeOptions, _decode},
        encode::{EncodeOptions, _encode},
        generate::{GenerateOptions, GenerateResult, _generate},
        get_randomness::{GetRandomnessOptions, _get_randomness},
        rowid::{RowIDOptions, _rowid},
        verify::{VerifyOptions, VerifyResult, _verify},
    },
};

/// This struct contains the state of the `RowIDWithConfig` struct.
#[derive(Debug, Clone)]
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
/// use rowid::with_config::{RowIDWithConfig, RowIDWithConfigResult};
///
/// let rwc: RowIDWithConfigResult =
///     RowIDWithConfig::new().done().unwrap();
/// ```
#[derive(Debug, Clone)]
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
    /// use rowid::with_config::{RowIDWithConfig, RowIDWithConfigResult};
    ///
    /// let rwc: RowIDWithConfigResult =
    ///     RowIDWithConfig::new().done().unwrap();
    /// let id: String = rwc.rowid();
    /// ```
    pub fn rowid(&self) -> String {
        _rowid(RowIDOptions {
            char_list: &self.state.char_list,
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
    /// use rowid::with_config::{RowIDWithConfig, RowIDWithConfigResult};
    ///
    /// let rwc: RowIDWithConfigResult =
    ///     RowIDWithConfig::new().done().unwrap();
    /// let encoded: String = rwc.encode(SystemTime::now()).unwrap();
    /// ```
    pub fn encode<T: Into<SystemTime>>(
        &self,
        system_time: T,
    ) -> io::Result<String> {
        _encode(EncodeOptions {
            char_list: &self.state.char_list,
            system_time: system_time.into(),
        })
    }

    /// This function decodes the ID into a timestamp in milliseconds.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use std::time::SystemTime;
    /// use rowid::with_config::{RowIDWithConfig, RowIDWithConfigResult};
    ///
    /// let rwc: RowIDWithConfigResult =
    ///     RowIDWithConfig::new().done().unwrap();
    /// let decoded: SystemTime = rwc.decode("ABC123").unwrap();
    /// ```
    pub fn decode<S: AsRef<str>>(
        &self,
        encoded: S,
    ) -> io::Result<SystemTime> {
        _decode(DecodeOptions {
            char_list: &self.state.char_list,
            encoded: encoded.as_ref(),
        })
    }

    /// This function generates an ID based on the input.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use std::time::SystemTime;
    /// use rowid::{
    ///     base::GenerateResult,
    ///     with_config::{RowIDWithConfig, RowIDWithConfigResult}
    /// };
    ///
    /// let rwc: RowIDWithConfigResult =
    ///     RowIDWithConfig::new().done().unwrap();
    /// let now: SystemTime = SystemTime::now();
    /// let result: GenerateResult = rwc.generate(now, Some(22));
    /// ```
    pub fn generate<T: Into<SystemTime>>(
        &self,
        system_time: T,
        randomness_length: Option<usize>,
    ) -> GenerateResult {
        _generate(GenerateOptions {
            char_list: &self.state.char_list,
            system_time: system_time.into(),
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
    /// use rowid::{
    ///     base::VerifyResult,
    ///     with_config::{RowIDWithConfig, RowIDWithConfigResult}
    /// };
    ///
    /// let rwc: RowIDWithConfigResult =
    ///     RowIDWithConfig::new().done().unwrap();
    /// let result: VerifyResult = rwc.verify("ABC123");
    /// ```
    pub fn verify<S: AsRef<str>>(
        &self,
        encoded: S,
    ) -> VerifyResult {
        _verify(VerifyOptions {
            char_list: &self.state.char_list,
            encoded: encoded.as_ref(),
        })
    }

    /// This function generates randomness.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use rowid::with_config::{RowIDWithConfig, RowIDWithConfigResult};
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
            char_list: &self.state.char_list,
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
/// use rowid::with_config::{RowIDWithConfig, RowIDWithConfigResult};
///
/// let rwc: RowIDWithConfigResult =
///     RowIDWithConfig::new().done().unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct RowIDWithConfig {
    state: RowIDWithConfigState,
}

impl RowIDWithConfig {
    /// Creates a new `RowIDWithConfig` with default values.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use rowid::with_config::{RowIDWithConfig, RowIDWithConfigResult};
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
    /// The length of it must be greater or equal to `28`.
    ///
    /// default: `0123456789ABCDEFGHJKMNPQRSTVWXYZ`
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use rowid::with_config::{RowIDWithConfig, RowIDWithConfigResult};
    ///
    /// let rwc: RowIDWithConfigResult = RowIDWithConfig::new()
    ///     .char_list("0123456789ABCDEFGHJKMNPQRSTVWXYZ")
    ///     .done()
    ///     .unwrap();
    ///
    /// let id: String = rwc.rowid();
    /// ```
    pub fn char_list<S: Into<String>>(
        mut self,
        list: S,
    ) -> Self {
        self.state.char_list = list.into();
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
    /// use rowid::with_config::{RowIDWithConfig, RowIDWithConfigResult};
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
    /// use rowid::with_config::{RowIDWithConfig, RowIDWithConfigResult};
    ///
    /// let rwc: RowIDWithConfigResult =
    ///     RowIDWithConfig::new().done().unwrap();
    /// let id: String = rwc.rowid();
    /// ```
    pub fn done(self) -> io::Result<RowIDWithConfigResult> {
        if self.state.char_list.len() < 28 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                RowIDError::CharListLength.as_str(),
            ));
        }

        Ok(RowIDWithConfigResult {
            state: RowIDWithConfigState {
                char_list: self.state.char_list,
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
