use super::{abnf_type, new_lexer_state, AbnfDefinition, AbnfRule};
use std::collections::HashMap;

pub struct AbnfRules {
    rules: HashMap<String, AbnfRule>,
}

impl AbnfRules {
    pub fn new() -> Self {
        AbnfRules {
            rules: HashMap::new(),
        }
    }

    pub fn result(&self) -> &HashMap<String, AbnfRule> {
        &self.rules
    }

    pub fn parse(&mut self, src: &str) -> Result<(), ()> {
        let mut lexer_state = new_lexer_state();
        loop {
            let rule_name_token = lexer_state.next(src)?;
            if rule_name_token.get_type().eq(abnf_type::ABNF_TOKEN_EOF) {
                break Ok(());
            }
            let rule_name = String::from(rule_name_token.get_value());

            match lexer_state.next(src)?.get_value() {
                "=" => {
                    if self.rules.contains_key(&rule_name) {
                        return Err(());
                    }
                }
                "=/" => {
                    if !self.rules.contains_key(&rule_name) {
                        return Err(());
                    }
                }
                _ => return Err(()),
            }
            let definition = AbnfDefinition::new(src, &mut lexer_state)?;

            if let Some(rule) = self.rules.get_mut(&rule_name) {
                rule.append_select(definition);
            } else {
                let rule = AbnfRule::new(rule_name.as_str(), definition);
                self.rules.insert(rule_name, rule);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_line() {
        let mut builder = AbnfRules::new();

        assert_eq!(Ok(()), builder.parse("rule = rule2 / rule3;"));

        assert_eq!(1, builder.result().len());
    }

    #[test]
    fn test_build_twice_line() {
        let mut builder = AbnfRules::new();

        assert_eq!(Ok(()), builder.parse("rule = rule2 / rule3;"));
        assert_eq!(Ok(()), builder.parse("rule2 = rule3 / rule4;"));

        assert_eq!(2, builder.result().len());
    }

    #[test]
    fn test_build_multi_lines() {
        let mut builder = AbnfRules::new();

        assert_eq!(
            Ok(()),
            builder.parse("rule = rule2 / rule3;rule2 = rule3 / rule4;")
        );

        assert_eq!(2, builder.result().len());
    }

    #[test]
    fn test_build_twice_multi_lines() {
        let mut builder = AbnfRules::new();

        assert_eq!(
            Ok(()),
            builder.parse("rule = rule2 / rule3;rule2 = rule3 / rule4;")
        );
        assert_eq!(
            Ok(()),
            builder.parse("rule3 = rule2 / rule3;rule4 = rule3 / rule4;rule5 = rule6;")
        );

        assert_eq!(5, builder.result().len());
    }

    #[test]
    fn test_build_append() {
        let mut builder = AbnfRules::new();

        assert_eq!(
            Ok(()),
            builder.parse("rule = rule2 / rule3;rule =/ rule3 / rule4;")
        );

        assert_eq!(1, builder.result().len());
    }
}
