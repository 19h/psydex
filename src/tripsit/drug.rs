use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct DrugInteraction {
	pub status: Option<String>,
	pub note: Option<String>
}

#[derive(Debug)]
pub struct Drug {
	name: String,

	aliases: HashSet<String>,
	interactions: HashMap<String, DrugInteraction>,
	categories: HashSet<String>
}

impl Drug {
	#[inline]
	pub fn new (name: &str) -> Drug {
		Drug {
			name: name.to_string(),

			aliases: HashSet::new(),
			categories: HashSet::new(),

			interactions: HashMap::new()
		}
	}

	#[inline]
	pub fn add_interaction (&mut self, other_drug_name: &str, interaction: &DrugInteraction) {
		if self.interactions.contains_key(other_drug_name) {
			println!("Interaction between {:?} and {:?} already known.", self.name, other_drug_name);
			return;
		}

		self.interactions.insert(other_drug_name.to_string(), interaction.clone());
	}

	pub fn add_category (&mut self, category: &str) {
		self.categories.insert(category.to_string());
	}

	pub fn add_alias (&mut self, alias: &str) {
		self.aliases.insert(alias.to_string());
	}
}
