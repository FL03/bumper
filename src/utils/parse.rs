/*
    Appellation: parse <module>
    Created At: 2026.08.29:11:33:15
    Contrib: @FL03
*/
use crate::error::{SemVerParseError, VersionParseError};
use crate::types::{SemVer, SuffixedSemVer, Version};
use core::str::FromStr;

pub(crate) fn parse_component<T>(input: &str) -> Result<(T, &str), SemVerParseError>
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
