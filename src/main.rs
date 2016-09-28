//#![deny(warnings)]
extern crate hyper;

extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_minihttp as http;
extern crate futures;

#[macro_use] extern crate log;

extern crate iron;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate json;

mod tripsit;

fn main() {
    let _psydex = tripsit::PsyDex::new();
}
