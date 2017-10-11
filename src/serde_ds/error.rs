use std;
use std::fmt::{self, Display};
use std::num;

use serde::{ser, de};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    SerializationError(String),
    DeserializationError(String),
    ParseIntError(),
    UnsupportedValueType(&'static str),
    UnsupportedKeyType(),
    NonSelfDescribingType(),
    ExpectedType(&'static str),
    DoubleSizeMismatch(),
    NotYetImplemented(&'static str),
}

impl From<num::ParseIntError> for Error {
    fn from(_: num::ParseIntError) -> Self {
        Error::ParseIntError()
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::SerializationError(format!("{}", msg))
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::DeserializationError(format!("{}", msg))
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // Import the 'Error' trait locally scoped to prevent naming conflicts and access the
        // 'description' method.
        use std::error::Error;
        use serde_ds::Error::*;

        // Some errors carry extra information that may be relevant when 'Display' is called on
        // the error. This extends the simple error descriptions where appropriate.
        match *self {
            SerializationError(ref msg) =>
                fmt.write_fmt(format_args!("{}: {}", self.description(), msg)),
            DeserializationError(ref msg) =>
                fmt.write_fmt(format_args!("{}: {}", self.description(), msg)),
            NotYetImplemented(ref t) =>
                fmt.write_fmt(format_args!("{}: {}", self.description(), t)),
            ExpectedType(t) =>
                fmt.write_fmt(format_args!("{}: expected {}", self.description(), t)),
            UnsupportedValueType(t) =>
                fmt.write_fmt(format_args!("{}: {}", self.description(), t)),
            _ => fmt.write_str(self.description())
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ParseIntError() =>
                "could not parse integer from value",
            Error::NonSelfDescribingType() =>
                "cannot automatically determine desired type representation",
            Error::DoubleSizeMismatch() =>
                "floating-point value too large for chosen type",
            Error::SerializationError(_) =>
                "error during serialization",
            Error::DeserializationError(_) =>
                "error during deserialization",
            Error::NotYetImplemented(_) =>
                "support for type not yet implemented",
            Error::ExpectedType(_) =>
                "unexpected type encountered",
            Error::UnsupportedValueType(_) =>
                "unsupported value type",
            Error::UnsupportedKeyType() =>
                "non-string key types are unsupported"
        }
    }
}
