use std;
use std::fmt::{self, Display};

use serde::{ser, de};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    SomeError()
}

// https://serde.rs/error-handling.html

impl ser::Error for Error {
    fn custom<T: Display> (msg: T) -> Self {
        unimplemented!()
    }
}

impl de::Error for Error {
    fn custom<T: Display> (msg: T) -> Self {
        unimplemented!()
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
