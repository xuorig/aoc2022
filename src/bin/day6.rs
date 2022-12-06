use std::collections::HashMap;

pub fn main() {
    let input = include_str!("../inputs/6.txt");
    let mut window = HashMap::new();

    for (i, c) in input.trim().chars().enumerate() {
        *window.entry(c).or_insert(0) += 1;

        if i >= 14 {
            let cc = input.chars().nth(i-14).unwrap();
            window.entry(cc).and_modify(|val| *val -= 1);

            if window[&cc] == 0 {
                window.remove(&cc);
            }

            let four_uniques = window.iter().all(|(_, v)| {
                *v == 1
            });

            if four_uniques {
                println!("Index: {}", i+1);
                break;
            }
        }
    }
}
