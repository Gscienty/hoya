use super::{
    super::lex::{LexerState, StateChange, Token, TokenFactory},
    BnfState,
};

const TOKEN_NAME_TYPE: &str = "abnf_token_name";
const TOKEN_DEFINER_TYPE: &str = "abnf_token_definer";
const TOKEN_TERMINIAL_TYPE: &str = "abnf_token_terminal";
const TOKEN_RANGE_TYPE: &str = "abnf_token_range";
const TOKEN_LEFT_PARENTHESIS_TYPE: &str = "abnf_token_left_parenthesis";
const TOKEN_RIGHT_PARENTHESIS_TYPE: &str = "abnf_token_right_parenthesis";
const TOKEN_VARIABLE_TYPE: &str = "abnf_token_variable";
const TOKEN_LEFT_OPTIONS_TYPE: &str = "abnf_token_left_options";
const TOKEN_RIGHT_OPTIONS_TYPE: &str = "abnf_token_right_options";

const TOKEN_NAME_REGEX: &str = r"^[a-zA-Z][a-zA-Z0-9\-]*";
const TOKEN_DEFINER_REGEX: &str = r"^=/?";
const TOKEN_TERMINAL_BINARY_REGEX: &str = r"^%b(0|1)+(\.(0|1)+)*";
const TOKEN_TERMINAL_DECIMAL_REGEX: &str = r"^%d\d+(\.\d+)*";
const TOKEN_TERMINAL_HEXADECIMAL_REGEX: &str = r"^%h[a-fA-F0-9]+(\.[a-fA-F0-9]+)*";
const TOKEN_TERMINAL_STRING_REGEX: &str = r#""(:?\"|[^"])*""#;
const TOKEN_RANGE_REGEX: &str = r"^b(0|1)+-(0|1)|d\d+-\d+|x[a-fA-F0-9]+-[a-fA-F0-9]+";
const TOKEN_LEFT_PARENTHESIS_REGEX: &str = r"^\(";
const TOKEN_RIGHT_PARENTHESIS_REGEX: &str = r"^\)";
const TOKEN_VARIABLE_REGEX: &str = r"^\d*\*\d*|\d+";
const TOKEN_LEFT_OPTIONS_REGEX: &str = r"^\[";
const TOKEN_RIGHT_OPTIONS_REGEX: &str = r"^\]";

// ABNF_STATE_INIT
// from ABNF_STATE_ELEMENTS
// to   ABNF_STATE_DEFINER
const ABNF_STATE_INIT: &str = "abnf_state_init";

// ABNF_STATE_DEFINER
// from ABNF_STATE_INIT
// to   ABNF_REQUIRE_STATE_ELEMENTS
const ABNF_STATE_DEFINER: &str = "abnf_state_definer";

// ABNF_STATE_REQUIRE_ELEMENTS
// from ABNF_STATE_DEFINER | ABNF_STATE_CHOOSE
// to ABNF_STATE_ELEMENTS
const ABNF_STATE_REQUIRE_ELEMENTS: &str = "abnf_state_require_elements";

// ABNF_STATE_ELEMENTS
// from ABNF_STATE_REQUIRE_ELEMENTS | ABNF_STATE_CHOOSE
const ABNF_STATE_ELEMENTS: &str = "abnf_state_elements";

// ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
// from BNF_STATE_ELEMENTS | ABNF_STATE_PARENTHESIS
// to   ABNF_STATE_PARENTHESIS_ELEMENTS
const ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS: &str = "abnf_state_parenthesis_require_elements";

// ABNF_STATE_PARENTHESIS_ELEMENTS
// from ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
const ABNF_STATE_PARENTHESIS_ELEMENTS: &str = "abnf_state_parenthesis_elements";

// ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
// from ABNF_STATE_REQUIRE_ELEMENTS |ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS |
// ABNF_STATE_PARENTHESIS_ELEMENTS | ABNF_STATE_ELEMENTS
const ABNF_STATE_VARIABLE_REQUIRE_ELEMENT: &str = "abnf_state_variable_require_element";

// ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS
// from
// to
const ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS: &str = "abnf_state_options_require_elements";

struct ABNFToken {
    name: &'static str,
    re: &'static str,
    factory: fn(&mut BnfState, token: &str) -> (Token, StateChange),
}

const ABNF_INIT_TOKEN_NAME: ABNFToken = ABNFToken {
    name: TOKEN_NAME_TYPE,
    re: TOKEN_NAME_REGEX,
    factory: |_, token| {
        TokenFactory::new(ABNF_INIT_TOKEN_NAME.name)
            .push_state(ABNF_STATE_DEFINER)
            .build(token)
    },
};

const ABNF_TOKEN_DEFINER: ABNFToken = ABNFToken {
    name: TOKEN_DEFINER_TYPE,
    re: TOKEN_DEFINER_REGEX,
    factory: |_, token| {
        TokenFactory::new(ABNF_TOKEN_DEFINER.name)
            .push_state(ABNF_STATE_REQUIRE_ELEMENTS)
            .build(token)
    },
};

const ABNF_REQUIRE_ELEMENTS_TOKEN_NAME: ABNFToken = ABNFToken {
    name: TOKEN_NAME_TYPE,
    re: TOKEN_NAME_REGEX,
    factory: |_, token| {
        TokenFactory::new(ABNF_REQUIRE_ELEMENTS_TOKEN_NAME.name)
            .push_state(ABNF_STATE_ELEMENTS)
            .build(token)
    },
};

const ABNF_REQUIRE_ELEMENTS_TOKEN_TERMINAL_BINARY: ABNFToken = ABNFToken {
    name: TOKEN_TERMINIAL_TYPE,
    re: TOKEN_TERMINAL_BINARY_REGEX,
    factory: |_, token| {
        TokenFactory::new(ABNF_REQUIRE_ELEMENTS_TOKEN_TERMINAL_BINARY.name)
            .push_state(ABNF_STATE_ELEMENTS)
            .build(token)
    },
};

const ABNF_REQUIRE_ELEMENTS_TOKEN_TERMINAL_DECIMAL: ABNFToken = ABNFToken {
    name: TOKEN_TERMINIAL_TYPE,
    re: TOKEN_TERMINAL_DECIMAL_REGEX,
    factory: |_, token| {
        TokenFactory::new(ABNF_REQUIRE_ELEMENTS_TOKEN_TERMINAL_DECIMAL.name)
            .push_state(ABNF_STATE_ELEMENTS)
            .build(token)
    },
};

const ABNF_REQUIRE_ELEMENTS_TOKEN_TERMINAL_HEXADECIMAL: ABNFToken = ABNFToken {
    name: TOKEN_TERMINIAL_TYPE,
    re: TOKEN_TERMINAL_HEXADECIMAL_REGEX,
    factory: |_, token| {
        TokenFactory::new(ABNF_REQUIRE_ELEMENTS_TOKEN_TERMINAL_HEXADECIMAL.name)
            .push_state(ABNF_STATE_ELEMENTS)
            .build(token)
    },
};

const ABNF_REQUIRE_ELEMENTS_TOKEN_TERMINAL_STRING: ABNFToken = ABNFToken {
    name: TOKEN_TERMINIAL_TYPE,
    re: TOKEN_TERMINAL_STRING_REGEX,
    factory: |_, token| {
        TokenFactory::new(ABNF_REQUIRE_ELEMENTS_TOKEN_TERMINAL_STRING.name)
            .push_state(ABNF_STATE_ELEMENTS)
            .build(token)
    },
};

const ABNF_REQUIRE_ELEMENTS_TOKEN_RANGE: ABNFToken = ABNFToken {
    name: TOKEN_RANGE_TYPE,
    re: TOKEN_RANGE_REGEX,
    factory: |_, token| {
        TokenFactory::new(ABNF_REQUIRE_ELEMENTS_TOKEN_RANGE.name)
            .push_state(ABNF_STATE_ELEMENTS)
            .build(token)
    },
};

const ABNF_REQUIRE_ELEMENTS_TOKEN_LEFT_PARENTHESIS: ABNFToken = ABNFToken {
    name: TOKEN_LEFT_PARENTHESIS_TYPE,
    re: TOKEN_LEFT_PARENTHESIS_REGEX,
    factory: |_, token| {
        TokenFactory::new(ABNF_REQUIRE_ELEMENTS_TOKEN_LEFT_PARENTHESIS.name)
            .push_state(ABNF_STATE_ELEMENTS)
            .push_state(ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS)
            .build(token)
    },
};

const ABNF_REQUIRE_ELEMENTS_TOKEN_VARIABLE: ABNFToken = ABNFToken {
    name: TOKEN_VARIABLE_TYPE,
    re: TOKEN_VARIABLE_REGEX,
    factory: |_, token| {
        TokenFactory::new(ABNF_REQUIRE_ELEMENTS_TOKEN_VARIABLE.name)
            .push_state(ABNF_STATE_ELEMENTS)
            .push_state(ABNF_STATE_VARIABLE_REQUIRE_ELEMENT)
            .build(token)
    },
};

const ABNF_REQUIRE_ELEMENTS_TOKEN_LEFT_OPTIONS: ABNFToken = ABNFToken {
    name: TOKEN_LEFT_OPTIONS_TYPE,
    re: TOKEN_LEFT_OPTIONS_REGEX,
    factory: |_, token| {
        TokenFactory::new(ABNF_REQUIRE_ELEMENTS_TOKEN_VARIABLE.name)
            .push_state(ABNF_STATE_ELEMENTS)
            .push_state(ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS)
            .build(token)
    },
};

const ABNF_TOKEN_EOF: &str = "abnf_token_eof";
const ABNF_IGNORE_REGEX: &str = r"^( |\t)";

fn set_state_parsers(
    state: &mut LexerState<BnfState>,
    token_state: &'static str,
    tokens: &Vec<ABNFToken>,
) {
    tokens.iter().for_each(|token| {
        state.add_token(token_state, token.re, token.factory);
    });
}

pub fn new_lexer_state() -> LexerState<BnfState> {
    let mut state = LexerState::new(ABNF_STATE_INIT, BnfState::new());

    state
        .set_eof(|| Token::new(ABNF_TOKEN_EOF, ""))
        .set_ignore(ABNF_IGNORE_REGEX);

    // ABNF_STATE_INIT
    set_state_parsers(&mut state, ABNF_STATE_INIT, &vec![ABNF_INIT_TOKEN_NAME]);

    // ABNF_STATE_DEFINER
    set_state_parsers(&mut state, ABNF_STATE_DEFINER, &vec![ABNF_TOKEN_DEFINER]);

    // ABNF_STATE_REQUIRE_ELEMENTS
    set_state_parsers(
        &mut state,
        ABNF_STATE_REQUIRE_ELEMENTS,
        &vec![
            ABNF_REQUIRE_ELEMENTS_TOKEN_NAME,
            ABNF_REQUIRE_ELEMENTS_TOKEN_TERMINAL_BINARY,
            ABNF_REQUIRE_ELEMENTS_TOKEN_TERMINAL_DECIMAL,
            ABNF_REQUIRE_ELEMENTS_TOKEN_TERMINAL_HEXADECIMAL,
            ABNF_REQUIRE_ELEMENTS_TOKEN_TERMINAL_STRING,
            ABNF_REQUIRE_ELEMENTS_TOKEN_RANGE,
            ABNF_REQUIRE_ELEMENTS_TOKEN_LEFT_PARENTHESIS,
            ABNF_REQUIRE_ELEMENTS_TOKEN_VARIABLE,
            ABNF_REQUIRE_ELEMENTS_TOKEN_LEFT_OPTIONS,
        ],
    );

    state
}
