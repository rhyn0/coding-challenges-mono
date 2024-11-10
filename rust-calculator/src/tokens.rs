use log::debug;
use std::{cmp, str::FromStr};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum OperatorType {
    Add,
    Sub,
    Div,
    Mul,
    Pow,
}

impl PartialOrd for OperatorType {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OperatorType {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let precedence = |op: &Self| match op {
            // greater value means heigher precedence.
            Self::Pow => 3,
            Self::Mul | Self::Div => 2,
            Self::Add | Self::Sub => 1,
        };
        debug!("Comparing operators {:?} and {:?}", self, other);
        precedence(self).cmp(&precedence(other))
    }
}

impl OperatorType {
    #[allow(clippy::cast_sign_loss)]
    pub fn apply(&self, lhs: isize, rhs: isize) -> isize {
        match self {
            Self::Add => lhs + rhs,
            Self::Div => lhs / rhs,
            Self::Sub => lhs - rhs,
            Self::Mul => lhs * rhs,
            Self::Pow => lhs.pow(u32::try_from(rhs).unwrap()),
        }
    }
}
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum MathEquationErr {
    #[error("The operator `{0}` is not recognized as a valid operation.")]
    InvalidOperatorType(String),
    #[error("Invalid operand number - `{0}`. Not recognized as float or int.")]
    InvalidOperand(String),
}

impl FromStr for OperatorType {
    type Err = MathEquationErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "/" => Ok(Self::Div),
            "*" => Ok(Self::Mul),
            "^" => Ok(Self::Pow),
            _ => Err(MathEquationErr::InvalidOperatorType(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MathToken {
    IntOperand(isize),
    // FloatOperand(f64),
    Operator(OperatorType),
    /// Opening or closing parentheses
    Parens(bool),
}

impl FromStr for MathToken {
    type Err = MathEquationErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // if let Ok(f) = s.parse::<f64>() {
        //     return Ok(Self::FloatOperand(f));
        // } else
        debug!("Checking token as parentheses");
        if s == "(" {
            return Ok(Self::Parens(true));
        } else if s == ")" {
            return Ok(Self::Parens(false));
        }
        debug!("Evaluating '{}' as an operand", s);
        if let Ok(i) = s.parse::<isize>() {
            return Ok(Self::IntOperand(i));
        }
        debug!("Evaluating '{}' as an operator", s);
        let op_res = s.parse::<OperatorType>();
        op_res.map_or_else(Err, |op| Ok(Self::Operator(op)))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Expression {
    /// postfix ordering of a given math equation.
    pub tokens: Vec<MathToken>,
}

impl FromStr for Expression {
    type Err = MathEquationErr;
    /// Shunting Yard Algorithm
    /// <https://en.wikipedia.org/wiki/Shunting_yard_algorithm#The_algorithm_in_detail>
    /// Assume that tokens (besides parentheses) are separated by spaces.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let toks: Result<Vec<_>, _> = s
            .split_whitespace()
            .flat_map(|t| {
                if t.starts_with('(') {
                    let mut chars = t.chars();
                    let first = chars
                        .next()
                        .expect("Opening parenthesis to exist")
                        .to_string();
                    let rest = chars.collect::<String>();
                    vec![first.parse::<MathToken>(), rest.parse::<MathToken>()]
                } else if t.ends_with(')') {
                    let chars = t.chars();
                    let rest: String = chars.clone().take(t.len() - 1).collect();
                    let last = chars
                        .last()
                        .expect("Closing parenthesis to exist")
                        .to_string();
                    vec![rest.parse::<MathToken>(), last.parse::<MathToken>()]
                } else {
                    vec![t.parse::<MathToken>()]
                }
            })
            .collect();

        toks.map_or_else(Err, |tokens| Ok(Self { tokens }))
    }
}

#[allow(dead_code)]
impl Expression {
    pub fn new(tokens: Vec<MathToken>) -> Self {
        Self { tokens }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_from_str() {
        assert_eq!("/".parse::<OperatorType>().unwrap(), OperatorType::Div);
        assert_eq!("*".parse::<OperatorType>().unwrap(), OperatorType::Mul);
        assert_eq!("-".parse::<OperatorType>().unwrap(), OperatorType::Sub);
        assert_eq!("+".parse::<OperatorType>().unwrap(), OperatorType::Add);
        assert_eq!("^".parse::<OperatorType>().unwrap(), OperatorType::Pow);
    }
    #[test]
    fn test_fail_operator_from_str() {
        assert!("--".parse::<OperatorType>().is_err());
    }
    #[test]
    fn test_mathtoken_from_str() {
        assert_eq!(
            "/".parse::<MathToken>().unwrap(),
            MathToken::Operator(OperatorType::Div)
        );
        assert_eq!(
            "*".parse::<MathToken>().unwrap(),
            MathToken::Operator(OperatorType::Mul)
        );
        assert_eq!(
            "-".parse::<MathToken>().unwrap(),
            MathToken::Operator(OperatorType::Sub)
        );
        assert_eq!(
            "+".parse::<MathToken>().unwrap(),
            MathToken::Operator(OperatorType::Add)
        );
        assert_eq!(
            "10".parse::<MathToken>().unwrap(),
            MathToken::IntOperand(10)
        );
        assert_eq!(
            "^".parse::<MathToken>().unwrap(),
            MathToken::Operator(OperatorType::Pow)
        );
    }
    #[test]
    fn test_fail_mathtoke_from_str() {
        assert!("--".parse::<MathToken>().is_err());
    }
    #[test]
    fn test_equation_from_str() {
        let input = "3 + 4";
        assert!(input.parse::<Expression>().is_ok());
        assert_eq!(
            input.parse::<Expression>().unwrap(),
            Expression {
                tokens: vec![
                    MathToken::IntOperand(3),
                    MathToken::Operator(OperatorType::Add),
                    MathToken::IntOperand(4),
                ]
            }
        );
    }
    #[test]
    fn test_expression_parentheses() {
        let input = "(3 + 4) * 2";
        assert_eq!(
            input.parse::<Expression>().unwrap(),
            Expression::new(vec![
                MathToken::Parens(true),
                MathToken::IntOperand(3),
                MathToken::Operator(OperatorType::Add),
                MathToken::IntOperand(4),
                MathToken::Parens(false),
                MathToken::Operator(OperatorType::Mul),
                MathToken::IntOperand(2),
            ])
        );
    }
    #[test]
    fn test_expression_power() {
        let input = "3 ^ 4";
        assert_eq!(
            input.parse::<Expression>().unwrap(),
            Expression::new(vec![
                MathToken::IntOperand(3),
                MathToken::Operator(OperatorType::Pow),
                MathToken::IntOperand(4)
            ])
        );
    }
}
