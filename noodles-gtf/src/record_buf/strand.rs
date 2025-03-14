//! GTF record strand.

use std::{error, fmt, str::FromStr};

/// A GTF record strand.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Strand {
    /// Forward strand (`+`).
    Forward,
    /// Reverse strand (`-`).
    Reverse,
}

/// An error returned when a raw GTF record strand fails to parse.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// The input is empty.
    Empty,
    /// The strand is invalid.
    Invalid(String),
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("empty input"),
            Self::Invalid(s) => write!(f, "expected {{+, -}}, got {s}"),
        }
    }
}

impl FromStr for Strand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Err(ParseError::Empty),
            "+" => Ok(Self::Forward),
            "-" => Ok(Self::Reverse),
            _ => Err(ParseError::Invalid(s.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() -> Result<(), ParseError> {
        assert_eq!("+".parse::<Strand>()?, Strand::Forward);
        assert_eq!("-".parse::<Strand>()?, Strand::Reverse);

        assert_eq!("".parse::<Strand>(), Err(ParseError::Empty));
        assert_eq!(
            "!".parse::<Strand>(),
            Err(ParseError::Invalid(String::from("!")))
        );

        Ok(())
    }
}
