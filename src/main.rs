//#![deny(warnings)]
extern crate hyper;

extern crate serde;
extern crate serde_json;

extern crate pbr;
extern crate time;

#[macro_use] extern crate log;
#[macro_use] extern crate json;

extern crate lmdb_zero;
extern crate leveldb;

pub mod memory;
pub mod tripsit;
pub mod revmap;

use memory::*;

fn main() {
    let lmdb_env = lmdb_create_env("./db");
    let lmdb_db = lmdb_open(&lmdb_env);

    let memory = memory::Memory::new(&lmdb_env, &lmdb_db);

    let psydex = tripsit::PsyDex::new(memory);

    for (key, _) in &psydex.categories.tags {
    	println!("{:?}", key);
    }

    println!("{:?}", psydex.categories.get_tag_entries("psychedelic"));
}
