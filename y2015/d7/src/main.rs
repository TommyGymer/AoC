use std::{error::Error, fmt::Display, fs};

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_till, take_until, take_while},
    combinator::map_res,
};

#[derive(Debug)]
enum Operator {
    AND,
    OR,
    NOT,
    LSHIFT,
    RSHIFT,
    ASSIGN,
}

#[derive(Debug)]
enum ParseOperatorError {
    UnknownOperator(String),
}

impl Display for ParseOperatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownOperator(value) => writeln!(f, "{} is an unknown operator", value),
        }
    }
}

impl Error for ParseOperatorError {}

impl Operator {
    fn from_str(value: &str) -> Result<Self, ParseOperatorError> {
        match value {
            "AND" => Ok(Operator::AND),
            "OR" => Ok(Operator::OR),
            "NOT" => Ok(Operator::NOT),
            "LSHIFT" => Ok(Operator::LSHIFT),
            "RSHIFT" => Ok(Operator::RSHIFT),
            _ => Err(ParseOperatorError::UnknownOperator(value.to_owned())),
        }
    }
}

fn operator(input: &str) -> IResult<&str, Operator> {
    map_res(take_till(|c| c == ' ' || c == '\n'), Operator::from_str).parse(input)
}

#[derive(Debug)]
struct Variable {
    name: String,
}

impl Variable {
    fn from_str(input: &str) -> Self {
        Self {
            name: input.to_owned(),
        }
    }
}

fn variable(input: &str) -> IResult<&str, Variable> {
    take_till(|c| c == ' ' || c == '\n')
        .map(Variable::from_str)
        .parse(input)
}

#[derive(Debug)]
enum Operand {
    VAR(Variable),
    LIT(u16),
}

enum OperandParseError {}

impl Operand {
    fn from_str(input: &str) -> Result<Operand, OperandParseError> {
        if let Ok(i) = u16::from_str_radix(input, 10) {
            Ok(Operand::LIT(i))
        } else {
            Ok(Operand::VAR(Variable::from_str(input)))
        }
    }
}

fn operand(input: &str) -> IResult<&str, Operand> {
    map_res(take_till(|c| c == ' ' || c == '\n'), Operand::from_str).parse(input)
}

#[derive(Debug)]
struct Expr {
    operator: Operator,
    operands: Vec<Operand>,
    destination: Variable,
}

fn assign_expr(input: &str) -> IResult<&str, Expr> {
    let (input, a) = operand(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, dest) = variable(input)?;

    Ok((
        input,
        Expr {
            operator: Operator::ASSIGN,
            operands: vec![],
            destination: dest,
        },
    ))
}

fn single_operand_expr(input: &str) -> IResult<&str, Expr> {
    let (input, op) = operator(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, a) = operand(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, dest) = variable(input)?;

    Ok((
        input,
        Expr {
            operator: op,
            operands: vec![a],
            destination: dest,
        },
    ))
}

fn double_operand_expr(input: &str) -> IResult<&str, Expr> {
    let (input, a) = operand(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, op) = operator(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, b) = operand(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, dest) = variable(input)?;

    Ok((
        input,
        Expr {
            operator: op,
            operands: vec![a, b],
            destination: dest,
        },
    ))
}

fn expr(input: &str) -> IResult<&str, Expr> {
    alt((single_operand_expr, double_operand_expr, assign_expr)).parse(input)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<&str> = input.split('\n').filter(|line| line.len() > 0).collect();

    let exprs: Vec<Expr> = lines
        .into_iter()
        .enumerate()
        .filter_map(|(i, line)| match expr(line) {
            Ok((_, e)) => Some(e),
            Err(error) => {
                println!("line {}: {:?}", i, error);
                None
            }
        })
        .collect();
}
