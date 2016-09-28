use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DrugInteraction {
	pub status: Option<String>,
	pub note: Option<String>
}

#[derive(Debug)]
pub struct Drug {
	name: String,

	/*
		<drug>
			-> <interaction>
				-> status: <foo>[& <bar>]
				[-> note]
	*/
	interactions: HashMap<String, DrugInteraction>
}

impl Drug {
	#[inline]
	pub fn new (name: &str) -> Drug {
		Drug {
			name: name.to_string(),
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
}
