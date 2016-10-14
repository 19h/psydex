extern crate hyper;
extern crate json;

extern crate pbr;

use pbr::ProgressBar;

use memory;

use time;

use std::str;
use std::collections::HashMap;

pub mod client;

pub mod drug;
use self::drug::{DrugInteraction, Drug};

static TS_URL_ALLDRUGS: &'static str = "http://tripbot.tripsit.me/api/tripsit/getAllDrugNames";
static TS_URL_DRUG_PREFIX: &'static str = "http://tripbot.tripsit.me/api/tripsit/getDrug?name=";

static TS_DRUG_CACHE_EXPIRE: u64 = 7200;

#[inline]
fn client_cond_get (url: &str, memory: &mut memory::Memory, client: &client::HTTP) -> Vec<u8> {
    /* unixtime */
    let now = time::now().to_timespec().sec as u64;

    let mut url_ts_id = url.to_string();
            url_ts_id.push_str(&"_ts");

    let known_url_ts = memory.get_u64(&url_ts_id);

    if known_url_ts.is_some() && (known_url_ts.unwrap() > TS_DRUG_CACHE_EXPIRE) {
        return memory.get(&url).unwrap();
    }

    let res = client.get(url);

    let _ = memory.put_u64(&url_ts_id, &now);

    memory.put(&url, &res);

    res
}

pub struct PsyDex<'a> {
    pub drugs: HashMap<String, Drug>,

    client: client::HTTP,
    memory: &'a memory::Memory<'a>
}

impl<'a> PsyDex<'a> {
    #[inline]
    pub fn new(memory: &'a memory::Memory) -> PsyDex<'a> {
        let mut psydex = PsyDex {
            drugs: HashMap::new(),

            client: client::HTTP::new(),
            memory: memory
        };

        psydex.load_drug_names();
        psydex.load_drug_interactions();

        return psydex;
    }

    #[inline]
    fn load_drug_names (&mut self) {
        let res = self.client.get(&*TS_URL_ALLDRUGS);
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
    fn drug_name_window (drugs: &mut HashMap<String, Drug>) -> u32 {
        let mut window = 0;

        for (drug_name, mut drug) in drugs.iter() {
            let len = drug_name.len();

            if len > window {
                window = len;
            }
        }

        window as u32
    }

    #[inline]
    fn drug_name_pad (drug_name: &'a str, window: u32) -> String {
        let pad_size = window - drug_name.len() as u32;
        let pad = (0..pad_size).map(|_| " ").collect::<String>();

        let mut padded_drug_name = drug_name.to_string();

        padded_drug_name.push_str(&pad);

        padded_drug_name.to_string()
    }

    #[inline]
    fn load_drug_interaction (drug_name: &str,
                              drug: &mut Drug,
                              memory: &mut memory::Memory,
                              client: &client::HTTP) {
        let ts_drug_uri = ((&*TS_URL_DRUG_PREFIX).to_string() + drug_name).to_string();

        let res = client_cond_get(&ts_drug_uri, memory, client);

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
    }

    #[inline]
    fn load_drug_interactions (&mut self) {
        if self.drugs.len() == 0 {
            unreachable!("Either Tripsit returned invalid data or a contract was breached");
        }

        let mut drug_name_window = Self::drug_name_window(&mut self.drugs);

        let mut progress = ProgressBar::new(self.drugs.len() as u64);

        progress.format("╢==+╟");

        for (drug_name, mut drug) in &mut self.drugs {
            let t_dn = &drug_name.to_string();

            progress.message(&Self::drug_name_pad(t_dn, drug_name_window));

            self::PsyDex::load_drug_interaction(&drug_name, &mut drug, &mut self.memory, &self.client);

            progress.inc();
        }

        progress.finish();
    }
}
