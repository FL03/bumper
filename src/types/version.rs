/*
    Appellation: version <module>
    Created At: 2026.08.29:11:35:04
    Contrib: @FL03
*/
use crate::error::VersionParseError;
use crate::types::{SemVer, SuffixedSemVer};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Version<T = u16> {
    SemVer(SemVer<T>),
    Suffixed(SuffixedSemVer<T>),
}

impl<T> Version<T> {
    pub fn semver(version: SemVer<T>) -> Self {
        Self::SemVer(version)
    }

    pub fn suffixed(version: SuffixedSemVer<T>) -> Self {
        Self::Suffixed(version)
    }

    pub fn as_semver(&self) -> &SemVer<T> {
        match self {
            Self::SemVer(version) => version,
            Self::Suffixed(version) => &version.version,
        }
    }

    pub fn is_suffixed(&self) -> bool {
        matches!(self, Self::Suffixed(_))
    }
}

impl<T> core::fmt::Display for Version<T>
where
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SemVer(version) => version.fmt(f),
            Self::Suffixed(version) => version.fmt(f),
        }
    }
}

impl<T> core::str::FromStr for Version<T>
where
    T: core::str::FromStr,
{
    type Err = VersionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        crate::parse::parse_version::<T>(input)
    }
}
