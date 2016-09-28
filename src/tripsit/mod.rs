extern crate hyper;
extern crate json;

use std::str;
use std::collections::HashMap;

mod client;

mod drug;
use self::drug::{DrugInteraction, Drug};

static TS_URL_ALLDRUGS: &'static str = "http://tripbot.tripsit.me/api/tripsit/getAllDrugNames";
static TS_URL_DRUG_PREFIX: &'static str = "http://tripbot.tripsit.me/api/tripsit/getDrug?name=";

pub struct PsyDex {
    pub drugs: HashMap<String, Drug>
}

impl PsyDex {
    #[inline]
    pub fn new() -> PsyDex {
        let mut psydex = PsyDex {
            drugs: HashMap::new()
        };

        psydex.load_drug_names();
        psydex.load_drug_interactions();

        return psydex;
    }

    #[inline]
    fn load_drug_names (&mut self) {
        let res = client::HTTP::new().get(&*TS_URL_ALLDRUGS);
        let mut drugs_obj = json::parse(str::from_utf8(&res).unwrap()).unwrap();

        let drugs = drugs_obj["data"][0].take();

        for (_, drug_name) in drugs.members().enumerate() {
            match drug_name.as_str() {
                Some(name) => {
                    self.drugs.insert(name.to_string(), Drug::new(name));
                },
                None => {
                    println!("Invalid object in structure: {:?}", drug_name);
                }
            }
        }
    }

    fn ts_process_combos (drug: &json::JsonValue) -> HashMap<String, DrugInteraction> {
        HashMap::new()
    }

    #[inline]
    fn load_drug_interactions (&mut self) {
        if self.drugs.len() == 0 {
            unreachable!("Either Tripsit returned invalid data or a contract was breached");
        }

        let mut i = 0;

        for (drug_name, _) in &self.drugs {
            if i == 1 {
                break;
            }

            let ts_drug_uri = ((&*TS_URL_DRUG_PREFIX).to_string() + drug_name).to_string();
            let res = client::HTTP::new().get(&ts_drug_uri);
            let mut drug_obj = json::parse(str::from_utf8(&res).unwrap()).unwrap();

            println!("{:?}", drug_obj);

            i += 1;
        }
    }
}