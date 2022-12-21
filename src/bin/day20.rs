use std::collections::VecDeque;

fn main() {
    let input = include_str!("../inputs/20.txt");

    let original: Vec<i64> = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap() * 811589153)
        .collect();

    let mixed = mix(&original);

    let zero_index = mixed.iter().position(|x| *x == 0).expect("0 must be in numbers");

    let coords = vec![
        mixed[(zero_index + 1000).rem_euclid(mixed.len())],
        mixed[(zero_index + 2000).rem_euclid(mixed.len())],
        mixed[(zero_index + 3000).rem_euclid(mixed.len())],
    ];

        println!("{}", coords.iter().sum::<i64>());
}

fn mix(original: &Vec<i64>) -> VecDeque<i64> {
    let mut mixed: VecDeque<(usize, i64)> = original
        .iter()
        .cloned()
        .enumerate()
        .collect();

    for _ in 0..10 {
        for (original_index, _) in original.iter().enumerate() {
            let current_index = mixed
                .iter()
                .position(|(orig, _)| *orig == original_index)
                .unwrap();

            let val = mixed[current_index];

            let new_index = (current_index as i64 + val.1 - 1).rem_euclid(mixed.len() as i64 - 1) + 1;

            let val = mixed.remove(current_index).unwrap();
            mixed.insert(new_index as usize, val);

        }
    }


    mixed.iter().map(|x| x.1).collect()
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::mix;

    #[test]
    fn it_works() {
        let input = "1\n2\n-3\n3\n-2\n0\n4";

        let original: Vec<i64> = input
            .lines()
            .map(|l| l.parse::<i64>().unwrap())
            .collect();

        let mixed = mix(&original);
        assert_eq!(VecDeque::from([1, 2, -3, 4, 0, 3, -2]), mixed);
    }
}
