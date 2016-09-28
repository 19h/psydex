use std::collections::HashMap;

#[derive(Debug)]
pub struct DrugInteraction {
	status: Option<String>,
	note: Option<String>
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
	pub fn add_interaction (&mut self, name: &str, interaction: DrugInteraction) {
		if self.interactions.contains_key(name) {
			debug!("Interaction between {:?} and {:?} already known.", self.name, name);
			return;
		}

		self.interactions.insert(name.to_string(), interaction);
	}
}