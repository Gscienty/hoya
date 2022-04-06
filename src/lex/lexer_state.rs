/*
 * 词法解析器
 *
 * 本文件主要实现两个结构：LexerState 和 LexerStateSnapshot。
 *
 * LexerState 用于记录词法解析过程中的状态，它将作为词法解
 * 析过程中实现状态转换的重要模块。在LexerState中，将会记录
 * 词法解析的文本指针、并且记录了词法解析的所有状态，每个状
 * 态中的Token生成规则。
 *
 * LexerStateSnapshot 用于存储 LexerState 的快照。
 *
 */

use super::{NextStateChange, Position, StateChange, Token};
use regex::Regex;
use std::collections::HashMap;

pub struct LexerTokenSnapshot {
    state_stack: Vec<&'static str>,
    current_state: &'static str,
}

impl LexerTokenSnapshot {
    pub fn new(state_stack: Vec<&'static str>, current_state: &'static str) -> Self {
        LexerTokenSnapshot {
            state_stack,
            current_state,
        }
    }

    pub fn get_state_stack(&self) -> Vec<&'static str> {
        self.state_stack.clone()
    }

    pub fn get_current_state(&self) -> &'static str {
        self.current_state
    }
}

// 词法解析器快照
pub struct LexerStateSnapshot {
    offset: usize,
    line: usize,
    line_offset: usize,
    token_snapshot: LexerTokenSnapshot,
}

impl LexerStateSnapshot {
    pub fn new(
        offset: usize,
        line: usize,
        line_offset: usize,
        token_snapshot: LexerTokenSnapshot,
    ) -> Self {
        LexerStateSnapshot {
            offset,
            line,
            line_offset,
            token_snapshot,
        }
    }

    pub fn get_offset(&self) -> usize {
        self.offset
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_line_offset(&self) -> usize {
        self.line_offset
    }

    pub fn get_token_snapshot(&self) -> &LexerTokenSnapshot {
        &self.token_snapshot
    }
}

type TokenFactoryFunc<S> = fn(&mut S, &str) -> (Token, StateChange);

struct LexerTokenFactory<S> {
    state_stack: Vec<&'static str>,
    current_state: &'static str,
    token_factory: HashMap<&'static str, Vec<(Regex, TokenFactoryFunc<S>)>>,
}

impl<S> LexerTokenFactory<S> {
    pub fn new(initial_status: &'static str) -> Self {
        LexerTokenFactory {
            state_stack: Vec::new(),
            current_state: initial_status,
            token_factory: HashMap::new(),
        }
    }

    pub fn reset(&mut self, initial_status: &'static str) {
        self.state_stack.clear();
        self.current_state = initial_status;
        self.token_factory.clear();
    }

    fn set(&mut self, state: &'static str) -> Option<&mut Vec<(Regex, TokenFactoryFunc<S>)>> {
        if !self.token_factory.contains_key(state) {
            self.token_factory.insert(state, Vec::new());
        }

        self.token_factory.get_mut(state)
    }

    pub fn add(&mut self, state: &'static str, re: &str, token: TokenFactoryFunc<S>) {
        self.set(state)
            .and_then(|tokens| Some(Regex::new(re).and_then(|re| Ok(tokens.push((re, token))))));
    }

    fn push(&mut self, state: &'static str) {
        self.state_stack.push(self.current_state);
        self.current_state = state;
    }

    fn pop(&mut self, times: usize) {
        (0..times).for_each(|_| {
            self.state_stack
                .pop()
                .and_then(|state| Some(self.current_state = state));
        });
    }

    pub fn change_state(&mut self, next_state: &NextStateChange) {
        match next_state {
            &NextStateChange::Push(state) => self.push(state),
            &NextStateChange::Pop(times) => self.pop(times),
        }
    }

    pub fn get<'t>(&mut self, src: &'t str) -> Option<(&'t str, &TokenFactoryFunc<S>)> {
        self.token_factory
            .get(self.current_state)
            .and_then(|tokens| {
                tokens.iter().find_map(|(re, factory)| {
                    re.find(src)
                        .and_then(|matched| Some((matched.as_str(), factory)))
                })
            })
    }

    pub fn dump(&self) -> LexerTokenSnapshot {
        LexerTokenSnapshot::new(self.state_stack.clone(), self.current_state)
    }

    pub fn restore(&mut self, snapshot: &LexerTokenSnapshot) {
        self.state_stack = snapshot.get_state_stack();
        self.current_state = snapshot.get_current_state();
    }
}

// 词法解析器
pub struct LexerState<S> {
    custom_state: S,

    offset: usize,
    line: usize,
    line_offset: usize,

    ignore_regex: Option<Regex>,
    eof: Option<fn() -> Token>,
    is_eof: bool,

    token_factory: LexerTokenFactory<S>,
}

impl<S> LexerState<S> {
    // 构造一个词法解析器
    pub fn new(initial_status: &'static str, custom_state: S) -> Self {
        LexerState {
            custom_state,

            offset: 0,
            line: 1,
            line_offset: 0,

            ignore_regex: None,
            eof: None,
            is_eof: false,

            token_factory: LexerTokenFactory::new(initial_status),
        }
    }

    // 重置词法解析器
    pub fn reset(&mut self, initial_status: &'static str, custom_state: S) {
        self.custom_state = custom_state;

        self.offset = 0;
        self.line = 1;
        self.line_offset = 0;

        self.eof = None;
        self.is_eof = false;

        self.token_factory.reset(initial_status);
    }

    pub fn get_custom_state(&self) -> &S {
        &self.custom_state
    }

    pub fn get_mut_custom_state(&mut self) -> &mut S {
        &mut self.custom_state
    }

    // 对当前 LexerState 状态进行快照存储
    pub fn dump(&self) -> LexerStateSnapshot {
        LexerStateSnapshot::new(
            self.offset,
            self.line,
            self.line_offset,
            self.token_factory.dump(),
        )
    }

    // 还原 LexerState 状态
    pub fn restore(&mut self, snapshot: LexerStateSnapshot) {
        self.offset = snapshot.get_offset();
        self.line = snapshot.get_line();
        self.line_offset = snapshot.get_line_offset();
        self.token_factory.restore(snapshot.get_token_snapshot());
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
        lexer_token: TokenFactoryFunc<S>,
    ) -> &mut Self {
        self.token_factory.add(state, re, lexer_token);

        self
    }

    fn state_change(&mut self, change: &StateChange) {
        self.offset += change.get_increment_offset();

        if change.get_increment_lines().ne(&0) {
            self.line += change.get_increment_lines();
            self.line_offset = self.offset;
        }

        change
            .get_next_state()
            .iter()
            .for_each(|state| self.token_factory.change_state(state));
    }

    fn get_current_position(&self) -> Position {
        Position::new(self.line, self.offset - self.line_offset)
    }

    fn next_token(&mut self, src: &str) -> Option<(Token, StateChange)> {
        src.get(self.offset..).and_then(|src| {
            let custom_state = &mut self.custom_state;

            self.token_factory
                .get(src)
                .and_then(|(token, factory)| Some(factory(custom_state, token)))
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
