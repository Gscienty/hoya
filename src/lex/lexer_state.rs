use super::{IncrementOffset, Position, Token};
use regex::Regex;

// 词法解析器
pub struct LexerState<S> {
    custom_state: S,
    offset: usize,
    line: usize,
    line_offset: usize,

    ignore_regex: Option<Regex>,
    lexer_tokens: Vec<(Regex, fn(&mut S, &str) -> (Token, IncrementOffset))>,
    eof: Option<fn() -> Token>,
    is_eof: bool,
}

impl<S> LexerState<S> {
    // 构造一个词法解析器
    pub fn new(custom_state: S) -> Self {
        LexerState {
            custom_state,
            offset: 0,
            line: 1,
            line_offset: 0,

            ignore_regex: None,
            lexer_tokens: Vec::new(),
            eof: None,
            is_eof: false,
        }
    }

    // 重置词法解析器
    pub fn reset(&mut self) {
        self.offset = 0;
        self.line = 1;
        self.line_offset = 0;

        self.ignore_regex = None;
        self.lexer_tokens.clear();
        self.eof = None;
        self.is_eof = false;
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
        re: &str,
        lexer_token: fn(&mut S, &str) -> (Token, IncrementOffset),
    ) -> &mut Self {
        Regex::new(re)
            .and_then(|re| Ok(self.lexer_tokens.push((re, lexer_token))))
            .ok();

        self
    }

    fn increment_offset(&mut self, increment_offset: IncrementOffset) {
        self.offset += increment_offset.get_increment_offset();

        if increment_offset.get_increment_lines().ne(&0) {
            self.line += increment_offset.get_increment_lines();
            self.line_offset = self.offset;
        }
    }

    fn get_current_position(&self) -> Position {
        Position::new(self.line, self.offset - self.line_offset)
    }

    fn next_token(&mut self, src: &str) -> Option<(Token, IncrementOffset)> {
        src.get(self.offset..).and_then(|src| {
            if let Some((matched, token_factory)) =
                self.lexer_tokens.iter().find_map(|(re, token_factory)| {
                    re.find(src)
                        .and_then(|matched| Some((matched.as_str(), token_factory)))
                })
            {
                Some(token_factory(&mut self.custom_state, matched))
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
                .and_then(|(mut token, increment_offset)| {
                    self.increment_offset(increment_offset);
                    token.set_location(begin_position, self.get_current_position());

                    Some(token)
                })
                .ok_or(())
        }
    }
}

pub fn new_token(
    token_type: &str,
    token_value: &str,
    increment_offset: usize,
    increment_lines: usize,
) -> (Token, IncrementOffset) {
    (
        Token::new(token_type, token_value),
        IncrementOffset::new(increment_offset, increment_lines),
    )
}

pub fn new_simple_token(token_type: &str, token_value: &str) -> (Token, IncrementOffset) {
    new_token(token_type, token_value, token_value.len(), 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_calc() {
        const NUMBER_TOKEN: &str = r"^\d+";
        const NAME_TOKEN: &str = r"^[a-zA-Z_][a-zA-Z0-9_]*";
        const LITERAL_TOKEN: &str = r"^(\+?=|\+|;)";

        let mut state = LexerState::new(());
        state
            .set_eof(|| Token::new("eof", ""))
            .set_ignore(r"^( |\t)")
            .add_token(NUMBER_TOKEN, |_, token| new_simple_token("number", token))
            .add_token(NAME_TOKEN, |_, token| new_simple_token("name", token))
            .add_token(LITERAL_TOKEN, |_, token| new_simple_token("literal", token));

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
}
