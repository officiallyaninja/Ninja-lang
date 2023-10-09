use crate::lex::{Operation, Token};

#[derive(PartialEq, Debug)]
pub enum ASTNode {
    Operator {
        left: Box<ASTNode>,
        op: Operation,
        right: Box<ASTNode>,
    },
    Number(i64),
}

impl ASTNode {
    pub fn from_token_stream(tokens: Vec<Token>) -> Option<ASTNode> {
        if tokens.len() == 1 {
            return match tokens
                .last()
                .expect("length is one so there must be some element")
            {
                Token::Number(num) => Some(ASTNode::Number(*num)),
                _ => None,
            };
        }

        let mut stream = tokens.into_iter();
        let mut left: Vec<Token> = Vec::new();
        let op: Operation = loop {
            let Some(token) = stream.next()
            else {
                return None
            };
            if let Token::Operation(oper) = &token {
                if let Operation::Div | Operation::Mul = oper {
                    break oper.clone();
                }
            }
            left.push(token);
        };
        let right: Vec<_> = stream.collect();

        let Some(left) = ASTNode::from_token_stream(left)
        else {
            return None;
        };
        let Some(right) = ASTNode::from_token_stream(right)
        else {
            return None;
        };

        Some(ASTNode::Operator {
            left: Box::new(left),
            op,
            right: Box::new(right),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{ast::ASTNode, lex};

    #[test]
    fn two_plus_two() {
        assert_eq!(
            ASTNode::from_token_stream(lex("2*2").expect("should lex"))
                .expect("should parse as syntax tree"),
            ASTNode::Operator {
                left: Box::new(ASTNode::Number(2)),
                op: lex::Operation::Mul,
                right: Box::new(ASTNode::Number(2))
            }
        )
    }
}
