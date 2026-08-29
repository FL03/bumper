/*
    Appellation: semver <module>
    Created At: 2026.08.29:11:28:56
    Contrib: @FL03
*/
use crate::error::SemVerParseError;
use core::ops::{Add, Rem, Sub};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SemVer<T = u16> {
    pub(crate) major: T,
    pub(crate) minor: T,
    pub(crate) patch: T,
}

impl<T> SemVer<T> {
    /// Construct 0.0.0.
    ///
    /// IMPORTANT:
    /// The Zero bound exists ONLY on this method.
    pub fn new() -> Self
    where
        T: num_traits::Zero,
    {
        Self {
            major: T::zero(),
            minor: T::zero(),
            patch: T::zero(),
        }
    }

    pub const fn from_parts(major: T, minor: T, patch: T) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn major(&self) -> &T {
        &self.major
    }

    pub fn minor(&self) -> &T {
        &self.minor
    }

    pub fn patch(&self) -> &T {
        &self.patch
    }

    pub fn into_parts(self) -> (T, T, T) {
        (self.major, self.minor, self.patch)
    }
}

impl<T> SemVer<T>
where
    T: Clone
        + PartialEq
        + PartialOrd
        + num_traits::One
        + Add<Output = T>
        + Sub<Output = T>
        + Rem<Output = T>,
{
    pub fn successor(&self) -> Self {
        let one = T::one();

        let nine = one.clone()
            + one.clone()
            + one.clone()
            + one.clone()
            + one.clone()
            + one.clone()
            + one.clone()
            + one.clone()
            + one.clone();

        let zero = one.clone() - one.clone();

        if self.patch < nine {
            Self {
                major: self.major.clone(),
                minor: self.minor.clone(),
                patch: self.patch.clone() + one.clone(),
            }
        } else if self.minor < nine {
            Self {
                major: self.major.clone(),
                minor: self.minor.clone() + one.clone(),
                patch: zero,
            }
        } else {
            Self {
                major: self.major.clone() + one,
                minor: zero.clone(),
                patch: zero,
            }
        }
    }
}

impl<T> core::fmt::Display for SemVer<T>
where
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

//
// ============================================================================
// SuffixedSemVer
// ============================================================================
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SuffixedSemVer<T = u16> {
    pub(crate) version: SemVer<T>,
    pub(crate) separator: String,
    pub(crate) suffix: String,
}

impl<T> SuffixedSemVer<T> {
    pub fn new(
        version: SemVer<T>,
        separator: impl Into<String>,
        suffix: impl Into<String>,
    ) -> Self {
        Self {
            version,
            separator: separator.into(),
            suffix: suffix.into(),
        }
    }

    pub fn version(&self) -> &SemVer<T> {
        &self.version
    }

    pub fn separator(&self) -> &str {
        &self.separator
    }

    pub fn suffix(&self) -> &str {
        &self.suffix
    }

    pub fn into_parts(self) -> (SemVer<T>, String, String) {
        (self.version, self.separator, self.suffix)
    }
}

impl<T> core::fmt::Display for SuffixedSemVer<T>
where
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}{}", self.version, self.separator, self.suffix)
    }
}
impl<T> core::str::FromStr for SemVer<T>
where
    T: core::str::FromStr,
{
    type Err = SemVerParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (version, remaining) = crate::parse::parse_semver::<T>(input)?;

        if !remaining.is_empty() {
            return Err(SemVerParseError::TrailingInput);
        }

        Ok(version)
    }
}
