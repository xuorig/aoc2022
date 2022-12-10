use std::str::FromStr;

use itertools::Itertools;

fn main() {
    let input = include_str!("../inputs/10.txt");
    let mut cycles = vec![];
    let mut current_value = 1;

    input
        .lines()
        .map(|l| l.parse::<Command>().unwrap())
        .for_each(|c| {
            match c {
                Command::Addx(n) => {
                    cycles.push(current_value.clone());
                    cycles.push(current_value.clone());
                    current_value += n;
                }
                Command::Noop => {
                    cycles.push(current_value.clone());
                }
            }
        });

    let sum = [20,60,100,140,180,220].iter().fold(0, |mut sum, i| {
        sum += i * cycles[*i as usize-1];
        sum
    });

    println!("SUM: {}", sum);

    for chunk in &cycles.iter().chunks(40) {
        let mut row = String::new();

        for (i,v) in chunk.enumerate() {
            if (v-1..v+2).contains(&(i as i64)) {
                row.push('#');
            } else {
                row.push('.');
            }
        }

        println!("{:?}", row);
    }
}

enum Command {
    Addx(i64),
    Noop,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let command = split.next().expect("Expected a command name");

        match command {
            "addx" => {
                let amount = split.next().expect("Expected a number with addx");
                let int_amount = amount.parse::<i64>().expect("Expected addx argument to be an i64");
                Ok(Command::Addx(int_amount))
            },
            "noop" => Ok(Command::Noop),
            _ => unreachable!(),
        }
    }
}
