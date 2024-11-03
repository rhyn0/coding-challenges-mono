use log::debug;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum OperatorType {
    Add,
    Sub,
    Div,
    Mul,
}

impl OperatorType {
    pub const fn apply(&self, lhs: isize, rhs: isize) -> isize {
        match self {
            Self::Add => lhs + rhs,
            Self::Div => lhs / rhs,
            Self::Sub => lhs - rhs,
            Self::Mul => lhs * rhs,
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
            _ => Err(MathEquationErr::InvalidOperatorType(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MathToken {
    IntOperand(isize),
    // FloatOperand(f64),
    Operator(OperatorType),
}

impl FromStr for MathToken {
    type Err = MathEquationErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // if let Ok(f) = s.parse::<f64>() {
        //     return Ok(Self::FloatOperand(f));
        // } else
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
        let toks: Result<Vec<_>, _> = s.split_whitespace().map(str::parse).collect();

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
        let div = "/";
        let mul = "*";
        let sub = "-";
        let add = "+";
        assert_eq!(div.parse::<OperatorType>().unwrap(), OperatorType::Div);
        assert_eq!(mul.parse::<OperatorType>().unwrap(), OperatorType::Mul);
        assert_eq!(sub.parse::<OperatorType>().unwrap(), OperatorType::Sub);
        assert_eq!(add.parse::<OperatorType>().unwrap(), OperatorType::Add);
    }
    #[test]
    fn test_fail_operator_from_str() {
        assert!("--".parse::<OperatorType>().is_err());
    }
    #[test]
    fn test_mathtoken_from_str() {
        let div = "/";
        let mul = "*";
        let sub = "-";
        let add = "+";
        assert_eq!(
            div.parse::<MathToken>().unwrap(),
            MathToken::Operator(OperatorType::Div)
        );
        assert_eq!(
            mul.parse::<MathToken>().unwrap(),
            MathToken::Operator(OperatorType::Mul)
        );
        assert_eq!(
            sub.parse::<MathToken>().unwrap(),
            MathToken::Operator(OperatorType::Sub)
        );
        assert_eq!(
            add.parse::<MathToken>().unwrap(),
            MathToken::Operator(OperatorType::Add)
        );
        assert_eq!(
            "10".parse::<MathToken>().unwrap(),
            MathToken::IntOperand(10)
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
    }
    #[test]
    fn test_equation_postfix() {
        let input = "3 + 4";
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
}
