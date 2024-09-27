//! # RowID
//!
//! A time-based unique ID solution.
//!
//! ## Quick Start
//!
//! You may create a RowID with the following code:
//!
//! ```no_run
//! use rowid::rowid;
//!
//! let id: String = rowid();
//! ```
//!
//! Or customize the RowID with the following code:
//!
//! ```no_run
//! use rowid::{RowIDWithConfig, RowIDWithConfigResult};
//!
//! let rwc: RowIDWithConfigResult = RowIDWithConfig::new()
//!     .char_list("0123456789ABCDEFGHJKMNPQRSTVWXYZ".to_string())
//!     .randomness_length(22)
//!     .done()
//!     .unwrap();
//!
//! let id: String = rwc.rowid();
//! ```

mod base;
mod common;
mod functions;
mod utils;

// basics

pub use crate::functions::generate::GenerateResult;
pub use crate::functions::verify::VerifyResult;

pub use crate::base::rowid::{
    decode, encode, generate, get_randomness, rowid, verify,
};

pub use crate::base::rowid_with_config::{
    RowIDWithConfig, RowIDWithConfigResult, RowIDWithConfigState,
};

pub use crate::utils::system_time::{
    system_time_to_timestamp, timestamp_to_system_time,
};

// errors

/// This module contains all the error messages.
pub mod errors {
    pub use crate::common::errors::{
        DECODE_ECD_INVALID_ERR, DECODE_ECD_LENGTH_ERR, ENCODE_ST_INVALID_ERR,
        RWC_DONE_CHAR_LIST_LENGTH_ERR,
    };
}
