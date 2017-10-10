#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate maplit;

extern crate serde;
extern crate serde_json;
extern crate base64;
extern crate chrono;

#[cfg(test)]
extern crate serde_bytes;

// TODO: Rename -> api
pub mod datastore;
pub mod serde_ds;
