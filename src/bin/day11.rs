use std::{str::FromStr, cell::RefCell};

use nom::{IResult, bytes::complete::{tag, take}, character::complete::{digit1, multispace0, space1}, sequence::{preceded, delimited}, combinator::map_res, multi::separated_list1, branch::alt};

fn main() {
    let input = include_str!("../inputs/11.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let monkeys: Vec<RefCell<Monkey>> = input
        .split("\n\n")
        .map(|monkey| {
            RefCell::new(Monkey::parse(monkey).expect("Failed to parse Mankey").1) 
        })
        .collect();

    let mut processed_items: Vec<usize> = vec![0; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].borrow_mut();

            while let Some(item) = monkey.items.pop() {
                let worry_level = monkey.execute_operation(item);
                let bored_level = worry_level / 3;
                let target_monkey = monkey.test_item(bored_level);
                monkeys[target_monkey].borrow_mut().items.push(bored_level);
                processed_items[i] += 1;
            }
        }
    }

    println!("{:?}", processed_items);
    processed_items.sort();
    let first = processed_items.pop().unwrap();
    let second = processed_items.pop().unwrap();
    println!("{}", first * second);
}

fn part2(input: &str) {
    let monkeys: Vec<RefCell<Monkey>> = input
        .split("\n\n")
        .map(|monkey| {
            RefCell::new(Monkey::parse(monkey).expect("Failed to parse Mankey").1) 
        })
        .collect();

    let mut processed_items: Vec<usize> = vec![0; monkeys.len()];
    let mod_product = monkeys.iter().map(|m| m.borrow().test.val).product::<u64>();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].borrow_mut();

            for item in &monkey.items {
                let new_item = monkey.execute_operation(*item) % mod_product;
                let target = monkey.test_item(new_item);
                monkeys[target].borrow_mut().items.push(new_item);
                processed_items[i] += 1;
            }

            monkey.items.clear();
        }
    }

    println!("{:?}", processed_items);
    processed_items.sort();
    let first = processed_items.pop().unwrap();
    let second = processed_items.pop().unwrap();
    println!("{}", first * second);
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    // Returns which monkey to throw this item to
    fn test_item(&self, item: u64) -> usize {
        // Hardcoded to division
        if item % self.test.val == 0 {
            self.test.true_monkey
        } else {
            self.test.false_monkey  
        }
    }

    fn execute_operation(&self, item: u64) -> u64 {
        let left = match self.operation.left.as_str() {
            "old" => item,
            _ => self.operation.left.parse::<u64>().unwrap()
        };
        let right = match self.operation.right.as_str() {
            "old" => item,
            _ => self.operation.right.parse::<u64>().unwrap()
        };

        match self.operation.typ {
            OperationType::Multiply => left * right,
            OperationType::Add => left + right,
            _ => unreachable!()
        }
    }

    fn parse(input: &str) -> IResult<&str, Monkey> {
        let (i, _number) = map_res(delimited(tag("Monkey "), digit1, tag(":")), str::parse::<usize>)(input)?;
        let (i, items) = delimited(multispace0, preceded(tag("Starting items: "), separated_list1(tag(", "), digit1)), multispace0)(i)?;
        let (i, operation_left) = delimited(multispace0, preceded(tag("Operation: new = "), alt((tag("old"), digit1))), multispace0)(i)?;
        let (i, operation_type) = map_res(take(1_usize), str::parse::<OperationType>)(i)?;
        let (i, operation_right) = preceded(space1, alt((tag("old"), digit1)))(i)?;
        let (i, test) = map_res(delimited(multispace0, preceded(tag("Test: divisible by "), digit1), multispace0), str::parse::<u64>)(i)?;
        let (i, monkey_true) = map_res(delimited(multispace0, preceded(tag("If true: throw to monkey "), digit1), multispace0), str::parse::<usize>)(i)?;
        let (_, monkey_false) = map_res(delimited(multispace0, preceded(tag("If false: throw to monkey "), digit1), multispace0), str::parse::<usize>)(i)?;

        let items = items.iter().map(|i| {
            i.parse::<u64>().unwrap()
        }).collect();

        let monkey = Monkey {
            items,
            operation: Operation {
                typ: operation_type,
                left: operation_left.to_string(),
                right: operation_right.to_string()
            },
            test: Test {
                val: test,
                true_monkey: monkey_true,
                false_monkey: monkey_false
            }
        };

        println!("{:?}", monkey);

        Ok(("",  monkey))
    }
}

#[derive(Debug, Clone)]
struct Operation {
    typ: OperationType,
    left: String,
    right: String
}

#[derive(Debug, Clone)]
enum OperationType {
    Divide,
    Multiply,
    Add,
    Substract,
}

impl FromStr for OperationType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(OperationType::Multiply),
            "/" => Ok(OperationType::Divide),
            "+" => Ok(OperationType::Add),
            "-" => Ok(OperationType::Substract),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
struct Test {
    val: u64,
    true_monkey: usize,
    false_monkey: usize
}
