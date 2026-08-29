/*
    Appellation: bumper <library>
    Created At: 2026.08.29:11:29:55
    Contrib: @FL03
*/
//! Welcome to `bumper`, a semver parser enabling programmatic increments and routine versioning
//! strategies.
//!
#![allow(
    async_fn_in_trait,
    non_snake_case,
    clippy::needless_doctest_main,
    clippy::non_canonical_clone_impl,
    clippy::non_canonical_partial_ord_impl
)]
#![cfg_attr(not(feature = "std"), no_std)]
// compile-time checks
#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! { "The `bumper` crate requires that either the `std` or `alloc` feature be enabled to compile." }
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;

// modules
pub mod error;
#[cfg(feature = "parse")]
pub mod parse;

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;
    mod semver;
    mod version;
    mod prelude {
        pub use super::semver::*;
        pub use super::version::*;
    }
}

// re-export
#[cfg(feature = "parse")]
pub use self::parse::VersionParser;
#[doc(inline)]
pub use self::{error::*, types::*};
// prelude
pub mod prelude {
    #[cfg(feature = "parse")]
    pub use crate::parse::*;
    pub use crate::types::*;
}
