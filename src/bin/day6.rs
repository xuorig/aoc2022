use std::collections::HashSet;

const SIZE: usize = 14;

pub fn main() {
    let input = include_str!("../inputs/6.txt");

    let chars: Vec<char> = input.trim().chars().collect();

    for (i, window) in chars.windows(SIZE).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(window.iter());

        if set.len() == SIZE {
            println!("Index: {}", i + SIZE);
            break;
        }
    }
}
