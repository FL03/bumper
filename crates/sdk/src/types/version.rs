/*
    Appellation: version <module>
    Created At: 2026.08.29:11:35:04
    Contrib: @FL03
*/
use crate::error::VersionParseError;
use crate::types::{parse_semver, SemVer, SuffixedSemVer};

fn parse_version<T>(input: &str) -> Result<Version<T>, VersionParseError>
where
    T: core::str::FromStr,
{
    let (version, remaining) = parse_semver::<T>(input).map_err(VersionParseError::SemVer)?;

    if remaining.is_empty() {
        return Ok(Version::SemVer(version));
    }

    let (separator, suffix) = parse_suffix(remaining)?;

    Ok(Version::Suffixed(SuffixedSemVer {
        version,
        separator,
        suffix,
    }))
}


fn parse_suffix(input: &str) -> Result<(String, String), VersionParseError> {
    if input.is_empty() {
        return Err(VersionParseError::InvalidSuffix);
    }

    let separator = match input.as_bytes()[0] {
        b'-' => "-",
        b'+' => "+",
        _ => return Err(VersionParseError::InvalidFormat),
    };

    let suffix = &input[1..];

    if suffix.is_empty() {
        return Err(VersionParseError::InvalidSuffix);
    }

    // Whitespace is never valid inside a suffix.
    if suffix.chars().any(char::is_whitespace) {
        return Err(VersionParseError::InvalidSuffix);
    }

    // Do not allow another separator immediately after the separator.
    //
    // This prevents:
    //
    // 6.5.9--
    // 6.5.9-+
    // 6.5.9-+dev
    //
    if suffix.starts_with('-') || suffix.starts_with('+') {
        return Err(VersionParseError::InvalidSuffix);
    }

    Ok((separator.to_owned(), suffix.to_owned()))
}


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
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
        parse_version::<T>(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_dev_suffix() {
        let version: Version = "6.5.9-dev".parse().unwrap();

        assert!(version.is_suffixed());
        assert_eq!(version.to_string(), "6.5.9-dev");
    }

    #[test]
    fn parses_rc_suffix() {
        let version: Version = "6.5.9-rc".parse().unwrap();

        assert_eq!(version.to_string(), "6.5.9-rc");
    }

    #[test]
    fn parses_build_suffix() {
        let version: Version = "6.5.9+build".parse().unwrap();

        assert_eq!(version.to_string(), "6.5.9+build");
    }

    #[test]
    fn parses_hyphenated_suffix() {
        let version: Version = "6.5.9-nightly.2026".parse().unwrap();

        assert_eq!(version.to_string(), "6.5.9-nightly.2026");
    }

    #[test]
    fn plain_semver_becomes_plain_variant() {
        let version: Version = "6.5.9".parse().unwrap();

        assert!(matches!(version, Version::SemVer(_)));
    }

    #[test]
    fn suffixed_version_becomes_suffixed_variant() {
        let version: Version = "6.5.9-dev".parse().unwrap();

        assert!(matches!(version, Version::Suffixed(_)));
    }

    #[test]
    fn rejects_empty_suffix() {
        assert!("6.5.9-".parse::<Version>().is_err());
        assert!("6.5.9+".parse::<Version>().is_err());
    }

    #[test]
    fn rejects_whitespace_suffix() {
        assert!("6.5.9-dev foo".parse::<Version>().is_err());
    }

    #[test]
    fn rejects_double_separator_suffix() {
        assert!("6.5.9--dev".parse::<Version>().is_err());
        assert!("6.5.9-+dev".parse::<Version>().is_err());
    }

    #[test]
    fn preserves_separator() {
        let version: Version = "6.5.9+build".parse().unwrap();

        match version {
            Version::Suffixed(version) => {
                assert_eq!(version.separator(), "+");
                assert_eq!(version.suffix(), "build");
            }
            _ => panic!("expected suffixed version"),
        }
    }
}
