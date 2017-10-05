#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate base64;
extern crate chrono;

pub mod datastore;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
