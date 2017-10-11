// Implementation of Serde Serialiser & Deserialiser for Datastore entities

mod error;
mod ser;
mod de;

/*
By convention a Serde data format crate provides the following in the root module or re-exported
from the root module:

* [x] An Error type common to both serialization and deserialization.
* [x] A Result typedef which is equivalent to std::result::Result<T, Error>.
* [x] A Serializer type which implements serde::Serializer.
* [x] A Deserializer type which implements serde::Deserializer.
* [ ] One or more to_abc functions depending on what types the format supports serializing to.
      For example to_string which returns a String, to_bytes which returns a Vec<u8>, or to_writer
      which writes into an io::Write.
* [ ] One or more from_xyz functions depending on what types the format supports deserializing from.
      For example from_str which takes a &str, from_bytes which takes a &[u8], or from_reader which
      takes an io::Read.
*/

pub use self::error::{Error, Result};

#[cfg(test)]
mod ser_tests;

#[cfg(test)]
mod de_tests;

#[cfg(test)]
mod roundtrip_tests;
