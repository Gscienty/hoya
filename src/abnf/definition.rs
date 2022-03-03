use super::{super::lex::LexerState, abnf_type, BnfState};
use std::str::Chars;

#[derive(Clone, Copy, Debug)]
pub enum RepeatTimes {
    Times(i64),
    Infinity,
}

#[derive(Clone, Debug)]
pub enum BnfDefinition {
    Series(Vec<Box<BnfDefinition>>),
    Select(Vec<Box<BnfDefinition>>),
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

    fn range_numeric<'a>(n: char, chars: &'a mut Chars) -> Result<i64, ()> {
        let mut result: i64 = 0;
        loop {
            if let Some(v) = chars.next() {
                if v.eq(&'-') {
                    break Ok(result);
                }

                match n {
                    'b' => {
                        result = (result << 1)
                            + match v {
                                '0'..='1' => (v as i64) - ('0' as i64),
                                _ => break Err(()),
                            }
                    }
                    'd' => {
                        result = (result * 10)
                            + match v {
                                '0'..='9' => (v as i64) - ('0' as i64),
                                _ => break Err(()),
                            }
                    }
                    'x' => {
                        result = (result << 4)
                            + match v {
                                '0'..='9' => (v as i64) - ('0' as i64),
                                'a'..='z' => (v as i64) - ('a' as i64) + 10,
                                'A'..='Z' => (v as i64) - ('A' as i64) + 10,
                                _ => break Err(()),
                            }
                    }
                    _ => break Err(()),
                }
            } else {
                break Ok(result);
            }
        }
    }

    fn repeat_numeric<'a>(chars: &'a mut Chars) -> (i64, char) {
        let mut result: i64 = 0;
        loop {
            if let Some(v) = chars.next() {
                if v.eq(&'*') {
                    break (result, '*');
                }

                result = (result * 10)
                    + match v {
                        '0'..='9' => (v as i64) - ('0' as i64),
                        _ => break (0, '\0'),
                    }
            } else {
                break (result, '\0');
            }
        }
    }

    fn impl_variable_new(
        content: &str,
        src: &str,
        abnf_lexer: &mut LexerState<BnfState>,
    ) -> Result<Box<Self>, ()> {
        let mut begin = RepeatTimes::Times(0);
        let mut end = RepeatTimes::Infinity;

        if content.chars().nth(0).eq(&Some('*')) {
            if content.len().ne(&1) {
                // *<n>
                let mut chars = content.chars();
                let _ = chars.next();
                let (end_times, _) = Self::repeat_numeric(&mut chars);
                end = RepeatTimes::Times(end_times);
            }
        } else {
            let mut chars = content.chars();
            let (begin_times, end_literal) = Self::repeat_numeric(&mut chars);
            begin = RepeatTimes::Times(begin_times);

            if end_literal.eq(&'*') {
                let (end_times, _) = Self::repeat_numeric(&mut chars);
                end = RepeatTimes::Times(end_times);
            }
        }

        let token = abnf_lexer.next(src)?;
        if token.get_type().eq(abnf_type::ABNF_TOKEN_EOF) {
            return Err(());
        }
        let node =
            match token.get_type() {
                abnf_type::TOKEN_LEFT_OPTIONS_TYPE => Box::new(BnfDefinition::Options(
                    Self::impl_new(src, abnf_lexer, abnf_type::TOKEN_RIGHT_OPTIONS_TYPE)?,
                )),

                abnf_type::TOKEN_LEFT_PARENTHESIS_TYPE => Box::new(BnfDefinition::Group(
                    Self::impl_new(src, abnf_lexer, abnf_type::TOKEN_RIGHT_PARENTHESIS_TYPE)?,
                )),

                abnf_type::TOKEN_NAME_TYPE | abnf_type::TOKEN_REQUIREMENT_TYPE => {
                    Box::new(BnfDefinition::Rule(String::from(token.get_value())))
                }

                abnf_type::TOKEN_TERMINAL_TYPE => {
                    Box::new(BnfDefinition::Terminal(String::from(token.get_value())))
                }

                _ => return Err(()),
            };

        Ok(Box::new(Self::Repeat((begin, end, node))))
    }

    fn impl_range_new(content: &str) -> Result<Box<Self>, ()> {
        let mut chars = content.chars();
        let _ = chars.next(); // skip literal '%'

        if let Some(n) = chars.next() {
            let begin = Self::range_numeric(n, &mut chars)?;
            let end = Self::range_numeric(n, &mut chars)?;

            if begin < end {
                Ok(Box::new(Self::Range((begin, end))))
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    fn impl_new(
        src: &str,
        abnf_lexer: &mut LexerState<BnfState>,
        end: &str,
    ) -> Result<Box<Self>, ()> {
        let mut newly_definition = true;
        let mut result: Vec<Box<BnfDefinition>> = Vec::new();
        loop {
            let token = abnf_lexer.next(src)?;

            if token.get_type().eq(end) {
                break;
            }

            let node = match token.get_type() {
                abnf_type::TOKEN_LEFT_OPTIONS_TYPE => Box::new(BnfDefinition::Options(
                    Self::impl_new(src, abnf_lexer, abnf_type::TOKEN_RIGHT_OPTIONS_TYPE)?,
                )),

                abnf_type::TOKEN_LEFT_PARENTHESIS_TYPE => Box::new(BnfDefinition::Group(
                    Self::impl_new(src, abnf_lexer, abnf_type::TOKEN_RIGHT_PARENTHESIS_TYPE)?,
                )),

                abnf_type::TOKEN_NAME_TYPE | abnf_type::TOKEN_REQUIREMENT_TYPE => {
                    Box::new(BnfDefinition::Rule(String::from(token.get_value())))
                }

                abnf_type::TOKEN_TERMINAL_TYPE => {
                    Box::new(BnfDefinition::Terminal(String::from(token.get_value())))
                }

                abnf_type::TOKEN_RANGE_TYPE => Self::impl_range_new(token.get_value())?,

                abnf_type::TOKEN_VARIABLE_TYPE => {
                    Self::impl_variable_new(token.get_value(), src, abnf_lexer)?
                }

                abnf_type::TOKEN_SELECT_TYPE => {
                    newly_definition = true;
                    continue;
                }

                _ => return Err(()),
            };

            if newly_definition || result.is_empty() {
                result.push(node);
            } else {
                if let Some(last_node) = result.last_mut() {
                    match &mut **last_node {
                        BnfDefinition::Series(value) => value.push(node),
                        _ => {
                            if let Some(last_node) = result.pop() {
                                result.push(Box::new(Self::Series(vec![last_node, node])))
                            }
                        }
                    }
                }
            }
            newly_definition = false;
        }

        if result.is_empty() {
            Err(())
        } else if result.len().eq(&1) {
            result.into_iter().next().ok_or(())
        } else {
            Ok(Box::new(BnfDefinition::Select(result)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::new_lexer_state;
    use super::*;

    #[test]
    fn test_definition_series() {
        let src = "rule = rule1 (rule2 / rule3) [rule4 rule5];";
        let mut lex_state = new_lexer_state();

        let _ = lex_state.next(src);
        let _ = lex_state.next(src);

        if let Ok(definition) = BnfDefinition::new(src, &mut lex_state) {
            match *definition {
                BnfDefinition::Series(rules) => {
                    assert_eq!(3, rules.len());
                }
                _ => panic!("error type"),
            }
        } else {
            panic!("error");
        }
    }

    #[test]
    fn test_definition_select() {
        let src = "rule = rule1 / rule2 / rule3 / rule4 (rule5 rule6) / rule7;";
        let mut lex_state = new_lexer_state();

        let _ = lex_state.next(src);
        let _ = lex_state.next(src);

        if let Ok(definition) = BnfDefinition::new(src, &mut lex_state) {
            match *definition {
                BnfDefinition::Select(rules) => {
                    assert_eq!(5, rules.len());
                }
                _ => panic!("error type"),
            }
        } else {
            panic!("error");
        }
    }

    #[test]
    fn test_definition_group() {
        let src = "rule = (rule1 (rule2 rule3));";
        let mut lex_state = new_lexer_state();

        let _ = lex_state.next(src);
        let _ = lex_state.next(src);

        if let Ok(definition) = BnfDefinition::new(src, &mut lex_state) {
            match *definition {
                BnfDefinition::Group(_) => {}
                _ => panic!("error type"),
            }
        } else {
            panic!("error");
        }
    }

    #[test]
    fn test_definition_options() {
        let src = "rule = [rule1 (rule2 rule3) / rule4];";
        let mut lex_state = new_lexer_state();

        let _ = lex_state.next(src);
        let _ = lex_state.next(src);

        if let Ok(definition) = BnfDefinition::new(src, &mut lex_state) {
            match *definition {
                BnfDefinition::Options(_) => {}
                _ => panic!("error type"),
            }
        } else {
            panic!("error");
        }
    }

    #[test]
    fn test_definition_terminal() {
        let src = "rule = \"terminal\";";
        let mut lex_state = new_lexer_state();

        let _ = lex_state.next(src);
        let _ = lex_state.next(src);

        if let Ok(definition) = BnfDefinition::new(src, &mut lex_state) {
            match *definition {
                BnfDefinition::Terminal(_) => {}
                _ => panic!("error type"),
            }
        } else {
            panic!("error");
        }
    }

    #[test]
    fn test_definition_range() {
        let src = "rule = %x6b-7f;";
        let mut lex_state = new_lexer_state();

        let _ = lex_state.next(src);
        let _ = lex_state.next(src);

        if let Ok(definition) = BnfDefinition::new(src, &mut lex_state) {
            match *definition {
                BnfDefinition::Range(_) => {}
                _ => panic!("error type"),
            }
        } else {
            panic!("error");
        }
    }

    #[test]
    fn test_definition_variable_rule() {
        let src = "rule = 1*2(rule1 rule2 / rule3);";
        let mut lex_state = new_lexer_state();

        let _ = lex_state.next(src);
        let _ = lex_state.next(src);

        if let Ok(definition) = BnfDefinition::new(src, &mut lex_state) {
            match *definition {
                BnfDefinition::Repeat(_) => {}
                _ => panic!("error type"),
            }
        } else {
            panic!("error");
        }
    }
}
