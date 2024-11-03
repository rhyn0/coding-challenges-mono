use crate::tokens::{Expression, MathToken};

#[derive(Debug, PartialEq, Eq)]
pub struct PostExpression {
    /// postfix ordering of a given math equation.
    tokens: Vec<MathToken>,
}

impl PostExpression {
    pub fn eval(self) -> isize {
        let mut stack = Vec::new();
        for tok in self.tokens {
            match tok {
                MathToken::IntOperand(x) => stack.push(x),
                MathToken::Operator(op) => {
                    let rhs = stack.pop().expect("A number for an operator");
                    let lhs = stack.pop().expect("A number for a binary operator");
                    stack.push(op.apply(lhs, rhs));
                }
            };
        }
        assert!(stack.len() == 1, "Invalid postfix expression");
        stack.pop().unwrap()
    }
    pub fn from_infix(eq: Expression) -> Self {
        let mut queue: Vec<MathToken> = Vec::new();
        let mut op_stack: Vec<MathToken> = Vec::new();

        for tok in eq.tokens {
            if let MathToken::IntOperand(x) = tok {
                queue.push(MathToken::IntOperand(x));
            } else {
                op_stack.push(tok);
            }
        }
        while let Some(tok) = op_stack.pop() {
            queue.push(tok);
        }

        Self { tokens: queue }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::OperatorType;

    use super::*;

    #[test]
    fn test_infix_to_postfix() {
        let eq = Expression::new(vec![
            MathToken::IntOperand(3),
            MathToken::Operator(OperatorType::Add),
            MathToken::IntOperand(4),
        ]);
        assert_eq!(
            PostExpression::from_infix(eq),
            PostExpression {
                tokens: vec![
                    MathToken::IntOperand(3),
                    MathToken::IntOperand(4),
                    MathToken::Operator(OperatorType::Add),
                ]
            }
        );
    }
    #[test]
    fn test_eval_add() {
        let eq = PostExpression {
            tokens: vec![
                MathToken::IntOperand(3),
                MathToken::IntOperand(4),
                MathToken::Operator(OperatorType::Add),
            ],
        };
        assert_eq!(eq.eval(), 7);
    }
    #[test]
    fn test_eval_sub() {
        let eq = PostExpression {
            tokens: vec![
                MathToken::IntOperand(3),
                MathToken::IntOperand(4),
                MathToken::Operator(OperatorType::Sub),
            ],
        };
        assert_eq!(eq.eval(), -1);
    }
    #[test]
    fn test_eval_mul() {
        let eq = PostExpression {
            tokens: vec![
                MathToken::IntOperand(3),
                MathToken::IntOperand(4),
                MathToken::Operator(OperatorType::Mul),
            ],
        };
        assert_eq!(eq.eval(), 12);
    }
    #[test]
    fn test_eval_div() {
        let eq = PostExpression {
            tokens: vec![
                MathToken::IntOperand(3),
                MathToken::IntOperand(4),
                MathToken::Operator(OperatorType::Div),
            ],
        };
        assert_eq!(eq.eval(), 0);
    }
}
