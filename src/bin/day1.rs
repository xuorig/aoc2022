use itertools::Itertools;

fn main() {
    let input = include_str!("../inputs/1.txt");

    let groups = input
        .lines()
        .map(|cals| cals.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;

            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }

            sum
        });

    let max = groups.clone().max();
    let top3: u64 = groups.sorted_by_key(|&v| u64::MAX - v).take(3).sum();

    println!("Max Calories: {:?}", max.expect("No max found"));
    println!("Top 3 Sum Calories: {:?}", top3)
}
