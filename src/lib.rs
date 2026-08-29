/*
    Appellation: bumper <library>
    Created At: 2026.08.29:11:29:55
    Contrib: @FL03
*/

#[cfg(feature = "alloc")]
extern crate alloc;

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
    pub use self::prelude::*;
    mod parse;

    mod prelude {
        pub use super::parse::*;
    }
}

#[doc(inline)]
pub use self::{error::*, types::*};

pub mod prelude {
    #[cfg(feature = "parse")]
    pub use crate::parse::*;
}
