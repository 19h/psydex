//#![deny(warnings)]
extern crate hyper;

extern crate serde;
extern crate serde_json;

#[macro_use] extern crate log;
#[macro_use] extern crate json;

extern crate lmdb_zero;

mod cache;
mod tripsit;

use cache::*;

fn main() {
    let lmdb_env = lmdb_create_env("./db");
    let lmdb_db = lmdb_open(&lmdb_env);

    let mut cache = cache::Cache::new(&lmdb_env, &lmdb_db);

    cache.put("yolo", &"motherfucker".as_bytes().to_vec());

    println!("{:?}", cache.get("yolo"));

    let _psydex = tripsit::PsyDex::new();
}
