use super::{
    super::lex::{LexerState, StateChange, Token, TokenFactory},
    BnfState,
};

struct ABNFToken {
    name: &'static str,
    re: &'static str,
    factory: fn(&mut BnfState, token: &str) -> (Token, StateChange),
}

const ABNF_STATE_INIT: &str = "abnf_state_init";

const ABNF_TOKEN_NAME: ABNFToken = ABNFToken {
    name: "abnf_token_name",
    re: r"[a-zA-Z_][a-zA-Z0-9_]*",
    factory: parse_name,
};
fn parse_name(_: &mut BnfState, token: &str) -> (Token, StateChange) {
    TokenFactory::new(ABNF_TOKEN_NAME.name).build(token)
}

const ABNF_TOKEN_EOF: &str = "abnf_token_eof";
const ABNF_IGNORE_REGEX: &str = r"^( |\t)";

fn set_state_parsers(state: &mut LexerState<BnfState>, tokens: &Vec<ABNFToken>) {
    tokens.iter().for_each(|token| {
        state.add_token(token.name, token.re, token.factory);
    });
}

pub fn new_lexer_state() -> LexerState<BnfState> {
    let mut state = LexerState::new(ABNF_STATE_INIT, BnfState::new());

    state
        .set_eof(|| Token::new(ABNF_TOKEN_EOF, ""))
        .set_ignore(ABNF_IGNORE_REGEX);

    // ABNF_STATE_INIT
    set_state_parsers(&mut state, &vec![ABNF_TOKEN_NAME]);

    state
}
