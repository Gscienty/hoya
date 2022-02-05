use super::{IncrementOffset, Position, Token};
use regex::Regex;

type LexMatchedFunc = fn(&str) -> (Token, IncrementOffset);

// 词法解析器
pub struct Lexer {
    offset: usize,
    line: usize,
    line_offset: usize,

    ignore_regex: Option<Regex>,
    lex_regex: Vec<(Regex, LexMatchedFunc)>,
    eof: Option<fn() -> Token>,
}

impl Lexer {
    // 构造一个词法解析器
    pub fn new() -> Self {
        Lexer {
            offset: 0,
            line: 1,
            line_offset: 0,
            ignore_regex: None,
            lex_regex: Vec::new(),
            eof: None,
        }
    }

    // 重置词法解析器
    pub fn reset(&mut self) {
        self.offset = 0;
        self.line = 1;
        self.line_offset = 0;

        self.ignore_regex = None;
        self.lex_regex.clear();
        self.eof = None;
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

    // 设定截止Token
    // eof: 构造截止Token的方法
    pub fn set_eof(&mut self, eof: fn() -> Token) -> &mut Self {
        self.eof = Some(eof);

        self
    }

    // 添加Token
    // re: Token正则表达式
    // token_factory: 构造Token的方法
    pub fn add_token(&mut self, re: &str, token_factory: LexMatchedFunc) -> &mut Self {
        if let Ok(reg) = Regex::new(re) {
            self.lex_regex.push((reg, token_factory));
        }

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
            self.lex_regex.iter().find_map(|(re, token_factory)| {
                re.find(src)
                    .and_then(|matched| Some(token_factory(matched.as_str())))
            })
        })
    }

    // 从源码中获取一个Token
    // src: 源码
    pub fn next(&mut self, src: &str) -> Result<Token, ()> {
        while self.offset.lt(&src.len()) {
            // 跳过被忽略的字符
            if self.skip_ignore(src) {
                break;
            }

            let begin_position = self.get_current_position();
            return self
                .next_token(src)
                .and_then(|(mut token, increment_offset)| {
                    self.increment_offset(increment_offset);
                    token.set_location(begin_position, self.get_current_position());
                    Some(token)
                })
                .ok_or(());
        }

        self.eof.and_then(|eof| Some(eof())).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_calc() {
        let mut lexer = Lexer::new();
        lexer
            .set_ignore(r"^( |\t)")
            .set_eof(|| Token::new("eof", ""))
            .add_token(r"^(=|\+|\*|/|\^|%|\(|\))", |token_slice| {
                (
                    Token::new("literal", token_slice),
                    IncrementOffset::new(1, 0),
                )
            })
            .add_token(r"^[a-zA-Z_][a-zA-Z0-9_]*", |token_slice| {
                (
                    Token::new("name", token_slice),
                    IncrementOffset::new(token_slice.len(), 0),
                )
            })
            .add_token(r"^\d+", |token_slice| {
                (
                    Token::new("number", token_slice),
                    IncrementOffset::new(token_slice.len(), 0),
                )
            });

        let src = "variable = 123 + 234 * 345 / 456 ^ (567 % 678) + variable_2";

        let expect = vec![
            ("name", "variable"),
            ("literal", "="),
            ("number", "123"),
            ("literal", "+"),
            ("number", "234"),
            ("literal", "*"),
            ("number", "345"),
            ("literal", "/"),
            ("number", "456"),
            ("literal", "^"),
            ("literal", "("),
            ("number", "567"),
            ("literal", "%"),
            ("number", "678"),
            ("literal", ")"),
            ("literal", "+"),
            ("name", "variable_2"),
            ("eof", ""),
        ];

        for (expect_type, expect_value) in expect {
            if let Ok(token) = lexer.next(src) {
                assert_eq!(expect_type, token.get_type());
                assert_eq!(expect_value, token.get_value());
            } else {
                panic!("failed unknow")
            }
        }
    }

    #[test]
    fn lex_calc2() {
        let mut lexer = Lexer::new();

        lexer
            .set_ignore(r"^( |\t)")
            .set_eof(|| Token::new("eof", ""))
            .add_token(r"^\+=?", |token_slice| {
                (
                    Token::new("literal", token_slice),
                    IncrementOffset::new(token_slice.len(), 0),
                )
            })
            .add_token(r"^[a-zA-Z_][a-zA-Z0-9_]*", |token_slice| {
                (
                    Token::new("name", token_slice),
                    IncrementOffset::new(token_slice.len(), 0),
                )
            })
            .add_token(r"^\d+", |token_slice| {
                (
                    Token::new("number", token_slice),
                    IncrementOffset::new(token_slice.len(), 0),
                )
            })
            .add_token(r"^\n", |_| {
                (Token::new("new_line", ""), IncrementOffset::new(1, 1))
            });

        let src = "
    var1 += var2 + var_3 + 4 + 5
    + _var__4

";

        let expect = vec![
            ("new_line", ""),
            ("name", "var1"),
            ("literal", "+="),
            ("name", "var2"),
            ("literal", "+"),
            ("name", "var_3"),
            ("literal", "+"),
            ("number", "4"),
            ("literal", "+"),
            ("number", "5"),
            ("new_line", ""),
            ("literal", "+"),
            ("name", "_var__4"),
            ("new_line", ""),
            ("new_line", ""),
            ("eof", ""),
        ];

        for (expect_type, expect_value) in expect {
            if let Ok(token) = lexer.next(src) {
                assert_eq!(expect_type, token.get_type());
                assert_eq!(expect_value, token.get_value());
            } else {
                panic!("failed unknow");
            }
        }
    }
}
