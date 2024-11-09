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

mod common;
mod functions;

/// Base module that contains the basic functions and structs.
pub mod base;

/// Config module, for customization.
pub mod with_config;

/// Time module.
pub mod time;
