use super::BnfDefinition;

pub struct BnfRule {
    name: String,
    definition: Box<BnfDefinition>,
}

impl BnfRule {
    pub fn new(rule_name: &str) -> Self {
        BnfRule {
            name: String::from(rule_name),
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}
