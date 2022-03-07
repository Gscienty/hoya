use super::AbnfDefinition;

#[derive(Clone)]
pub struct AbnfRule {
    name: String,
    definition: Box<AbnfDefinition>,
}

impl AbnfRule {
    pub fn new(rule_name: &str, definition: Box<AbnfDefinition>) -> Self {
        AbnfRule {
            name: String::from(rule_name),
            definition,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_definition(&self) -> &AbnfDefinition {
        self.definition.as_ref()
    }

    pub fn append_select(&mut self, definition: Box<AbnfDefinition>) {
        match &mut *self.definition {
            AbnfDefinition::Select(value) => value.push(definition),
            _ => {
                self.definition = Box::new(AbnfDefinition::Select(vec![
                    self.definition.clone(),
                    definition,
                ]))
            }
        }
    }
}
