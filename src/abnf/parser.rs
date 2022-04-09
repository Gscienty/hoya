use super::{
    super::lex::{LexerState, Token, TokenFactory},
    BnfState,
};

pub mod abnf_type {
    pub const TOKEN_NAME_TYPE: &str = "abnf_token_name";
    pub const TOKEN_DEFINER_TYPE: &str = "abnf_token_definer";
    pub const TOKEN_TERMINAL_TYPE: &str = "abnf_token_terminal";
    pub const TOKEN_RANGE_TYPE: &str = "abnf_token_range";
    pub const TOKEN_LEFT_PARENTHESIS_TYPE: &str = "abnf_token_left_parenthesis";
    pub const TOKEN_RIGHT_PARENTHESIS_TYPE: &str = "abnf_token_right_parenthesis";
    pub const TOKEN_VARIABLE_TYPE: &str = "abnf_token_variable";
    pub const TOKEN_LEFT_OPTIONS_TYPE: &str = "abnf_token_left_options";
    pub const TOKEN_RIGHT_OPTIONS_TYPE: &str = "abnf_token_right_options";
    pub const TOKEN_SELECT_TYPE: &str = "abnf_token_select";
    pub const TOKEN_END_TYPE: &str = "abnf_token_end";
    pub const TOKEN_REQUIREMENT_TYPE: &str = "abnf_requirement";
    pub const ABNF_TOKEN_EOF: &str = "abnf_token_eof";
}

const TOKEN_NAME_REGEX: &str = r"^[a-zA-Z][a-zA-Z0-9\-]*";
const TOKEN_DEFINER_REGEX: &str = r"^=/?";
const TOKEN_TERMINAL_BINARY_REGEX: &str = r"^%b(0|1)+(\.(0|1)+)*";
const TOKEN_TERMINAL_DECIMAL_REGEX: &str = r"^%d\d+(\.\d+)*";
const TOKEN_TERMINAL_HEXADECIMAL_REGEX: &str = r"^%h[a-fA-F0-9]+(\.[a-fA-F0-9]+)*";
const TOKEN_TERMINAL_STRING_REGEX: &str = r#"^"(?:\\"|[^"])*?""#;
const TOKEN_RANGE_REGEX: &str = r"^%(b(0|1)+-(0|1)+|d\d+-\d+|x[a-fA-F0-9]+-[a-fA-F0-9]+)";
const TOKEN_LEFT_PARENTHESIS_REGEX: &str = r"^\(";
const TOKEN_RIGHT_PARENTHESIS_REGEX: &str = r"^\)";
const TOKEN_VARIABLE_REGEX: &str = r"^(\d*\*\d*|\d+)";
const TOKEN_LEFT_OPTIONS_REGEX: &str = r"^\[";
const TOKEN_RIGHT_OPTIONS_REGEX: &str = r"^\]";
const TOKEN_SELECT_REGEX: &str = r"^/";
const TOKEN_END_REGEX: &str = r"^;";
const TOKEN_REQUIREMENT_REGEX: &str = r"^<.*?>";

pub mod abnf_state {
    // ABNF_STATE_INIT
    //
    // from ABNF_STATE_ELEMENTS
    //
    // to   ABNF_STATE_DEFINER
    pub const ABNF_STATE_INIT: &str = "abnf_state_init";

    // ABNF_STATE_DEFINER
    //
    // from ABNF_STATE_INIT
    //
    // to   ABNF_REQUIRE_STATE_ELEMENTS
    pub const ABNF_STATE_DEFINER: &str = "abnf_state_definer";

    // ABNF_STATE_REQUIRE_ELEMENTS
    //
    // from ABNF_STATE_DEFINER |
    //      ABNF_STATE_ELEMENTS
    //
    // to   ABNF_STATE_ELEMENTS
    pub const ABNF_STATE_REQUIRE_ELEMENTS: &str = "abnf_state_require_elements";

    // ABNF_STATE_ELEMENTS
    //
    // from ABNF_STATE_REQUIRE_ELEMENTS |
    //      ABNF_STATE_ELEMENTS |
    //      ABNF_STATE_PARENTHESIS_ELEMENTS |
    //      ABNF_STATE_VARIABLE_REQUIRE_ELEMENT |
    //      ABNF_STATE_OPTIONS_ELEMENTS
    //
    // to   ABNF_STATE_REQUIRE_ELEMENTS |
    //      ABNF_STATE_ELEMENTS |
    //      ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS |
    //      ABNF_STATE_VARIABLE_REQUIRE_ELEMENT |
    //      ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS |
    //      ABNF_STATE_INIT
    pub const ABNF_STATE_ELEMENTS: &str = "abnf_state_elements";

    // ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
    //
    // from ABNF_STATE_ELEMENTS |
    //      ABNF_STATE_PARENTHESIS_ELEMENTS |
    //      ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS |
    //      ABNF_STATE_OPTIONS_ELEMENTS |
    //      ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS |
    //      ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
    //
    // to   ABNF_STATE_PARENTHESIS_ELEMENTS
    pub const ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS: &str =
        "abnf_state_parenthesis_require_elements";

    // ABNF_STATE_PARENTHESIS_ELEMENTS
    //
    // from ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
    //
    // to   ABNF_STATE_PARENTHESIS_ELEMENTS |
    //      ABNF_STATE_ELEMENTS |
    //      ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS |
    //      ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS |
    //      ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
    pub const ABNF_STATE_PARENTHESIS_ELEMENTS: &str = "abnf_state_parenthesis_elements";

    // ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
    //
    // from ABNF_STATE_REQUIRE_ELEMENTS |
    //      ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS |
    //      ABNF_STATE_PARENTHESIS_ELEMENTS |
    //      ABNF_STATE_ELEMENTS
    //
    // to   ABNF_STATE_ELEMENTS |
    //      ABNF_STATE_OPTIONS_ELEMENTS |
    //      ABNF_STATE_PARENTHESIS_ELEMENTS
    pub const ABNF_STATE_VARIABLE_REQUIRE_ELEMENT: &str = "abnf_state_variable_require_element";

    // ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS
    //
    // from ABNF_STATE_REQUIRE_ELEMENTS |
    //      ABNF_STATE_ELEMENTS |
    //      ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS |
    //      ABNF_STATE_PARENTHESIS_ELEMENTS
    //
    // to   ABNF_STATE_OPTIONS_ELEMENTS
    pub const ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS: &str = "abnf_state_options_require_elements";

    // ABNF_STATE_OPTIONS_ELEMENTS
    //
    // from ABNF_STATE_OPTIONS_ELEMENTS |
    //      ABNF_STATE_REQUIRE_ELEMENTS
    //
    // to   ABNF_STATE_PARENTHESIS_ELEMENTS |
    //      ABNF_STATE_ELEMENTS |
    //      ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS |
    //      ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS |
    //      ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
    pub const ABNF_STATE_OPTIONS_ELEMENTS: &str = "abnf_state_options_elements";
}

const ABNF_IGNORE_REGEX: &str = r"^( |\t)";

macro_rules! set_abnf_rules {
    (LET $lexer_state: ident $(WHEN $token_state: ident SET $(
        [$token_regex: ident] => $token_type: ident $(- $pop: literal)? $(+ $push: ident)*
    )*)*) => {{
        $($lexer_state.state(abnf_state::$token_state)
            $(.token($token_regex, |_: &mut BnfState, token: &str| {
                TokenFactory::new(abnf_type::$token_type)
                    $(.pop_state($pop))?
                    $(.push_state(abnf_state::$push))*
                    .build(token)
            }))*;)*

        $lexer_state
    }};
}

pub fn new_lexer_state() -> LexerState<BnfState> {
    let mut state = LexerState::new(abnf_state::ABNF_STATE_INIT, BnfState::new());
    state
        .set_eof(|| Token::new(abnf_type::ABNF_TOKEN_EOF, ""))
        .set_ignore(ABNF_IGNORE_REGEX);

    set_abnf_rules!(
        LET state

        WHEN ABNF_STATE_INIT
        SET
            [TOKEN_NAME_REGEX]                 => TOKEN_NAME_TYPE
                + ABNF_STATE_DEFINER

        WHEN ABNF_STATE_DEFINER
        SET
            [TOKEN_DEFINER_REGEX]              => TOKEN_DEFINER_TYPE           - 1
                + ABNF_STATE_ELEMENTS
                + ABNF_STATE_REQUIRE_ELEMENTS

        WHEN ABNF_STATE_REQUIRE_ELEMENTS
        SET
            [TOKEN_NAME_REGEX]                 => TOKEN_NAME_TYPE              - 1
            [TOKEN_TERMINAL_BINARY_REGEX]      => TOKEN_TERMINAL_TYPE          - 1
            [TOKEN_TERMINAL_DECIMAL_REGEX]     => TOKEN_TERMINAL_TYPE          - 1
            [TOKEN_TERMINAL_HEXADECIMAL_REGEX] => TOKEN_TERMINAL_TYPE          - 1
            [TOKEN_TERMINAL_STRING_REGEX]      => TOKEN_TERMINAL_TYPE          - 1
            [TOKEN_RANGE_REGEX]                => TOKEN_RANGE_TYPE             - 1
            [TOKEN_REQUIREMENT_REGEX]          => TOKEN_REQUIREMENT_TYPE       - 1
            [TOKEN_VARIABLE_REGEX]             => TOKEN_VARIABLE_TYPE          - 1
                + ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
            [TOKEN_LEFT_PARENTHESIS_REGEX]     => TOKEN_LEFT_PARENTHESIS_TYPE  - 1
                + ABNF_STATE_PARENTHESIS_ELEMENTS
                + ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
            [TOKEN_LEFT_OPTIONS_REGEX]         => TOKEN_LEFT_OPTIONS_TYPE      - 1
                + ABNF_STATE_OPTIONS_ELEMENTS
                + ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS

        WHEN ABNF_STATE_ELEMENTS
        SET
            [TOKEN_NAME_REGEX]                 => TOKEN_NAME_TYPE
            [TOKEN_TERMINAL_BINARY_REGEX]      => TOKEN_TERMINAL_TYPE
            [TOKEN_TERMINAL_DECIMAL_REGEX]     => TOKEN_TERMINAL_TYPE
            [TOKEN_TERMINAL_HEXADECIMAL_REGEX] => TOKEN_TERMINAL_TYPE
            [TOKEN_TERMINAL_STRING_REGEX]      => TOKEN_TERMINAL_TYPE
            [TOKEN_RANGE_REGEX]                => TOKEN_RANGE_TYPE
            [TOKEN_REQUIREMENT_REGEX]          => TOKEN_REQUIREMENT_TYPE
            [TOKEN_END_REGEX]                  => TOKEN_END_TYPE               - 1
            [TOKEN_SELECT_REGEX]               => TOKEN_SELECT_TYPE
                + ABNF_STATE_REQUIRE_ELEMENTS
            [TOKEN_VARIABLE_REGEX]             => TOKEN_VARIABLE_TYPE
                + ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
            [TOKEN_LEFT_PARENTHESIS_REGEX]     => TOKEN_LEFT_PARENTHESIS_TYPE
                + ABNF_STATE_PARENTHESIS_ELEMENTS
                + ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
            [TOKEN_LEFT_OPTIONS_REGEX]         => TOKEN_LEFT_OPTIONS_TYPE
                + ABNF_STATE_OPTIONS_ELEMENTS
                + ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS

        WHEN ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
        SET
            [TOKEN_NAME_REGEX]                 => TOKEN_NAME_TYPE              - 1
            [TOKEN_TERMINAL_BINARY_REGEX]      => TOKEN_TERMINAL_TYPE          - 1
            [TOKEN_TERMINAL_DECIMAL_REGEX]     => TOKEN_TERMINAL_TYPE          - 1
            [TOKEN_TERMINAL_HEXADECIMAL_REGEX] => TOKEN_TERMINAL_TYPE          - 1
            [TOKEN_TERMINAL_STRING_REGEX]      => TOKEN_TERMINAL_TYPE          - 1
            [TOKEN_RANGE_REGEX]                => TOKEN_RANGE_TYPE             - 1
            [TOKEN_REQUIREMENT_REGEX]          => TOKEN_REQUIREMENT_TYPE       - 1
            [TOKEN_VARIABLE_REGEX]             => TOKEN_VARIABLE_TYPE          - 1
                + ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
            [TOKEN_LEFT_PARENTHESIS_REGEX]     => TOKEN_LEFT_PARENTHESIS_TYPE  - 1
                + ABNF_STATE_PARENTHESIS_ELEMENTS
                + ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
            [TOKEN_LEFT_OPTIONS_REGEX]         => TOKEN_LEFT_OPTIONS_TYPE      - 1
                + ABNF_STATE_OPTIONS_ELEMENTS
                + ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS

        WHEN ABNF_STATE_PARENTHESIS_ELEMENTS
        SET
            [TOKEN_NAME_REGEX]                 => TOKEN_NAME_TYPE
            [TOKEN_TERMINAL_BINARY_REGEX]      => TOKEN_TERMINAL_TYPE
            [TOKEN_TERMINAL_DECIMAL_REGEX]     => TOKEN_TERMINAL_TYPE
            [TOKEN_TERMINAL_HEXADECIMAL_REGEX] => TOKEN_TERMINAL_TYPE
            [TOKEN_TERMINAL_STRING_REGEX]      => TOKEN_TERMINAL_TYPE
            [TOKEN_RANGE_REGEX]                => TOKEN_RANGE_TYPE
            [TOKEN_REQUIREMENT_REGEX]          => TOKEN_REQUIREMENT_TYPE
            [TOKEN_RIGHT_PARENTHESIS_REGEX]    => TOKEN_RIGHT_PARENTHESIS_TYPE - 1
            [TOKEN_VARIABLE_REGEX]             => TOKEN_VARIABLE_TYPE
                + ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
            [TOKEN_SELECT_REGEX]               => TOKEN_SELECT_TYPE
                + ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
            [TOKEN_LEFT_PARENTHESIS_REGEX]     => TOKEN_LEFT_PARENTHESIS_TYPE
                + ABNF_STATE_PARENTHESIS_ELEMENTS
                + ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
            [TOKEN_LEFT_OPTIONS_REGEX]         => TOKEN_LEFT_OPTIONS_TYPE
                + ABNF_STATE_OPTIONS_ELEMENTS
                + ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS

        WHEN ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
        SET
            [TOKEN_NAME_REGEX]                 => TOKEN_NAME_TYPE              - 1
            [TOKEN_TERMINAL_BINARY_REGEX]      => TOKEN_TERMINAL_TYPE          - 1
            [TOKEN_TERMINAL_DECIMAL_REGEX]     => TOKEN_TERMINAL_TYPE          - 1
            [TOKEN_TERMINAL_HEXADECIMAL_REGEX] => TOKEN_TERMINAL_TYPE          - 1
            [TOKEN_TERMINAL_STRING_REGEX]      => TOKEN_TERMINAL_TYPE          - 1
            [TOKEN_RANGE_REGEX]                => TOKEN_RANGE_TYPE             - 1
            [TOKEN_REQUIREMENT_REGEX]          => TOKEN_REQUIREMENT_TYPE       - 1
            [TOKEN_LEFT_PARENTHESIS_REGEX]     => TOKEN_LEFT_PARENTHESIS_TYPE  - 1
                + ABNF_STATE_PARENTHESIS_ELEMENTS
                + ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS

        WHEN ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS
        SET
            [TOKEN_NAME_REGEX]                 => TOKEN_NAME_TYPE              - 1
            [TOKEN_TERMINAL_BINARY_REGEX]      => TOKEN_TERMINAL_TYPE          - 1
            [TOKEN_TERMINAL_DECIMAL_REGEX]     => TOKEN_TERMINAL_TYPE          - 1
            [TOKEN_TERMINAL_HEXADECIMAL_REGEX] => TOKEN_TERMINAL_TYPE          - 1
            [TOKEN_TERMINAL_STRING_REGEX]      => TOKEN_TERMINAL_TYPE          - 1
            [TOKEN_RANGE_REGEX]                => TOKEN_RANGE_TYPE             - 1
            [TOKEN_REQUIREMENT_REGEX]          => TOKEN_REQUIREMENT_TYPE       - 1
            [TOKEN_VARIABLE_REGEX]             => TOKEN_VARIABLE_TYPE          - 1
                + ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
            [TOKEN_LEFT_PARENTHESIS_REGEX]     => TOKEN_LEFT_PARENTHESIS_TYPE  - 1
                + ABNF_STATE_PARENTHESIS_ELEMENTS
                + ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
            [TOKEN_LEFT_OPTIONS_REGEX]         => TOKEN_LEFT_OPTIONS_TYPE      - 1
                + ABNF_STATE_OPTIONS_ELEMENTS
                + ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS

        WHEN ABNF_STATE_OPTIONS_ELEMENTS
        SET
            [TOKEN_NAME_REGEX]                 => TOKEN_NAME_TYPE
            [TOKEN_TERMINAL_BINARY_REGEX]      => TOKEN_TERMINAL_TYPE
            [TOKEN_TERMINAL_DECIMAL_REGEX]     => TOKEN_TERMINAL_TYPE
            [TOKEN_TERMINAL_HEXADECIMAL_REGEX] => TOKEN_TERMINAL_TYPE
            [TOKEN_TERMINAL_STRING_REGEX]      => TOKEN_TERMINAL_TYPE
            [TOKEN_RANGE_REGEX]                => TOKEN_RANGE_TYPE
            [TOKEN_REQUIREMENT_REGEX]          => TOKEN_REQUIREMENT_TYPE
            [TOKEN_RIGHT_OPTIONS_REGEX]        => TOKEN_RIGHT_OPTIONS_TYPE     - 1
            [TOKEN_SELECT_REGEX]               => TOKEN_SELECT_TYPE
                + ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS
            [TOKEN_VARIABLE_REGEX]             => TOKEN_VARIABLE_TYPE
                + ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
            [TOKEN_LEFT_PARENTHESIS_REGEX]     => TOKEN_LEFT_PARENTHESIS_TYPE
                + ABNF_STATE_PARENTHESIS_ELEMENTS
                + ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
            [TOKEN_LEFT_OPTIONS_REGEX]         => TOKEN_LEFT_OPTIONS_TYPE
                + ABNF_STATE_OPTIONS_ELEMENTS
                + ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_token(src: &str, tokens: Vec<(&str, &str)>) {
        let mut lex_state = new_lexer_state();

        for (token_type, token_value) in tokens {
            if let Ok(token) = lex_state.next(src) {
                assert_eq!(token_type, token.get_type());
                assert_eq!(token_value, token.get_value());
            } else {
                panic!("error");
            }
        }
    }

    #[test]
    fn test_simple_abnf() {
        assert_token(
            "token = name1 name2 \"name3\";",
            vec![
                (abnf_type::TOKEN_NAME_TYPE, "token"),
                (abnf_type::TOKEN_DEFINER_TYPE, "="),
                (abnf_type::TOKEN_NAME_TYPE, "name1"),
                (abnf_type::TOKEN_NAME_TYPE, "name2"),
                (abnf_type::TOKEN_TERMINAL_TYPE, "\"name3\""),
                (abnf_type::TOKEN_END_TYPE, ";"),
            ],
        );
    }

    #[test]
    fn test_terminal_abnf() {
        assert_token(
            "token = \"\\\"foo\\\"\" \"bar\";",
            vec![
                (abnf_type::TOKEN_NAME_TYPE, "token"),
                (abnf_type::TOKEN_DEFINER_TYPE, "="),
                (abnf_type::TOKEN_TERMINAL_TYPE, "\"\\\"foo\\\"\""),
                (abnf_type::TOKEN_TERMINAL_TYPE, "\"bar\""),
                (abnf_type::TOKEN_END_TYPE, ";"),
            ],
        );
    }

    #[test]
    fn test_parenthesis_abnf() {
        assert_token(
            "token = (name1 (\"terminal string\"))name2((name3)name4);",
            vec![
                (abnf_type::TOKEN_NAME_TYPE, "token"),
                (abnf_type::TOKEN_DEFINER_TYPE, "="),
                (abnf_type::TOKEN_LEFT_PARENTHESIS_TYPE, "("),
                (abnf_type::TOKEN_NAME_TYPE, "name1"),
                (abnf_type::TOKEN_LEFT_PARENTHESIS_TYPE, "("),
                (abnf_type::TOKEN_TERMINAL_TYPE, "\"terminal string\""),
                (abnf_type::TOKEN_RIGHT_PARENTHESIS_TYPE, ")"),
                (abnf_type::TOKEN_RIGHT_PARENTHESIS_TYPE, ")"),
                (abnf_type::TOKEN_NAME_TYPE, "name2"),
                (abnf_type::TOKEN_LEFT_PARENTHESIS_TYPE, "("),
                (abnf_type::TOKEN_LEFT_PARENTHESIS_TYPE, "("),
                (abnf_type::TOKEN_NAME_TYPE, "name3"),
                (abnf_type::TOKEN_RIGHT_PARENTHESIS_TYPE, ")"),
                (abnf_type::TOKEN_NAME_TYPE, "name4"),
                (abnf_type::TOKEN_RIGHT_PARENTHESIS_TYPE, ")"),
                (abnf_type::TOKEN_END_TYPE, ";"),
            ],
        )
    }

    #[test]
    fn test_options_abnf() {
        assert_token(
            "token = [option-token1 \";\" option-token2 (parenthesis [option-token3])];",
            vec![
                (abnf_type::TOKEN_NAME_TYPE, "token"),
                (abnf_type::TOKEN_DEFINER_TYPE, "="),
                (abnf_type::TOKEN_LEFT_OPTIONS_TYPE, "["),
                (abnf_type::TOKEN_NAME_TYPE, "option-token1"),
                (abnf_type::TOKEN_TERMINAL_TYPE, "\";\""),
                (abnf_type::TOKEN_NAME_TYPE, "option-token2"),
                (abnf_type::TOKEN_LEFT_PARENTHESIS_TYPE, "("),
                (abnf_type::TOKEN_NAME_TYPE, "parenthesis"),
                (abnf_type::TOKEN_LEFT_OPTIONS_TYPE, "["),
                (abnf_type::TOKEN_NAME_TYPE, "option-token3"),
                (abnf_type::TOKEN_RIGHT_OPTIONS_TYPE, "]"),
                (abnf_type::TOKEN_RIGHT_PARENTHESIS_TYPE, ")"),
                (abnf_type::TOKEN_RIGHT_OPTIONS_TYPE, "]"),
            ],
        )
    }

    #[test]
    fn test_select_abnf() {
        assert_token(
            "token = choose-1 / choose-2 / choose-3;",
            vec![
                (abnf_type::TOKEN_NAME_TYPE, "token"),
                (abnf_type::TOKEN_DEFINER_TYPE, "="),
                (abnf_type::TOKEN_NAME_TYPE, "choose-1"),
                (abnf_type::TOKEN_SELECT_TYPE, "/"),
                (abnf_type::TOKEN_NAME_TYPE, "choose-2"),
                (abnf_type::TOKEN_SELECT_TYPE, "/"),
                (abnf_type::TOKEN_NAME_TYPE, "choose-3"),
                (abnf_type::TOKEN_END_TYPE, ";"),
            ],
        )
    }
}
