use std::collections::HashMap;

use nom::{IResult, bytes::complete::{tag, take_until, take}, character::complete::{digit1, space1}, sequence::{preceded, delimited}, branch::alt, combinator::map};

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("../inputs/21.txt");
    let commands: HashMap<String, Operation> = input
        .lines()
        .map(|l| parse_command(l).unwrap().1 )
        .map(|c| (c.monkey, c.op) )
        .collect();

    let root_command = &commands["root".into()];
    let res = root_command.evaluate(&commands);

    println!("{}", res);
}

fn part2() {
    let input = include_str!("../inputs/21.txt");

    let commands: HashMap<String, Operation> = input
        .lines()
        .map(|l| parse_command(l).unwrap().1 )
        .map(|c| (c.monkey, c.op) )
        .collect();

    let root = &commands["root"];

    if let Operation::Op(((a,b), _op)) = root {
        let solved_left = commands[a].solve(&commands);
        let solved_right = commands[b].solve(&commands);

        if let Algebra::Val(mut x) = solved_right {
            if let Algebra::VariableOperations(mut operations) = solved_left {
                while let Some(op) = operations.pop() {
                    match op {
                        VariableOperation::Left((op, val)) => match op {
                            OperationType::Add => x = x + val,
                            OperationType::Sub => x = x - val,
                            OperationType::Mul => x = x * val,
                            OperationType::Div => x = x / val,
                        },
                        VariableOperation::Right((op, val)) => match op {
                            OperationType::Add => x = val + x,
                            OperationType::Sub => x = val - x,
                            OperationType::Mul => x = val * x,
                            OperationType::Div => x = val / x,
                        },
                    }
                }
            }

            println!("RES: {}", x);
        }

    } else {
        todo!()
    }
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (i, monkey) = take_until(":")(input)?;
    let (i, operation) = preceded(
        tag(": "),
        parse_operation
    )(i)?;

    return Ok((
    i,
        Command {
            monkey: monkey.into(),
            op: operation
        }
    ))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((
        map(digit1, |x: &str| Operation::Constant(x.parse::<i64>().unwrap())),
        parse_computation
    ))(input)
}

fn parse_computation(input: &str) -> IResult<&str, Operation> {
    let (i, monkey_one) = map(take(4usize), String::from)(input)?;
    let (i, sign) = delimited(space1, take(1usize), space1)(i)?;
    let (i, monkey_two) = map(take(4usize), String::from)(i)?;

    let operation_type = match sign {
        "+" => OperationType::Add,
        "-" => OperationType::Sub,
        "*" => OperationType::Mul,
        "/" => OperationType::Div,
        _ => panic!("Unsupported operation")
    };

    Ok((i, Operation::Op(((monkey_one.clone(), monkey_two.clone()), operation_type))))
}

#[derive(Debug)]
struct Command {
    monkey: String,
    op: Operation,
}

#[derive(Debug, Clone)]
enum Operation {
    Constant(i64),
    Op(((String, String), OperationType))
}

#[derive(Debug, Clone)]
enum OperationType {
    Add,
    Sub,
    Mul,
    Div
}

impl Operation {
    fn evaluate(&self, defs: &HashMap<String, Self>) -> i64 {
        match self {
            Operation::Constant(x) => *x,
            Operation::Op(((a, b), op)) => {
                let a = defs[a].evaluate(defs);
                let b = defs[b].evaluate(defs);
                
                match op {
                    OperationType::Add => a + b,
                    OperationType::Sub => a - b,
                    OperationType::Mul => a * b,
                    OperationType::Div => a / b,
                }
            },
        }
    }

    fn solve(&self, defs: &HashMap<String, Self>) -> Algebra {
        match self {
            Operation::Constant(x) => Algebra::Val(*x),
            Operation::Op(((a, b), op)) => {
                let a = if a == "humn" {
                    Algebra::VariableOperations(vec![])
                } else {
                    defs[a].solve(defs)
                };

                let b = defs[b].solve(defs);

                match (a, b) {
                    (Algebra::Val(x), Algebra::Val(y)) => match op {
                        OperationType::Add => Algebra::Val(x + y),
                        OperationType::Sub => Algebra::Val(x - y),
                        OperationType::Mul => Algebra::Val(x * y),
                        OperationType::Div => Algebra::Val(x / y),
                    },
                    (Algebra::Val(x), Algebra::VariableOperations(operations)) => match op {
                        OperationType::Add => {
                            let mut operations = operations.clone();
                            operations.push(VariableOperation::Left((OperationType::Sub, x)));
                            Algebra::VariableOperations(operations)
                        },
                        OperationType::Sub => {
                            let mut operations = operations.clone();
                            operations.push(VariableOperation::Right((OperationType::Sub, x)));
                            Algebra::VariableOperations(operations)
                        },
                        OperationType::Mul => {
                            let mut operations = operations.clone();
                            operations.push(VariableOperation::Left((OperationType::Div, x)));
                            Algebra::VariableOperations(operations)
                        },
                        OperationType::Div => {
                            let mut operations = operations.clone();
                            operations.push(VariableOperation::Right((OperationType::Div, x)));
                            Algebra::VariableOperations(operations)
                        },
                    },
                    (Algebra::VariableOperations(operations), Algebra::Val(x)) => match op {
                        OperationType::Add => {
                            let mut operations = operations.clone();
                            operations.push(VariableOperation::Left((OperationType::Sub, x)));
                            Algebra::VariableOperations(operations)
                        },
                        OperationType::Sub => {
                            let mut operations = operations.clone();
                            operations.push(VariableOperation::Left((OperationType::Add, x)));
                            Algebra::VariableOperations(operations)
                        },
                        OperationType::Mul => {
                            let mut operations = operations.clone();
                            operations.push(VariableOperation::Left((OperationType::Div, x)));
                            Algebra::VariableOperations(operations)
                        },
                        OperationType::Div => {
                            let mut operations = operations.clone();
                            operations.push(VariableOperation::Left((OperationType::Mul, x)));
                            Algebra::VariableOperations(operations)
                        },
                    },
                    (Algebra::VariableOperations(_), Algebra::VariableOperations(_)) => unreachable!(),
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
enum VariableOperation {
    // x + 3
    Left((OperationType, i64)),
    // 3 + x
    Right((OperationType, i64)),
}

#[derive(Debug, Clone)]
enum Algebra {
    Val(i64),
    VariableOperations(Vec<VariableOperation>)
}
