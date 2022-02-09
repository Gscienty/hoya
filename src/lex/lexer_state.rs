use super::{NextStateChange, Position, StateChange, Token};
use regex::Regex;
use std::collections::HashMap;

// 词法解析器
pub struct LexerState<S> {
    custom_state: S,
    offset: usize,
    line: usize,
    line_offset: usize,

    state_stack: Vec<&'static str>,
    current_state: &'static str,
    ignore_regex: Option<Regex>,
    lexer_tokens: HashMap<&'static str, Vec<(Regex, fn(&mut S, &str) -> (Token, StateChange))>>,
    eof: Option<fn() -> Token>,
    is_eof: bool,
}

impl<S> LexerState<S> {
    // 构造一个词法解析器
    pub fn new(initial_status: &'static str, custom_state: S) -> Self {
        LexerState {
            custom_state,
            offset: 0,
            line: 1,
            line_offset: 0,

            state_stack: Vec::new(),
            current_state: initial_status,
            ignore_regex: None,
            lexer_tokens: HashMap::new(),
            eof: None,
            is_eof: false,
        }
    }

    // 重置词法解析器
    pub fn reset(&mut self, initial_status: &'static str, custom_state: S) {
        self.reset_src(initial_status, custom_state);

        self.ignore_regex = None;
        self.lexer_tokens.clear();
        self.eof = None;
        self.is_eof = false;
    }

    pub fn reset_src(&mut self, initial_status: &'static str, custom_state: S) {
        self.state_stack = Vec::new();
        self.current_state = initial_status;
        self.custom_state = custom_state;
        self.offset = 0;
        self.line = 1;
        self.line_offset = 0;
    }

    // 跳过忽略字符，返回offset > src.len()
    // src: 源码
    fn skip_ignore(&mut self, src: &str) -> bool {
        loop {
            if let Some(offset_increment) = self.ignore_regex.as_ref().and_then(|re| {
                src.get(self.offset..).and_then(|offset_src| {
                    re.find(offset_src)
                        .and_then(|matched| Some(matched.end() - matched.start()))
                })
            }) {
                self.offset += offset_increment;
            } else {
                break self.offset.ge(&src.len());
            }
        }
    }

    // 设定忽略字符
    // re: 忽略字符正则表达式
    pub fn set_ignore(&mut self, re: &str) -> &mut Self {
        if let Ok(reg) = Regex::new(re) {
            self.ignore_regex = Some(reg);
        }

        self
    }

    // 设定终止Token
    // eof: 构造终止Token的方法
    pub fn set_eof(&mut self, eof: fn() -> Token) -> &mut Self {
        self.eof = Some(eof);

        self
    }

    // 添加Token
    // re: Token正则表达式
    // token_factory: 构造Token的方法
    pub fn add_token(
        &mut self,
        state: &'static str,
        re: &str,
        lexer_token: fn(&mut S, &str) -> (Token, StateChange),
    ) -> &mut Self {
        if !self.lexer_tokens.contains_key(state) {
            self.lexer_tokens.insert(state, Vec::new());
        }

        Regex::new(re)
            .or_else(|_| Err(()))
            .and_then(|re| {
                self.lexer_tokens
                    .get_mut(state)
                    .and_then(|lexer_tokens| Some(lexer_tokens.push((re, lexer_token))))
                    .ok_or(())
            })
            .ok();

        self
    }

    fn state_change(&mut self, change: &StateChange) {
        self.offset += change.get_increment_offset();

        if change.get_increment_lines().ne(&0) {
            self.line += change.get_increment_lines();
            self.line_offset = self.offset;
        }
        change.get_next_state().and_then(|state| {
            Some(match state {
                NextStateChange::Push(state) => {
                    self.state_stack.push(self.current_state);
                    self.current_state = state;
                }
                NextStateChange::Pop(times) => {
                    for _ in 0..times {
                        self.state_stack
                            .pop()
                            .and_then(|state| Some(self.current_state = state));
                    }
                }
            })
        });
    }

    fn get_current_position(&self) -> Position {
        Position::new(self.line, self.offset - self.line_offset)
    }

    fn next_token(&mut self, src: &str) -> Option<(Token, StateChange)> {
        src.get(self.offset..).and_then(|src| {
            if let Some((token, factory)) =
                self.lexer_tokens
                    .get(self.current_state)
                    .and_then(|tokens| {
                        tokens.iter().find_map(|(re, factory)| {
                            re.find(src)
                                .and_then(|matched| Some((matched.as_str(), factory)))
                        })
                    })
            {
                Some(factory(&mut self.custom_state, token))
            } else {
                None
            }
        })
    }

    fn next_eof(&mut self) -> Result<Token, ()> {
        self.is_eof = true;

        self.eof.and_then(|eof| Some(eof())).ok_or(())
    }

    // 从源码中获取一个Token
    // src: 源码
    pub fn next(&mut self, src: &str) -> Result<Token, ()> {
        if self.is_eof || self.offset.ge(&src.len()) || self.skip_ignore(src) {
            self.next_eof()
        } else {
            let begin_position = self.get_current_position();

            self.next_token(src)
                .and_then(|(mut token, state_change)| {
                    self.state_change(&state_change);

                    token.set_location(begin_position, self.get_current_position());

                    Some(token)
                })
                .ok_or(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::TokenFactory;
    use super::*;

    #[test]
    fn parse_calc() {
        const INITIAL_STATE: &str = "INITIAL";

        const NUMBER_TOKEN: &str = r"^\d+";
        const NUMBER_TYPE: &str = "number";
        const NAME_TOKEN: &str = r"^[a-zA-Z_][a-zA-Z0-9_]*";
        const NAME_TYPE: &str = "name";
        const LITERAL_TOKEN: &str = r"^(\+?=|\+|;)";
        const LITERAL_TYPE: &str = "literal";

        let mut state = LexerState::new(INITIAL_STATE, ());
        state
            .set_eof(|| Token::new("eof", ""))
            .set_ignore(r"^( |\t)")
            .add_token(INITIAL_STATE, NUMBER_TOKEN, |_, token| {
                TokenFactory::new(NUMBER_TYPE).build(token)
            })
            .add_token(INITIAL_STATE, NAME_TOKEN, |_, token| {
                TokenFactory::new(NAME_TYPE).build(token)
            })
            .add_token(INITIAL_STATE, LITERAL_TOKEN, |_, token| {
                TokenFactory::new(LITERAL_TYPE).build(token)
            });

        let src = "var += a + b +\t123 \t\t +456; var = 789";
        for (token_type, token_value) in vec![
            ("name", "var"),
            ("literal", "+="),
            ("name", "a"),
            ("literal", "+"),
            ("name", "b"),
            ("literal", "+"),
            ("number", "123"),
            ("literal", "+"),
            ("number", "456"),
            ("literal", ";"),
            ("name", "var"),
            ("literal", "="),
            ("number", "789"),
            ("eof", ""),
            ("eof", ""),
            ("eof", ""),
        ] {
            if let Ok(token) = state.next(src) {
                assert_eq!(token_type, token.get_type());
                assert_eq!(token_value, token.get_value());
            } else {
                panic!("error")
            }
        }
    }

    #[test]
    fn parse_increment() {
        let mut state = LexerState::new("init", ());

        state
            .set_eof(|| Token::new("eof", ""))
            .set_ignore(r"^( |\t)");

        state
            .add_token("init", r"^[a-z]+", |_, token| {
                TokenFactory::new("name")
                    .push_state("require_literal")
                    .build(token)
            })
            .add_token("init", r"^\+{2}", |_, token| {
                TokenFactory::new("literal")
                    .push_state("require_increment_name")
                    .build(token)
            });

        state.add_token("require_literal", r"^\+{1,2}", |_, token| {
            if token.len().eq(&1) {
                TokenFactory::new("literal").pop_state(2).build("+")
            } else {
                TokenFactory::new("literal")
                    .push_state("name_increment_after")
                    .build("++")
            }
        });

        state.add_token("require_increment_name", r"^[a-z]+", |_, token| {
            TokenFactory::new("name")
                .push_state("increment_name_after")
                .build(token)
        });

        state.add_token("increment_name_after", r"^\+", |_, token| {
            TokenFactory::new("literal").pop_state(2).build(token)
        });

        state.add_token("name_increment_after", r"^\+", |_, token| {
            TokenFactory::new("literal").pop_state(3).build(token)
        });

        let src = "++k+i+++++j";
        for (token_type, token_value) in vec![
            ("literal", "++"), // init -> require_increment_name
            ("name", "k"),     // require_increment_name -> increment_name_after
            ("literal", "+"),  // increment_name_after -> init
            ("name", "i"),     // init -> require_literal
            ("literal", "++"), // require_literal -> name_increment_after
            ("literal", "+"),  // name_increment_after -> init
            ("literal", "++"),
            ("name", "j"),
            ("eof", ""),
            ("eof", ""),
        ] {
            if let Ok(token) = state.next(src) {
                assert_eq!(token_type, token.get_type());
                assert_eq!(token_value, token.get_value());
            } else {
                panic!("error");
            }
        }
    }
}
