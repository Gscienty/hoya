use super::{
    super::{
        abnf::{AbnfDefinition, AbnfRule, AbnfRules},
        lex::Token,
    },
    GrammarTreeNode,
};

pub struct GrammarParser {
    abnf_rules: AbnfRules,
}

impl GrammarParser {
    pub fn new() -> Self {
        GrammarParser {
            abnf_rules: AbnfRules::new(),
        }
    }

    pub fn add_rules(&mut self, rules_src: &str) -> Result<(), ()> {
        self.abnf_rules.parse(rules_src)
    }

    pub fn get_rule(&self, rule_key: &str) -> Result<&AbnfRule, ()> {
        self.abnf_rules
            .result()
            .get(&String::from(rule_key))
            .ok_or(())
    }

    pub fn parse<'a>(
        &self,
        rule_key: &str,
        token_parser: &(dyn Fn() -> Result<Token, ()> + 'a),
    ) -> Result<Box<GrammarTreeNode>, ()> {
        // make root
        let enter_rule = self.get_rule(rule_key)?;

        self.parse_abnf_definition(enter_rule.get_definition(), token_parser)
            .and_then(|mut node| {
                node.set_rule(true, String::from(enter_rule.get_name()));

                Ok(node)
            })
    }

    fn parse_abnf_definition<'a>(
        &self,
        definition: &AbnfDefinition,
        token_parser: &(dyn Fn() -> Result<Token, ()> + 'a),
    ) -> Result<Box<GrammarTreeNode>, ()> {
        let mut node = Box::new(GrammarTreeNode::new());

        match definition {
            AbnfDefinition::Series(series_nodes) => {
                for series_node in series_nodes {
                    node.append_child(
                        self.parse_abnf_definition(series_node, token_parser)
                            .and_then(|mut node| {
                                node.set_rule(false, String::from(series_node.get_name()));
                                Ok(node)
                            })?,
                    )
                }
            }

            AbnfDefinition::Select(definitions) => {
                let mut selected = false;
                for definition in definitions {
                    if let Ok(mut child_node) = self.parse_abnf_definition(definition, token_parser)
                    {
                        child_node.set_rule(false, String::from(definition.get_name()));
                        node.append_child(child_node);
                        selected = true;

                        break;
                    }
                }

                if !selected {
                    return Err(());
                }
            }

            AbnfDefinition::Terminal(value) => {
                node.set_value(self.parse_abnf_definition_terminal(&value, token_parser)?);
            }

            AbnfDefinition::Rule(rule_node) => {
                let enter_rule = self.get_rule(rule_node.as_str())?;

                node.append_child(
                    self.parse_abnf_definition(enter_rule.get_definition(), token_parser)
                        .and_then(|mut node| {
                            node.set_rule(true, String::from(enter_rule.get_name()));

                            Ok(node)
                        })?,
                );
            }

            AbnfDefinition::Group(definition) => node.append_child(
                self.parse_abnf_definition(definition, token_parser)
                    .and_then(|mut node| {
                        node.set_rule(false, String::from(definition.get_name()));

                        Ok(node)
                    })?,
            ),

            AbnfDefinition::Options(definition) => {
                if let Ok(mut child_node) = self.parse_abnf_definition(definition, token_parser) {
                    child_node.set_rule(false, String::from(definition.get_name()));
                    node.append_child(child_node);
                }
            }

            _ => return Err(()),
        }

        Ok(node)
    }

    fn parse_abnf_definition_terminal<'a>(
        &self,
        definition_value: &String,
        token_parser: &(dyn Fn() -> Result<Token, ()> + 'a),
    ) -> Result<String, ()> {
        let token = token_parser()?;

        if definition_value.eq(token.get_value()) {
            Ok(String::from(token.get_value()))
        } else {
            Err(())
        }
    }
}
