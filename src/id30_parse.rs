use std::fmt;

use crate::Id30;

/// `Id30Parse` represents the successful result of parsing an id30 string:
///  - an [`Id30`] value, `id30`
///  - a boolean, `is_canonical`, which is `true` iff the parsed string was in
///    canonical encoding
///
/// When accepting id30 as input, non-canonical encodings should preferably
/// redirect the user to a canonical encoding of the input. For example, when
/// id30 is used in URLs, the server should respond with a redirect when the
/// id30 is not canonical. Parsing the given input as `Id30Parse` yields the
/// information necessary to implement this.
///
/// ```
/// # fn main() -> Result<(), id30::ParseError> {
/// let parse: id30::Id30Parse = "78ddpa".parse()?;
/// assert_eq!(u32::from(parse.id30), 243709642);
/// assert!(parse.is_canonical);
/// assert_eq!(&parse.id30.to_string(), "78ddpa");
///
/// let parse: id30::Id30Parse = "78DDPA".parse()?;
/// assert_eq!(u32::from(parse.id30), 243709642);
/// assert!(!parse.is_canonical);
/// assert_eq!(&parse.id30.to_string(), "78ddpa");
/// # Ok(())}
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Id30Parse {
    #[allow(missing_docs)]
    pub id30: Id30,

    #[allow(missing_docs)]
    pub is_canonical: bool,
}

/// The given string was not a valid `id30`
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The given string did not have length 6
    InvalidLength,

    /// The given string contained invalid characters, see the [crate] root
    /// for documentation of the alphabet
    InvalidCharacters,
}

impl fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidLength => "invalid length for Id30".fmt(fmt),
            ParseError::InvalidCharacters => "one or more invalid characters in string".fmt(fmt),
        }
    }
}

impl std::error::Error for ParseError {}
