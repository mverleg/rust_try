#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde;
extern crate serde_json;

pub mod data {
    pub mod example;
    pub mod request;
    pub mod response;
    pub mod transfer;
}
