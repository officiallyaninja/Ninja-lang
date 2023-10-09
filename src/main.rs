mod ast;
mod lex;
mod parse;
use lex::lex;
use parse::parse;
fn main() {
    println!("Hello, world!");
}

pub fn evaluate(expr: &str) -> Option<i64> {
    // parse(lex(expr)?)
    todo!()
}

#[cfg(test)]
mod test {
    use crate::evaluate;

    #[test]
    fn equality() {
        assert_eq!(evaluate("4").unwrap(), 4)
    }
}
