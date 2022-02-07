mod lex;
mod yacc;

pub use lex::*;
pub use yacc::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
