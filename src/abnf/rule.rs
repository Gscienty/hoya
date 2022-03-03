use super::BnfDefinition;

#[derive(Clone)]
pub struct BnfRule {
    name: String,
    definition: Box<BnfDefinition>,
}

impl BnfRule {
    pub fn new(rule_name: &str, definition: Box<BnfDefinition>) -> Self {
        BnfRule {
            name: String::from(rule_name),
            definition,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_definition(&self) -> &BnfDefinition {
        self.definition.as_ref()
    }

    pub fn append_select(&mut self, definition: Box<BnfDefinition>) {
        match &mut *self.definition {
            BnfDefinition::Select(value) => value.push(definition),
            _ => {
                self.definition = Box::new(BnfDefinition::Select(vec![
                    self.definition.clone(),
                    definition,
                ]))
            }
        }
    }
}
