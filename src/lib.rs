/*
    Appellation: bumper <library>
    Created At: 2026.08.29:11:29:55
    Contrib: @FL03
*/
// external crates
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
compile_error! { "Either the `std` or `alloc` feature must be enabled." }
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

pub mod utils {
    #[doc(inline)]
    pub(crate) use self::prelude::*;

    mod parse;

    mod prelude {
        pub(crate) use super::parse::*;
    }
}
// re-export
#[doc(inline)]
pub use self::{error::*, types::*};
// prelude
pub mod prelude {
    // #[cfg(feature = "parse")]
    // pub use crate::parse::*;
}
