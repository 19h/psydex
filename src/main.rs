//#![deny(warnings)]
extern crate hyper;

#[macro_use] extern crate log;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate json;

mod tripsit;

fn main() {
    let _psydex = tripsit::PsyDex::new();
}
