// repite::error
//
//! Error types.
//!
//

use core::result;
use sixbit::EncodeError;

/// `repite` result type.
pub type Result<N> = result::Result<N, Error>;

/// `repite` error type.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// An error involving the encoding of a rate's name.
    RateName(EncodeError),
}

/// allows converting into `Error` from other error types.
mod core_impls {
    use super::{EncodeError, Error};

    impl From<EncodeError> for Error {
        fn from(err: EncodeError) -> Self {
            Error::RateName(err)
        }
    }
}

/// impl `Display` & `Error` on all types.
#[cfg(feature = "std")]
mod std_impls {
    use super::Error;
    use std::{
        error::Error as StdError,
        fmt::{self, Debug},
    };

    impl StdError for Error {}
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Error::RateName(r) => Debug::fmt(r, f),
            }
        }
    }
}
