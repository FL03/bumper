/*
    Appellation: error <module>
    Created At: 2026.08.29:11:24:58
    Contrib: @FL03
*/

/// a type alias for a [`Result`](core::result::Result) equipped to leverage the crate error type.
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("invalid numeric version component")]
    InvalidComponent,
    #[error("invalid semantic version; expected MAJOR.MINOR.PATCH with an optional suffix.")]
    InvalidFormat,
    #[error("version component does not fit target integer type")]
    ComponentOverflow,
    #[error("unexpected trailing version input")]
    TrailingInput,
    #[error("invalid suffix")]
    InvalidSuffix,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // custom
    #[error(transparent)]
    ParseError(#[from] ParseError),
    // external errors
    #[error(transparent)]
    IOError(std::io::Error),
}
