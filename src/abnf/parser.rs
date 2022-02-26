use super::{
    super::lex::{LexerState, StateChange, Token, TokenFactory},
    BnfState,
};

const TOKEN_NAME_TYPE: &str = "abnf_token_name";
const TOKEN_DEFINER_TYPE: &str = "abnf_token_definer";
const TOKEN_TERMINAL_TYPE: &str = "abnf_token_terminal";
const TOKEN_RANGE_TYPE: &str = "abnf_token_range";
const TOKEN_LEFT_PARENTHESIS_TYPE: &str = "abnf_token_left_parenthesis";
const TOKEN_RIGHT_PARENTHESIS_TYPE: &str = "abnf_token_right_parenthesis";
const TOKEN_VARIABLE_TYPE: &str = "abnf_token_variable";
const TOKEN_LEFT_OPTIONS_TYPE: &str = "abnf_token_left_options";
const TOKEN_RIGHT_OPTIONS_TYPE: &str = "abnf_token_right_options";
const TOKEN_CHOOSE_TYPE: &str = "abnf_token_choose";
const TOKEN_END_TYPE: &str = "abnf_token_end";
const TOKEN_REQUIREMENT_TYPE: &str = "abnf_requirement";

const TOKEN_NAME_REGEX: &str = r"^[a-zA-Z][a-zA-Z0-9\-]*";
const TOKEN_DEFINER_REGEX: &str = r"^=/?";
const TOKEN_TERMINAL_BINARY_REGEX: &str = r"^%b(0|1)+(\.(0|1)+)*";
const TOKEN_TERMINAL_DECIMAL_REGEX: &str = r"^%d\d+(\.\d+)*";
const TOKEN_TERMINAL_HEXADECIMAL_REGEX: &str = r"^%h[a-fA-F0-9]+(\.[a-fA-F0-9]+)*";
const TOKEN_TERMINAL_STRING_REGEX: &str = r#"^"(?:\\"|[^"])*?""#;
const TOKEN_RANGE_REGEX: &str = r"^(b(0|1)+-(0|1)|d\d+-\d+|x[a-fA-F0-9]+-[a-fA-F0-9]+)";
const TOKEN_LEFT_PARENTHESIS_REGEX: &str = r"^\(";
const TOKEN_RIGHT_PARENTHESIS_REGEX: &str = r"^\)";
const TOKEN_VARIABLE_REGEX: &str = r"^(\d*\*\d*|\d+)";
const TOKEN_LEFT_OPTIONS_REGEX: &str = r"^\[";
const TOKEN_RIGHT_OPTIONS_REGEX: &str = r"^\]";
const TOKEN_CHOOSE_REGEX: &str = r"^/";
const TOKEN_END_REGEX: &str = r"^;";
const TOKEN_REQUIREMENT_REGEX: &str = r"^<.*?>";

// ABNF_STATE_INIT
//
// from ABNF_STATE_ELEMENTS
//
// to   ABNF_STATE_DEFINER
const ABNF_STATE_INIT: &str = "abnf_state_init";

// ABNF_STATE_DEFINER
//
// from ABNF_STATE_INIT
//
// to   ABNF_REQUIRE_STATE_ELEMENTS
const ABNF_STATE_DEFINER: &str = "abnf_state_definer";

// ABNF_STATE_REQUIRE_ELEMENTS
//
// from ABNF_STATE_DEFINER |
//      ABNF_STATE_ELEMENTS
//
// to   ABNF_STATE_ELEMENTS
const ABNF_STATE_REQUIRE_ELEMENTS: &str = "abnf_state_require_elements";

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
const ABNF_STATE_ELEMENTS: &str = "abnf_state_elements";

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
const ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS: &str = "abnf_state_parenthesis_require_elements";

// ABNF_STATE_PARENTHESIS_ELEMENTS
//
// from ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
//
// to   ABNF_STATE_PARENTHESIS_ELEMENTS |
//      ABNF_STATE_ELEMENTS |
//      ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS |
//      ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS |
//      ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
const ABNF_STATE_PARENTHESIS_ELEMENTS: &str = "abnf_state_parenthesis_elements";

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
const ABNF_STATE_VARIABLE_REQUIRE_ELEMENT: &str = "abnf_state_variable_require_element";

// ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS
//
// from ABNF_STATE_REQUIRE_ELEMENTS |
//      ABNF_STATE_ELEMENTS |
//      ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS |
//      ABNF_STATE_PARENTHESIS_ELEMENTS
//
// to   ABNF_STATE_OPTIONS_ELEMENTS
const ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS: &str = "abnf_state_options_require_elements";

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
const ABNF_STATE_OPTIONS_ELEMENTS: &str = "abnf_state_options_elements";

fn abnf_init_token_name(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_NAME_TYPE)
        .push_state(ABNF_STATE_DEFINER)
        .build(token)
}

fn abnf_definer_token_definer(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_DEFINER_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_REQUIRE_ELEMENTS)
        .build(token)
}

fn abnf_ele_type_name(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_NAME_TYPE).build(token)
}

fn abnf_reqe_type_name(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_NAME_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_ELEMENTS)
        .build(token)
}

fn abnf_reqpe_type_name(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_NAME_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_PARENTHESIS_ELEMENTS)
        .build(token)
}

fn abnf_reqve_type_name(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_NAME_TYPE).pop_state(1).build(token)
}

fn abnf_reqoe_type_name(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_NAME_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_OPTIONS_ELEMENTS)
        .build(token)
}

fn abnf_reqe_type_terminal(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_TERMINAL_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_ELEMENTS)
        .build(token)
}

fn abnf_ele_type_terminal(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_TERMINAL_TYPE).build(token)
}

fn abnf_reqpe_type_terminal(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_TERMINAL_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_PARENTHESIS_ELEMENTS)
        .build(token)
}

fn abnf_reqpe_type_requirement(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_REQUIREMENT_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_PARENTHESIS_ELEMENTS)
        .build(token)
}

fn abnf_reqve_type_terminal(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_TERMINAL_TYPE)
        .pop_state(1)
        .build(token)
}

fn abnf_reqve_type_requirement(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_REQUIREMENT_TYPE)
        .pop_state(1)
        .build(token)
}

fn abnf_reqoe_type_terminal(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_TERMINAL_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_OPTIONS_ELEMENTS)
        .build(token)
}

fn abnf_reqoe_type_requirement(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_REQUIREMENT_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_OPTIONS_ELEMENTS)
        .build(token)
}

fn abnf_reqe_type_range(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_RANGE_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_ELEMENTS)
        .build(token)
}

fn abnf_ele_type_range(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_RANGE_TYPE).build(token)
}

fn abnf_reqpe_type_range(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_RANGE_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_PARENTHESIS_ELEMENTS)
        .build(token)
}

fn abnf_reqve_type_range(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_RANGE_TYPE)
        .pop_state(1)
        .build(token)
}

fn abnf_reqoe_type_range(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_RANGE_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_OPTIONS_ELEMENTS)
        .build(token)
}

fn abnf_reqe_type_parenthesis(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_LEFT_PARENTHESIS_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_ELEMENTS)
        .push_state(ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS)
        .build(token)
}

fn abnf_ele_type_parenthesis(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_LEFT_PARENTHESIS_TYPE)
        .push_state(ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS)
        .build(token)
}

fn abnf_reqpe_type_parenthesis(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_LEFT_PARENTHESIS_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_PARENTHESIS_ELEMENTS)
        .push_state(ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS)
        .build(token)
}

fn abnf_reqve_type_parenthesis(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_LEFT_PARENTHESIS_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS)
        .build(token)
}

fn abnf_reqoe_type_parenthesis(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_LEFT_PARENTHESIS_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_OPTIONS_ELEMENTS)
        .push_state(ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS)
        .build(token)
}

fn abnf_reqe_type_variable(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_VARIABLE_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_ELEMENTS)
        .push_state(ABNF_STATE_VARIABLE_REQUIRE_ELEMENT)
        .build(token)
}

fn abnf_ele_type_variable(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_VARIABLE_TYPE)
        .push_state(ABNF_STATE_VARIABLE_REQUIRE_ELEMENT)
        .build(token)
}

fn abnf_reqpe_type_variable(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_VARIABLE_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_PARENTHESIS_ELEMENTS)
        .push_state(ABNF_STATE_VARIABLE_REQUIRE_ELEMENT)
        .build(token)
}

fn abnf_reqoe_type_variable(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_VARIABLE_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_OPTIONS_ELEMENTS)
        .push_state(ABNF_STATE_VARIABLE_REQUIRE_ELEMENT)
        .build(token)
}

fn abnf_reqe_type_options(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_LEFT_OPTIONS_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_ELEMENTS)
        .push_state(ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS)
        .build(token)
}

fn abnf_reqe_type_requirement(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_REQUIREMENT_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_ELEMENTS)
        .build(token)
}

fn abnf_ele_type_options(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_LEFT_OPTIONS_TYPE)
        .push_state(ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS)
        .build(token)
}

fn abnf_reqpe_type_options(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_LEFT_OPTIONS_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_PARENTHESIS_ELEMENTS)
        .push_state(ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS)
        .build(token)
}

fn abnf_reqoe_type_options(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_LEFT_OPTIONS_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_OPTIONS_ELEMENTS)
        .push_state(ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS)
        .build(token)
}

fn abnf_ele_type_choose(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_CHOOSE_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_REQUIRE_ELEMENTS)
        .build(token)
}

fn abnf_ele_type_requirement(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_REQUIREMENT_TYPE).build(token)
}

fn abnf_pele_type_choose(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_CHOOSE_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS)
        .build(token)
}

fn abnf_oele_type_choose(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_CHOOSE_TYPE)
        .pop_state(1)
        .push_state(ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS)
        .build(token)
}

fn abnf_ele_type_end(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_END_TYPE).pop_state(1).build(token)
}

fn abnf_pele_type_end(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_RIGHT_PARENTHESIS_TYPE)
        .pop_state(1)
        .build(token)
}

fn abnf_oele_type_end(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(TOKEN_RIGHT_OPTIONS_TYPE)
        .pop_state(1)
        .build(token)
}

const ABNF_TOKEN_EOF: &str = "abnf_token_eof";
const ABNF_IGNORE_REGEX: &str = r"^( |\t)";

fn set_state_parsers(
    state: &mut LexerState<BnfState>,
    token_state: &'static str,
    tokens: &Vec<(
        &'static str,
        fn(&mut BnfState, &str) -> (Token, StateChange),
    )>,
) {
    tokens.iter().for_each(|token| {
        state.add_token(token_state, token.0, token.1);
    });
}

pub fn new_lexer_state() -> LexerState<BnfState> {
    let mut state = LexerState::new(ABNF_STATE_INIT, BnfState::new());

    state
        .set_eof(|| Token::new(ABNF_TOKEN_EOF, ""))
        .set_ignore(ABNF_IGNORE_REGEX);

    // ABNF_STATE_INIT
    set_state_parsers(
        &mut state,
        ABNF_STATE_INIT,
        &vec![
            (TOKEN_NAME_REGEX, abnf_init_token_name), // -> ABNF_STATE_DEFINER
        ],
    );

    // ABNF_STATE_DEFINER
    set_state_parsers(
        &mut state,
        ABNF_STATE_DEFINER,
        &vec![
            (TOKEN_DEFINER_REGEX, abnf_definer_token_definer), // -> ABNF_STATE_REQUIRE_ELEMENTS
        ],
    );

    // ABNF_STATE_REQUIRE_ELEMENTS
    set_state_parsers(
        &mut state,
        ABNF_STATE_REQUIRE_ELEMENTS,
        &vec![
            // -> ABNF_STATE_ELEMENTS
            (TOKEN_NAME_REGEX, abnf_reqe_type_name),
            // -> ABNF_STATE_ELEMENTS
            (TOKEN_TERMINAL_BINARY_REGEX, abnf_reqe_type_terminal),
            // -> ABNF_STATE_ELEMENTS
            (TOKEN_TERMINAL_DECIMAL_REGEX, abnf_reqe_type_terminal),
            // -> ABNF_STATE_ELEMENTS
            (TOKEN_TERMINAL_HEXADECIMAL_REGEX, abnf_reqe_type_terminal),
            // -> ABNF_STATE_ELEMENTS
            (TOKEN_TERMINAL_STRING_REGEX, abnf_reqe_type_terminal),
            // -> ABNF_STATE_ELEMENTS
            (TOKEN_RANGE_REGEX, abnf_reqe_type_range),
            // -> ABNF_STATE_ELEMENTS -> ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
            (TOKEN_LEFT_PARENTHESIS_REGEX, abnf_reqe_type_parenthesis),
            // -> ABNF_STATE_ELEMENTS -> ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
            (TOKEN_VARIABLE_REGEX, abnf_reqe_type_variable),
            // ABNF_STATE_ELEMENTS -> ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS
            (TOKEN_LEFT_OPTIONS_REGEX, abnf_reqe_type_options),
            // -> ABNF_STATE_ELEMENTS
            (TOKEN_REQUIREMENT_REGEX, abnf_reqe_type_requirement),
        ],
    );

    // ABNF_STATE_ELEMENTS
    set_state_parsers(
        &mut state,
        ABNF_STATE_ELEMENTS,
        &vec![
            (TOKEN_NAME_REGEX, abnf_ele_type_name),
            (TOKEN_TERMINAL_BINARY_REGEX, abnf_ele_type_terminal),
            (TOKEN_TERMINAL_DECIMAL_REGEX, abnf_ele_type_terminal),
            (TOKEN_TERMINAL_HEXADECIMAL_REGEX, abnf_ele_type_terminal),
            (TOKEN_TERMINAL_STRING_REGEX, abnf_ele_type_terminal),
            (TOKEN_RANGE_REGEX, abnf_ele_type_range),
            // -> ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
            (TOKEN_LEFT_PARENTHESIS_REGEX, abnf_ele_type_parenthesis),
            // -> ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
            (TOKEN_VARIABLE_REGEX, abnf_ele_type_variable),
            // -> ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS
            (TOKEN_LEFT_OPTIONS_REGEX, abnf_ele_type_options),
            // -> ABNF_STATE_REQUIRE_ELEMENTS
            (TOKEN_CHOOSE_REGEX, abnf_ele_type_choose),
            // -> ABNF_STATE_INIT
            (TOKEN_END_REGEX, abnf_ele_type_end),
            (TOKEN_REQUIREMENT_REGEX, abnf_ele_type_requirement),
        ],
    );

    // ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
    set_state_parsers(
        &mut state,
        ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS,
        &vec![
            // -> ABNF_STATE_PARENTHESIS_ELEMENTS
            (TOKEN_NAME_REGEX, abnf_reqpe_type_name),
            // -> ABNF_STATE_PARENTHESIS_ELEMENTS
            (TOKEN_TERMINAL_BINARY_REGEX, abnf_reqpe_type_terminal),
            // -> ABNF_STATE_PARENTHESIS_ELEMENTS
            (TOKEN_TERMINAL_DECIMAL_REGEX, abnf_reqpe_type_terminal),
            // -> ABNF_STATE_PARENTHESIS_ELEMENTS
            (TOKEN_TERMINAL_HEXADECIMAL_REGEX, abnf_reqpe_type_terminal),
            // -> ABNF_STATE_PARENTHESIS_ELEMENTS
            (TOKEN_TERMINAL_STRING_REGEX, abnf_reqpe_type_terminal),
            // -> ABNF_STATE_PARENTHESIS_ELEMENTS
            (TOKEN_RANGE_REGEX, abnf_reqpe_type_range),
            // -> ABNF_STATE_PARENTHESIS_ELEMENTS -> ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
            (TOKEN_LEFT_PARENTHESIS_REGEX, abnf_reqpe_type_parenthesis),
            // -> ABNF_STATE_PARENTHESIS_ELEMENTS -> ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
            (TOKEN_VARIABLE_REGEX, abnf_reqpe_type_variable),
            // -> ABNF_STATE_PARENTHESIS_ELEMENTS -> ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS
            (TOKEN_LEFT_OPTIONS_REGEX, abnf_reqpe_type_options),
            // -> ABNF_STATE_PARENTHESIS_ELEMENTS
            (TOKEN_REQUIREMENT_REGEX, abnf_reqpe_type_requirement),
        ],
    );

    // ABNF_STATE_PARENTHESIS_ELEMENTS
    set_state_parsers(
        &mut state,
        ABNF_STATE_PARENTHESIS_ELEMENTS,
        &vec![
            (TOKEN_NAME_REGEX, abnf_ele_type_name),
            (TOKEN_TERMINAL_BINARY_REGEX, abnf_ele_type_terminal),
            (TOKEN_TERMINAL_DECIMAL_REGEX, abnf_ele_type_terminal),
            (TOKEN_TERMINAL_HEXADECIMAL_REGEX, abnf_ele_type_terminal),
            (TOKEN_TERMINAL_STRING_REGEX, abnf_ele_type_terminal),
            (TOKEN_RANGE_REGEX, abnf_ele_type_range),
            // -> ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
            (TOKEN_LEFT_PARENTHESIS_REGEX, abnf_ele_type_parenthesis),
            // -> ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
            (TOKEN_VARIABLE_REGEX, abnf_ele_type_variable),
            // -> ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS
            (TOKEN_LEFT_OPTIONS_REGEX, abnf_ele_type_options),
            // -> ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
            (TOKEN_CHOOSE_REGEX, abnf_pele_type_choose),
            // -> maybe
            // ABNF_STATE_ELEMENTS |
            // ABNF_STATE_PARENTHESIS_ELEMENTS |
            // ABNF_STATE_OPTIONS_ELEMENTS
            (TOKEN_RIGHT_PARENTHESIS_REGEX, abnf_pele_type_end),
            (TOKEN_REQUIREMENT_REGEX, abnf_ele_type_requirement),
        ],
    );

    // ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
    set_state_parsers(
        &mut state,
        ABNF_STATE_VARIABLE_REQUIRE_ELEMENT,
        &vec![
            // -> maybe
            // ABNF_STATE_ELEMENTS |
            // ABNF_STATE_PARENTHESIS_ELEMENTS |
            // ABNF_STATE_OPTIONS_ELEMENTS
            (TOKEN_NAME_REGEX, abnf_reqve_type_name),
            // -> maybe
            // ABNF_STATE_ELEMENTS |
            // ABNF_STATE_PARENTHESIS_ELEMENTS |
            // ABNF_STATE_OPTIONS_ELEMENTS
            (TOKEN_TERMINAL_BINARY_REGEX, abnf_reqve_type_terminal),
            // -> maybe
            // ABNF_STATE_ELEMENTS |
            // ABNF_STATE_PARENTHESIS_ELEMENTS |
            // ABNF_STATE_OPTIONS_ELEMENTS
            (TOKEN_TERMINAL_DECIMAL_REGEX, abnf_reqve_type_terminal),
            // -> maybe
            // ABNF_STATE_ELEMENTS |
            // ABNF_STATE_PARENTHESIS_ELEMENTS |
            // ABNF_STATE_OPTIONS_ELEMENTS
            (TOKEN_TERMINAL_HEXADECIMAL_REGEX, abnf_reqve_type_terminal),
            // -> maybe
            // ABNF_STATE_ELEMENTS |
            // ABNF_STATE_PARENTHESIS_ELEMENTS |
            // ABNF_STATE_OPTIONS_ELEMENTS
            (TOKEN_TERMINAL_STRING_REGEX, abnf_reqve_type_terminal),
            // -> maybe
            // ABNF_STATE_ELEMENTS |
            // ABNF_STATE_PARENTHESIS_ELEMENTS |
            // ABNF_STATE_OPTIONS_ELEMENTS
            (TOKEN_RANGE_REGEX, abnf_reqve_type_range),
            // -> maybe
            // ABNF_STATE_ELEMENTS |
            // ABNF_STATE_PARENTHESIS_ELEMENTS |
            // ABNF_STATE_OPTIONS_ELEMENTS
            // -> ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
            (TOKEN_LEFT_PARENTHESIS_REGEX, abnf_reqve_type_parenthesis),
            // -> maybe
            // ABNF_STATE_ELEMENTS |
            // ABNF_STATE_PARENTHESIS_ELEMENTS |
            // ABNF_STATE_OPTIONS_ELEMENTS
            (TOKEN_REQUIREMENT_REGEX, abnf_reqve_type_requirement),
        ],
    );

    // ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS
    set_state_parsers(
        &mut state,
        ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS,
        &vec![
            // -> ABNF_STATE_OPTIONS_ELEMENTS
            (TOKEN_NAME_REGEX, abnf_reqoe_type_name),
            // -> ABNF_STATE_OPTIONS_ELEMENTS
            (TOKEN_TERMINAL_BINARY_REGEX, abnf_reqoe_type_terminal),
            // -> ABNF_STATE_OPTIONS_ELEMENTS
            (TOKEN_TERMINAL_DECIMAL_REGEX, abnf_reqoe_type_terminal),
            // -> ABNF_STATE_OPTIONS_ELEMENTS
            (TOKEN_TERMINAL_HEXADECIMAL_REGEX, abnf_reqoe_type_terminal),
            // -> ABNF_STATE_OPTIONS_ELEMENTS
            (TOKEN_TERMINAL_STRING_REGEX, abnf_reqoe_type_terminal),
            // -> ABNF_STATE_OPTIONS_ELEMENTS
            (TOKEN_RANGE_REGEX, abnf_reqoe_type_range),
            // -> ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
            (TOKEN_LEFT_PARENTHESIS_REGEX, abnf_reqoe_type_parenthesis),
            // -> ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
            (TOKEN_VARIABLE_REGEX, abnf_reqoe_type_variable),
            // -> ABNF_STATE_OPTIONS_ELEMENTS -> ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS
            (TOKEN_LEFT_OPTIONS_REGEX, abnf_reqoe_type_options),
            // -> ABNF_STATE_OPTIONS_ELEMENTS
            (TOKEN_REQUIREMENT_REGEX, abnf_reqoe_type_requirement),
        ],
    );

    // ABNF_STATE_OPTIONS_ELEMENTS
    set_state_parsers(
        &mut state,
        ABNF_STATE_OPTIONS_ELEMENTS,
        &vec![
            (TOKEN_NAME_REGEX, abnf_ele_type_name),
            (TOKEN_TERMINAL_BINARY_REGEX, abnf_ele_type_terminal),
            (TOKEN_TERMINAL_DECIMAL_REGEX, abnf_ele_type_terminal),
            (TOKEN_TERMINAL_HEXADECIMAL_REGEX, abnf_ele_type_terminal),
            (TOKEN_TERMINAL_STRING_REGEX, abnf_ele_type_terminal),
            (TOKEN_RANGE_REGEX, abnf_ele_type_range),
            // -> ABNF_STATE_PARENTHESIS_REQUIRE_ELEMENTS
            (TOKEN_LEFT_PARENTHESIS_REGEX, abnf_ele_type_parenthesis),
            // -> ABNF_STATE_VARIABLE_REQUIRE_ELEMENT
            (TOKEN_VARIABLE_REGEX, abnf_ele_type_variable),
            // -> ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS
            (TOKEN_LEFT_OPTIONS_REGEX, abnf_ele_type_options),
            // -> ABNF_STATE_OPTIONS_REQUIRE_ELEMENTS
            (TOKEN_CHOOSE_REGEX, abnf_oele_type_choose),
            // -> maybe
            // ABNF_STATE_ELEMENTS |
            // ABNF_STATE_PARENTHESIS_ELEMENTS |
            // ABNF_STATE_OPTIONS_ELEMENTS
            (TOKEN_RIGHT_OPTIONS_REGEX, abnf_oele_type_end),
            (TOKEN_REQUIREMENT_REGEX, abnf_ele_type_requirement),
        ],
    );

    state
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
                (TOKEN_NAME_TYPE, "token"),
                (TOKEN_DEFINER_TYPE, "="),
                (TOKEN_NAME_TYPE, "name1"),
                (TOKEN_NAME_TYPE, "name2"),
                (TOKEN_TERMINAL_TYPE, "name2"),
                (TOKEN_END_TYPE, ";"),
            ],
        );
    }

    #[test]
    fn test_terminal_abnf() {
        assert_token(
            "token = \"\\\"foo\\\"\" \"bar\";",
            vec![
                (TOKEN_NAME_TYPE, "token"),
                (TOKEN_DEFINER_TYPE, "="),
                (TOKEN_TERMINAL_TYPE, "\"\\\"foo\\\"\""),
                (TOKEN_TERMINAL_TYPE, "\"bar\""),
                (TOKEN_END_TYPE, ";"),
            ],
        );
    }

    #[test]
    fn test_parenthesis_abnf() {
        assert_token(
            "token = (name1 (\"terminal string\"))name2((name3)name4);",
            vec![
                (TOKEN_NAME_TYPE, "token"),
                (TOKEN_DEFINER_TYPE, "="),
                (TOKEN_LEFT_PARENTHESIS_TYPE, "("),
                (TOKEN_NAME_TYPE, "name1"),
                (TOKEN_LEFT_PARENTHESIS_TYPE, "("),
                (TOKEN_TERMINAL_TYPE, "\"terminal string\""),
                (TOKEN_RIGHT_PARENTHESIS_TYPE, ")"),
                (TOKEN_RIGHT_PARENTHESIS_TYPE, ")"),
                (TOKEN_NAME_TYPE, "name2"),
                (TOKEN_LEFT_PARENTHESIS_TYPE, "("),
                (TOKEN_LEFT_PARENTHESIS_TYPE, "("),
                (TOKEN_NAME_TYPE, "name3"),
                (TOKEN_RIGHT_PARENTHESIS_TYPE, ")"),
                (TOKEN_NAME_TYPE, "name4"),
                (TOKEN_RIGHT_PARENTHESIS_TYPE, ")"),
                (TOKEN_END_TYPE, ";"),
            ],
        )
    }

    #[test]
    fn test_options_abnf() {
        assert_token(
            "token = [option-token1 \";\" option-token2 (parenthesis [option-token3])];",
            vec![
                (TOKEN_NAME_TYPE, "token"),
                (TOKEN_DEFINER_TYPE, "="),
                (TOKEN_LEFT_OPTIONS_TYPE, "["),
                (TOKEN_NAME_TYPE, "option-token1"),
                (TOKEN_TERMINAL_TYPE, "\";\""),
                (TOKEN_NAME_TYPE, "option-token2"),
                (TOKEN_LEFT_PARENTHESIS_TYPE, "("),
                (TOKEN_NAME_TYPE, "parenthesis"),
                (TOKEN_LEFT_OPTIONS_TYPE, "["),
                (TOKEN_NAME_TYPE, "option-token3"),
                (TOKEN_RIGHT_OPTIONS_TYPE, "]"),
                (TOKEN_RIGHT_PARENTHESIS_TYPE, ")"),
                (TOKEN_RIGHT_OPTIONS_TYPE, "]"),
            ],
        )
    }

    #[test]
    fn test_choose_abnf() {
        assert_token(
            "token = choose-1 / choose-2 / choose-3;",
            vec![
                (TOKEN_NAME_TYPE, "token"),
                (TOKEN_DEFINER_TYPE, "="),
                (TOKEN_NAME_TYPE, "choose-1"),
                (TOKEN_CHOOSE_TYPE, "/"),
                (TOKEN_NAME_TYPE, "choose-2"),
                (TOKEN_CHOOSE_TYPE, "/"),
                (TOKEN_NAME_TYPE, "choose-3"),
                (TOKEN_END_TYPE, ";"),
            ],
        )
    }
}
