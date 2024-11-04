use crate::tokens::{Expression, MathToken};
use log::debug;
use thiserror::Error;
#[derive(Debug, PartialEq, Eq)]
pub struct PostExpression {
    /// postfix ordering of a given math equation.
    tokens: Vec<MathToken>,
}

#[derive(Debug, Error)]
pub enum PostExpressionError {
    #[error("Failed to convert infix to postfix - `{0}`")]
    InvalidExpression(String),
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
                MathToken::Parens(_) => {
                    unreachable!("Unexpected MathToken type in evaluating function")
                }
            };
        }
        assert!(stack.len() == 1, "Invalid postfix expression");
        stack.pop().unwrap()
    }
}
impl TryFrom<Expression> for PostExpression {
    type Error = PostExpressionError;

    fn try_from(eq: Expression) -> Result<Self, Self::Error> {
        let mut queue: Vec<MathToken> = Vec::new();
        let mut op_stack: Vec<MathToken> = Vec::new();

        debug!("Started tokens while-loop");
        for tok in eq.tokens {
            debug!("Working on token {:?}", tok);
            match tok {
                MathToken::IntOperand(x) => queue.push(MathToken::IntOperand(x)),
                MathToken::Operator(op) => {
                    debug!("State of operator stack - {:?}", op_stack);
                    while let Some(MathToken::Operator(top_op)) = op_stack.last() {
                        if *top_op > op {
                            let tobe_pushed = op_stack
                                .pop()
                                .expect("A token should be here since we peeked it.");
                            queue.push(tobe_pushed);
                        } else {
                            break;
                        }
                    }
                    op_stack.push(MathToken::Operator(op));
                }
                MathToken::Parens(true) => op_stack.push(MathToken::Parens(true)),
                MathToken::Parens(false) => {
                    let mut matched = false;
                    while let Some(tok) = op_stack.pop() {
                        match tok {
                            MathToken::Parens(true) => {
                                matched = true;
                                break;
                            }
                            MathToken::Parens(false) => {
                                unreachable!("Stack NEVER stores closing parentheses")
                            }
                            _ => queue.push(tok),
                        }
                    }
                    if !matched {
                        return Err(PostExpressionError::InvalidExpression(
                            "No matching opening parenthesis".to_string(),
                        ));
                    }
                }
            }
        }
        while let Some(tok) = op_stack.pop() {
            queue.push(tok);
        }

        Ok(Self { tokens: queue })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::OperatorType;

    fn init_logger() {
        let _ = env_logger::builder()
            // Include all events in tests
            .filter_level(log::LevelFilter::max())
            // Ensure events are captured by `cargo test`
            .is_test(true)
            // Ignore errors initializing the logger if tests race to configure it
            .try_init();
    }

    #[test]
    fn test_infix_to_postfix() {
        let eq = Expression::new(vec![
            MathToken::IntOperand(3),
            MathToken::Operator(OperatorType::Add),
            MathToken::IntOperand(4),
        ]);
        assert_eq!(
            PostExpression::try_from(eq).unwrap(),
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
    #[test]
    fn test_precedence_postfix() {
        init_logger();
        let eq = Expression::new(vec![
            MathToken::IntOperand(3),
            MathToken::Operator(OperatorType::Add),
            MathToken::IntOperand(4),
            MathToken::Operator(OperatorType::Mul),
            MathToken::IntOperand(2),
        ]);
        assert_eq!(
            PostExpression::try_from(eq).unwrap(),
            PostExpression {
                tokens: vec![
                    MathToken::IntOperand(3),
                    MathToken::IntOperand(4),
                    MathToken::IntOperand(2),
                    MathToken::Operator(OperatorType::Mul),
                    MathToken::Operator(OperatorType::Add),
                ]
            }
        );
    }
    #[test]
    fn test_invalid_parens() {
        let eq = Expression::new(vec![
            MathToken::Parens(false),
            MathToken::IntOperand(3),
            MathToken::Parens(false),
        ]);
        assert!(PostExpression::try_from(eq).is_err());
    }
    #[test]
    fn test_paren_precedence() {
        let eq = Expression::new(vec![
            MathToken::Parens(true),
            MathToken::IntOperand(3),
            MathToken::Operator(OperatorType::Add),
            MathToken::IntOperand(4),
            MathToken::Parens(false),
            MathToken::Operator(OperatorType::Mul),
            MathToken::IntOperand(2),
        ]);
        assert_eq!(
            PostExpression::try_from(eq).unwrap(),
            PostExpression {
                tokens: vec![
                    MathToken::IntOperand(3),
                    MathToken::IntOperand(4),
                    MathToken::Operator(OperatorType::Add),
                    MathToken::IntOperand(2),
                    MathToken::Operator(OperatorType::Mul),
                ]
            }
        );
    }
    #[test]
    fn test_paren_precedence_eval() {
        let eq = Expression::new(vec![
            MathToken::Parens(true),
            MathToken::IntOperand(3),
            MathToken::Operator(OperatorType::Add),
            MathToken::IntOperand(4),
            MathToken::Parens(false),
            MathToken::Operator(OperatorType::Mul),
            MathToken::IntOperand(2),
        ]);
        assert_eq!(PostExpression::try_from(eq).unwrap().eval(), 14);
    }
    #[test]
    fn test_eval_power() {
        let eq = Expression::new(vec![
            MathToken::IntOperand(3),
            MathToken::Operator(OperatorType::Pow),
            MathToken::IntOperand(4),
        ]);
        assert_eq!(PostExpression::try_from(eq).unwrap().eval(), 81);
    }
}
