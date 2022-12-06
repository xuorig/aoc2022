use std::collections::HashSet;

use itertools::Itertools;

pub fn main() {
    part1();
    part2();
}

pub fn part1() {
    let input = include_str!("../inputs/3.txt");

    let mut total_sum: u32 = 0;

    for line in input.lines() {
        let items: Vec<u8> = line.chars().map(|c| {
            let code = c as u8;

            if c.is_ascii_uppercase() {
                27 + code - b'A'
            } else {
                1 + code - b'a'
            }
        }).collect();

        let midpoint = items.len() / 2;
        let mut first_part = HashSet::new();

        for (i, item) in items.iter().enumerate() {
            if i < midpoint {
                first_part.insert(item.clone());
            } else if first_part.contains(item) {
                total_sum += item.clone() as u32;
                break;
            }
        }
    }

    println!("TOTAL SUM: {}", total_sum);

}


pub fn part2() {
    let input = include_str!("../inputs/3.txt");

    let sum = input
        .lines()
        .tuples()
        .map(|(a, b, c)| {
            a.chars().find(|item| b.chars().contains(item) && c.chars().contains(item)).map(|item| {
                let code = item as u8;

                if item.is_ascii_uppercase() {
                    (27 + code - b'A') as usize
                } else {
                    (1 + code - b'a') as usize
                }
            }).unwrap_or_default()
        })
        .sum::<usize>();

    println!("TOTAL SUM: {}", sum)
}
