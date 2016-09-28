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
                    error!("Invalid object in structure: {:?}", drug_name);
                }
            }
        }
    }

    #[inline]
    fn load_drug_interactions (&mut self) {
        if self.drugs.len() == 0 {
            unreachable!("Either Tripsit returned invalid data or a contract was breached");
        }

        for (drug_name, drug) in &mut self.drugs {
            let ts_drug_uri = ((&*TS_URL_DRUG_PREFIX).to_string() + drug_name).to_string();

            println!("Loading {:?}", ts_drug_uri);

            let res = client::HTTP::new().get(&ts_drug_uri);
            let mut ts_json_res = json::parse(str::from_utf8(&res).unwrap()).unwrap();

            let ts_drug = ts_json_res["data"][0]["combos"].take();

            for (i, (combo_drug_name, combo_drug)) in ts_drug.entries().enumerate() {
                println!("{:?}, {:?}, {:?}", i, combo_drug_name, combo_drug);

                let combo_drug = combo_drug.clone();

                let interaction_status = match combo_drug["status"].as_str() {
                    Some(status) => Some(status.to_string()),
                    None => None
                };

                let interaction_note = match combo_drug["note"].as_str() {
                    Some(note) => Some(note.to_string()),
                    None => None
                };

                drug.add_interaction(combo_drug_name, &DrugInteraction {
                    status: interaction_status,
                    note: interaction_note
                });
            }

            println!("{:?}", drug);
        }
    }
}
