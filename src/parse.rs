/*
    Appellation: parse <module>
    Created At: 2026.08.29:11:16:03
    Contrib: @FL03
*/
use crate::error::{SemVerParseError, VersionParseError};
use crate::types::{SemVer, SuffixedSemVer, Version};
use core::str::FromStr;

pub(crate) fn parse_component<'a, T>(input: &'a str) -> Result<(T, &'a str), SemVerParseError>
where
    T: FromStr,
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

//
// ============================================================================
// SemVer parser
// ============================================================================
//

pub(crate) fn parse_semver<T>(input: &str) -> Result<(SemVer<T>, &str), SemVerParseError>
where
    T: FromStr,
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

//
// ============================================================================
// Suffix parser
// ============================================================================
//
// Supported separators:
//
// -
// +
//
// Examples:
//
// 6.5.9-dev
// 6.5.9-rc
// 6.5.9-nightly.2026
// 6.5.9+build
//
// The separator is preserved exactly in SuffixedSemVer.
//

pub(crate) fn parse_suffix(input: &str) -> Result<(String, String), VersionParseError> {
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

//
// ============================================================================
// Version parser
// ============================================================================
//

pub(crate) fn parse_version<T>(input: &str) -> Result<Version<T>, VersionParseError>
where
    T: FromStr,
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
