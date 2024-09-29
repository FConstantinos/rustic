use crate::ast::*;
use pest::error::Error;
use pest::Parser;

// The pest parser for Rust

#[derive(Parser)]
#[grammar = "rust.pest"]
pub struct RustParser;

// Functions to parse a Rust code string into a Rust AST

pub fn parse(source: &str) -> Result<Program, Error<Rule>> {
    let mut name = String::new();
    let mut inputs = Vec::new();
    let mut statements = Vec::new();

    let pairs = RustParser::parse(Rule::program, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::fn_header => {
                let mut inner_pairs = pair.into_inner();

                // Parse fn name
                name = inner_pairs.next().unwrap().as_str().to_string();

                // Parse fn inputs if any
                for inner_pair in inner_pairs {
                    match inner_pair.as_rule() {
                        Rule::input => {
                            inputs.push(parse_single_input(inner_pair));
                        }
                        _ => {}
                    }
                }
            }
            Rule::statement => {
                statements.push(parse_statement(pair.into_inner().next().unwrap()));
            }
            _ => {}
        }
    }

    Ok(Program {
        name,
        inputs,
        statements,
    })
}

fn parse_single_input(pair: pest::iterators::Pair<Rule>) -> Input {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();
    let input_type = parse_type(inner.next().unwrap());

    Input { name, input_type }
}

fn parse_type(pair: pest::iterators::Pair<Rule>) -> Type {
    match pair.as_str() {
        "u8" => Type::U8,
        _ => panic!("failed to parse type"),
    }
}

fn parse_statement(pair: pest::iterators::Pair<Rule>) -> Statement {
    match pair.as_rule() {
        Rule::assign => {
            let mut pair = pair.into_inner();

            let variable = pair.next().unwrap().as_str().to_string();
            let expression = parse_expression(pair.next().unwrap());

            Statement::Assign {
                variable,
                expression,
            }
        }
        _ => panic!("failed to parse statement"),
    }
}

fn parse_expression(pair: pest::iterators::Pair<Rule>) -> Expression {
    parse_addition(pair.into_inner().next().unwrap())
}

fn parse_addition(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut inner_pairs = pair.into_inner();

    // The initial left-hand side is a multiplication
    let mut expr = parse_multiplication(inner_pairs.next().unwrap());

    // Loop over any additional (operator multiplication) pairs
    while inner_pairs.peek().is_some() {
        let operator = parse_operator(inner_pairs.next().unwrap());
        let right_expr = parse_multiplication(inner_pairs.next().unwrap());
        expr = Expression::Binary {
            left: Value::Expression(Box::new(expr)),
            operator,
            right: Box::new(right_expr),
        };
    }

    expr
}

fn parse_multiplication(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut inner_pairs = pair.into_inner();

    // The initial left-hand side is a value
    let mut expr = Expression::Value(Box::new(parse_value(inner_pairs.next().unwrap())));

    // Loop over any additional (operator value) pairs
    while let Some(op_pair) = inner_pairs.next() {
        let operator = parse_operator(op_pair);
        let right_expr = parse_value(inner_pairs.next().unwrap());
        expr = Expression::Binary {
            left: Value::Expression(Box::new(expr)),
            operator,
            right: Box::new(Expression::Value(Box::new(right_expr))),
        };
    }

    expr
}

fn parse_value(pair: pest::iterators::Pair<Rule>) -> Value {
    match pair.as_rule() {
        Rule::integer => {
            // Parse the integer and trim the value type
            let int_str = pair.as_str();
            let int_len = int_str.len();
            let integer = &int_str[..int_len - 2].parse::<u8>().unwrap();

            Value::Integer(*integer)
        }
        Rule::ident => {
            let ident = pair.as_str().to_string();

            Value::Identifier(ident)
        }
        Rule::expression => {

            let expression = parse_expression(pair);

            Value::Expression(Box::new(expression))
        }
        _ => panic!("failed to parse value: {}", pair.as_str()),
    }
}

fn parse_operator(pair: pest::iterators::Pair<Rule>) -> Operator {
    match pair.as_str() {
        "+" => Operator::Add,
        "-" => Operator::Subtract,
        "*" => Operator::Multiply,
        "/" => Operator::Divide,
        _ => panic!("failed to parse operator"),
    }
}
