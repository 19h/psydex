extern crate hyper;
extern crate json;

use std::str;

mod client;

static TS_URL_ALLDRUGS: &'static str = "http://tripbot.tripsit.me/api/tripsit/getAllDrugNames";
static TS_URL_DRUG_PREFIX: &'static str = "http://tripbot.tripsit.me/api/tripsit/getDrug?name=";

pub struct TripsitDex<'a> {
    drug_names: Box<Vec<&'a str>>
}

impl<'a> TripsitDex<'a> {
    #[inline]
    pub fn new() -> Result<TripsitDex<'a>, ()> {
        Ok(TripsitDex {
            drug_names: Box::new(Vec::new())
        })
    }

    #[inline]
    pub fn load_drug_names (&mut self) {
        let res = client::HTTP::new().get(&*TS_URL_ALLDRUGS);
        let mut drugs_obj = json::parse(str::from_utf8(&res).unwrap()).unwrap();

        let drugs = drugs_obj["data"][0].take();

        let mut drug_names: Vec<&'a str> = Vec::new();

        for (v, i) in drugs.members().enumerate() {
            let drugstr = i.as_str();

            match drugstr {
                Some(name) => {
                    drug_names.push(name);
                },
                None => {}
            }
        }

        return ();
    }
}