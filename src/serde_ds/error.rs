use std;
use std::fmt::{self, Display};
use std::str::FromStr;
use std::num;

use serde::{ser, de};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    SerialisationError(String),
    DeserialisationError(String),
    ParseIntError(),
    UnsupportedValueType(&'static str),
    UnsupportedCompoundType(&'static str),
    UnsupportedKeyType(),
    NonSelfDescribingType(),
    ExpectedType(&'static str),
    DoubleSizeMismatch(),
}

impl From<num::ParseIntError> for Error {
    fn from(_: num::ParseIntError) -> Self {
        Error::ParseIntError()
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::SerialisationError(format!("{}", msg))
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::DeserialisationError(format!("{}", msg))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        unimplemented!()
    }
}
