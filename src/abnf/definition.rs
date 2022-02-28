use super::{super::lex::LexerState, abnf_type, BnfState};

#[derive(Clone, Copy, Debug)]
pub enum RepeatTimes {
    Times(i64),
    Infinity,
}

#[derive(Clone, Debug)]
pub enum BnfDefinition {
    Series(Vec<Box<BnfDefinition>>),
    Choose(Vec<Box<BnfDefinition>>),
    Terminal(String),
    Rule(String),
    Range((i64, i64)),
    Group(Box<BnfDefinition>),
    Options(Box<BnfDefinition>),
    Repeat((RepeatTimes, RepeatTimes, Box<BnfDefinition>)),
}

impl BnfDefinition {
    pub fn new(src: &str, abnf_lexer: &mut LexerState<BnfState>) -> Result<Box<Self>, ()> {
        Self::impl_new(src, abnf_lexer, abnf_type::TOKEN_END_TYPE)
    }

    fn impl_new(
        src: &str,
        abnf_lexer: &mut LexerState<BnfState>,
        end: &str,
    ) -> Result<Box<Self>, ()> {
        let mut token = abnf_lexer.next(src)?;

        let mut has_choose = false;
        let mut result: Vec<Box<BnfDefinition>> = Vec::new();
        loop {
            if token.get_type().eq(end) {
                break;
            }

            match token.get_type() {
                abnf_type::TOKEN_LEFT_OPTIONS_TYPE => {
                    result.push(Box::new(BnfDefinition::Options(Self::impl_new(
                        src,
                        abnf_lexer,
                        abnf_type::TOKEN_RIGHT_OPTIONS_TYPE,
                    )?)))
                }

                abnf_type::TOKEN_LEFT_PARENTHESIS_TYPE => {
                    result.push(Box::new(BnfDefinition::Group(Self::impl_new(
                        src,
                        abnf_lexer,
                        abnf_type::TOKEN_RIGHT_PARENTHESIS_TYPE,
                    )?)))
                }

                abnf_type::TOKEN_NAME_TYPE | abnf_type::TOKEN_REQUIREMENT_TYPE => result.push(
                    Box::new(BnfDefinition::Rule(String::from(token.get_value()))),
                ),

                abnf_type::TOKEN_TERMINAL_TYPE => result.push(Box::new(BnfDefinition::Terminal(
                    String::from(token.get_value()),
                ))),

                // TODO range
                abnf_type::TOKEN_RANGE_TYPE => return Err(()),

                // TODO repeat
                abnf_type::TOKEN_VARIABLE_TYPE => return Err(()),

                abnf_type::TOKEN_CHOOSE_TYPE => {
                    has_choose = true;

                    if result.len().eq(&0) {
                        return Err(());
                    } else if result.len().gt(&1) {
                        result = vec![Box::new(BnfDefinition::Series(result))];
                    }
                }

                _ => return Err(()),
            };

            token = abnf_lexer.next(src)?;
        }

        if has_choose {
            if result.len().le(&1) {
                Err(())
            } else {
                Ok(Box::new(BnfDefinition::Choose(result)))
            }
        } else {
            println!("here");
            if result.is_empty() {
                Err(())
            } else {
                if result.len().eq(&1) {
                    result.into_iter().next().ok_or(())
                } else {
                    Ok(Box::new(BnfDefinition::Series(result)))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::new_lexer_state;
    use super::*;

    #[test]
    fn test_series() {
        let src = "rule = rule1 rule2 rule3;";
        let mut lex_state = new_lexer_state();

        let _ = lex_state.next(src);
        let _ = lex_state.next(src);

        if let Ok(definition) = BnfDefinition::new(src, &mut lex_state) {
            match *definition {
                BnfDefinition::Series(_) => {}
                _ => panic!("error type"),
            }
        } else {
            panic!("error");
        }
    }
}
