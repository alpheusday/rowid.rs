//! # RowID
//!
//! A time-based unique ID solution.
//!
//! ## Quick Start
//!
//! Create an ID with the following code:
//!
//! ```no_run
//! use rowid::base::rowid;
//!
//! let id: String = rowid();
//! ```
//!
//! Or start a customization with the following code:
//!
//! ```no_run
//! use rowid::with_config::{RowIDWithConfig, RowIDWithConfigResult};
//!
//! let rwc: RowIDWithConfigResult = RowIDWithConfig::new()
//!     .char_list("0123456789ABCDEFGHJKMNPQRSTVWXYZ")
//!     .randomness_length(22)
//!     .done()
//!     .unwrap();
//!
//! let id: String = rwc.rowid();
//! ```

mod internal;

/// Base module that contains the basic functions and structs.
pub mod base {
    pub use crate::internal::functions::generate::GenerateResult;
    pub use crate::internal::functions::verify::VerifyResult;

    pub use crate::internal::base::{
        decode, encode, generate, get_randomness, rowid, verify,
    };

    pub use crate::internal::common::errors::RowIDError;
}

/// Config module, for customization.
pub mod with_config {
    pub use crate::internal::base::with_config::{
        RowIDWithConfig, RowIDWithConfigResult, RowIDWithConfigState,
    };
}

/// Time module.
pub mod time {
    pub use crate::internal::utils::time::{
        system_time_to_timestamp, timestamp_to_system_time,
    };
}
