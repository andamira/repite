// repite::lib
//
//! Manage loops and rates.
//

#![warn(clippy::all)]
#![allow(
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::needless_return,
    clippy::blanket_clippy_restriction_lints,
    clippy::pattern_type_mismatch
)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod error;

mod looper;
pub use looper::{Looper, LoopStatus};

mod rate;
pub use rate::Rate;

mod rate_stats;
pub use rate_stats::RateStats;

/// *(re-exported)*
pub use espera::{Duration, Instant};
