//#![deny(warnings)]
extern crate hyper;

extern crate serde;
extern crate serde_json;

extern crate pbr;
extern crate time;

#[macro_use] extern crate log;
#[macro_use] extern crate json;

extern crate lmdb_zero;

pub mod memory;
pub mod tripsit;

use memory::*;

fn main() {
    let lmdb_env = lmdb_create_env("./db");
    let lmdb_db = lmdb_open(&lmdb_env);

    let mut memory = memory::Memory::new(&lmdb_env, &lmdb_db);

    let _psydex = tripsit::PsyDex::new(&memory);
}
