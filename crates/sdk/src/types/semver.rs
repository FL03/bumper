/*
    Appellation: semver <module>
    Created At: 2026.08.29:11:28:56
    Contrib: @FL03
*/
use crate::error::SemVerParseError;
use core::ops::{Add, Rem, Sub};

fn parse_component<T>(input: &str) -> Result<(T, &str), SemVerParseError>
where
    T: core::str::FromStr,
{
    if input.is_empty() {
        return Err(SemVerParseError::InvalidFormat);
    }

    let bytes = input.as_bytes();

    // ------------------------------------------------------------------------
    // First character
    // ------------------------------------------------------------------------

    if !bytes[0].is_ascii_digit() {
        return Err(SemVerParseError::InvalidComponent);
    }

    // Leading zero is only legal when the component is exactly "0".
    if bytes[0] == b'0' {
        if bytes.get(1).is_some_and(u8::is_ascii_digit) {
            return Err(SemVerParseError::InvalidComponent);
        }

        let value = "0"
            .parse::<T>()
            .map_err(|_| SemVerParseError::ComponentOverflow)?;

        return Ok((value, &input[1..]));
    }

    // ------------------------------------------------------------------------
    // Consume the complete decimal component.
    // ------------------------------------------------------------------------

    let mut end = 1;

    while end < bytes.len() && bytes[end].is_ascii_digit() {
        end += 1;
    }

    let component = &input[..end];

    let value = component
        .parse::<T>()
        .map_err(|_| SemVerParseError::ComponentOverflow)?;

    Ok((value, &input[end..]))
}

pub(crate) fn parse_semver<T>(input: &str) -> Result<(SemVer<T>, &str), SemVerParseError>
where
    T: core::str::FromStr,
{
    let (major, input) = parse_component::<T>(input)?;

    let input = input
        .strip_prefix('.')
        .ok_or(SemVerParseError::InvalidFormat)?;

    let (minor, input) = parse_component::<T>(input)?;

    let input = input
        .strip_prefix('.')
        .ok_or(SemVerParseError::InvalidFormat)?;

    let (patch, input) = parse_component::<T>(input)?;

    Ok((
        SemVer {
            major,
            minor,
            patch,
        },
        input,
    ))
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct SemVer<T = u16> {
    pub(crate) major: T,
    pub(crate) minor: T,
    pub(crate) patch: T,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct SuffixedSemVer<T = u16> {
    pub(crate) version: SemVer<T>,
    pub(crate) separator: String,
    pub(crate) suffix: String,
}

/* -------- impls::<SemVer>:: -------- */

impl<T> SemVer<T> {
    /// Construct 0.0.0.
    ///
    pub fn new() -> Self
    where
        T: Default,
    {
        Self {
            major: T::default(),
            minor: T::default(),
            patch: T::default(),
        }
    }

    pub fn zero() -> Self
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

impl<T> Default for SemVer<T>
where
    T: num_traits::Zero,
{
    fn default() -> Self {
        Self::zero()
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

/* -------- SuffixedSemVer -------- */

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
        let (version, remaining) = parse_semver::<T>(input)?;

        if !remaining.is_empty() {
            return Err(SemVerParseError::TrailingInput);
        }

        Ok(version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_semver() {
        let version: SemVer = "6.5.9".parse().unwrap();

        assert_eq!(*version.major(), 6);
        assert_eq!(*version.minor(), 5);
        assert_eq!(*version.patch(), 9);
    }

    #[test]
    fn formats_basic_semver() {
        let version: SemVer = "6.5.9".parse().unwrap();

        assert_eq!(version.to_string(), "6.5.9");
    }

    #[test]
    fn rejects_v_prefix() {
        assert!("v6.5.9".parse::<SemVer>().is_err());
    }

    #[test]
    fn rejects_missing_patch() {
        assert!("6.5".parse::<SemVer>().is_err());
    }

    #[test]
    fn rejects_missing_minor() {
        assert!("6.9".parse::<SemVer>().is_err());
    }

    #[test]
    fn rejects_leading_zero_major() {
        assert!("06.5.9".parse::<SemVer>().is_err());
    }

    #[test]
    fn rejects_leading_zero_minor() {
        assert!("6.05.9".parse::<SemVer>().is_err());
    }

    #[test]
    fn rejects_leading_zero_patch() {
        assert!("6.5.09".parse::<SemVer>().is_err());
    }

    #[test]
    fn rejects_invalid_middle_component() {
        assert!("6.x.9".parse::<SemVer>().is_err());
    }

    #[test]
    fn accepts_zero_components() {
        let version: SemVer = "0.0.0".parse().unwrap();

        assert_eq!(version.to_string(), "0.0.0");
    }

    #[test]
    fn accepts_single_digit_components() {
        let version: SemVer = "6.5.9".parse().unwrap();

        assert_eq!(*version.major(), 6);
        assert_eq!(*version.minor(), 5);
        assert_eq!(*version.patch(), 9);
    }

    #[test]
    fn accepts_large_components() {
        let version: SemVer<u64> = "65535.65535.65535".parse().unwrap();

        assert_eq!(*version.major(), 65535);
        assert_eq!(*version.minor(), 65535);
        assert_eq!(*version.patch(), 65535);
    }

    #[test]
    fn detects_integer_overflow() {
        let result = "65536.5.9".parse::<SemVer<u16>>();

        assert!(matches!(result, Err(SemVerParseError::ComponentOverflow)));
    }

    #[test]
    fn detects_minor_integer_overflow() {
        let result = "1.65536.9".parse::<SemVer<u16>>();

        assert!(matches!(result, Err(SemVerParseError::ComponentOverflow)));
    }

    #[test]
    fn detects_patch_integer_overflow() {
        let result = "1.2.65536".parse::<SemVer<u16>>();

        assert!(matches!(result, Err(SemVerParseError::ComponentOverflow)));
    }

    #[test]
    fn new_is_zero() {
        let version = SemVer::<u16>::new();

        assert_eq!(version.to_string(), "0.0.0");
    }

    #[test]
    fn successor_increments_patch() {
        let version: SemVer = "6.5.8".parse().unwrap();

        assert_eq!(version.successor().to_string(), "6.5.9");
    }

    #[test]
    fn successor_rolls_minor() {
        let version: SemVer = "6.5.9".parse().unwrap();

        assert_eq!(version.successor().to_string(), "6.6.0");
    }

    #[test]
    fn successor_rolls_major() {
        let version: SemVer = "6.9.9".parse().unwrap();

        assert_eq!(version.successor().to_string(), "7.0.0");
    }

    #[test]
    fn generic_semver_works_with_u32() {
        let version: SemVer<u32> = "4294967295.9.9".parse().unwrap();

        assert_eq!(*version.major(), u32::MAX);
    }

    #[test]
    fn generic_semver_works_with_u64() {
        let version: SemVer<u64> = "18446744073709551615.9.9".parse().unwrap();

        assert_eq!(*version.major(), u64::MAX);
    }
}
