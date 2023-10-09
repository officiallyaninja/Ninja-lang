#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Number(i64),
    Operation(Operation),
    LBracket,
    RBracket,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operation {
    Add,
    Sub,
    Div,
    Mul,
}

pub fn lex(expr: &str) -> Option<Vec<Token>> {
    let mut tokens = vec![];
    let mut chars: Vec<char> = expr.chars().rev().collect();
    while let Some(char) = chars.pop() {
        let token = match char {
            ' ' => continue,
            '(' => Token::LBracket,
            ')' => Token::RBracket,
            '+' => Token::Operation(Operation::Add),
            '-' => Token::Operation(Operation::Sub),
            '*' => Token::Operation(Operation::Mul),
            '/' => Token::Operation(Operation::Div),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let mut nums: Vec<i64> = vec![];
                chars.push(char);
                loop {
                    let Some(c) = chars.pop()
                    else {
                        break;
                    };
                    if let Some(digit) = c.to_digit(10) {
                        nums.push(digit.into())
                    } else {
                        chars.push(c);
                        break;
                    }
                }

                let num: i64 = nums.into_iter().fold(0, |acc, num| acc * 10 + num);
                Token::Number(num)
            }
            _ => return None,
        };

        tokens.push(token)
    }

    Some(tokens)
}

#[cfg(test)]
mod test {
    use crate::lex::{Operation::*, Token::*};

    use super::lex;
    #[test]
    fn nums() {
        assert_eq!(lex("123"), Some(vec![Number(123)]));
        assert_eq!(lex("0"), Some(vec![Number(0)]));
        assert_eq!(lex("-32"), Some(vec![Operation(Sub), Number(32)]));
        assert_eq!(lex("231123213231"), Some(vec![Number(231123213231)]));
    }
    #[test]
    fn tokens() {
        assert_eq!(
            lex("+-/*"),
            Some(vec![
                Operation(Add),
                Operation(Sub),
                Operation(Div),
                Operation(Mul)
            ])
        );
    }

    #[test]
    fn brackets() {
        assert_eq!(lex("(()"), Some(vec![LBracket, LBracket, RBracket]));
    }

    #[test]
    fn ops() {
        assert_eq!(
            lex("2+3*4"),
            Some(vec![
                Number(2),
                Operation(Add),
                Number(3),
                Operation(Mul),
                Number(4)
            ])
        )
    }
    #[test]
    fn whitespace() {
        assert_eq!(
            lex(" 2  + 3  * 4  "),
            Some(vec![
                Number(2),
                Operation(Add),
                Number(3),
                Operation(Mul),
                Number(4)
            ])
        )
    }
    #[test]
    fn combo() {
        assert_eq!(
            lex("23+34*45"),
            Some(vec![
                Number(23),
                Operation(Add),
                Number(34),
                Operation(Mul),
                Number(45)
            ])
        )
    }
}
