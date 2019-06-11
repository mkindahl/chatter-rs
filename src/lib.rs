extern crate serde_cbor;
extern crate uuid;
#[macro_use]
extern crate serde_derive;

pub mod error;
pub mod gossip;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
