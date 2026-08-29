/*
    Appellation: error <module>
    Created At: 2026.08.29:11:24:58
    Contrib: @FL03
*/

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum SemVerParseError {
    #[error("invalid semantic version; expected MAJOR.MINOR.PATCH")]
    InvalidFormat,
    #[error("invalid numeric version component")]
    InvalidComponent,
    #[error("version component does not fit target integer type")]
    ComponentOverflow,
    #[error("unexpected trailing version input")]
    TrailingInput,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum VersionParseError {
    #[error(transparent)]
    SemVer(#[from] SemVerParseError),
    #[error("invalid suffix")]
    InvalidSuffix,
    #[error("invalid format")]
    InvalidFormat,
}
