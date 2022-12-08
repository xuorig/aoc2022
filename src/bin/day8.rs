use std::collections::HashSet;

fn main() {
    let input = include_str!("../inputs/8.txt");
    let trees: Vec<Vec<i32>> = input
        .trim()
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| {
                    c.to_string().parse::<i32>().unwrap()
                })
                .collect()
        })
        .collect();

    part1(trees.clone());
    part2(trees);
}

fn part1(trees: Vec<Vec<i32>>) {
    let columns = trees.first().unwrap().len();
    let mut highest_left;
    let mut highest_right;
    let mut highest_tops: Vec<i32> = vec![-1; columns];
    let mut highest_bottoms: Vec<i32> = vec![-1; columns];

    let mut visible = HashSet::new();

    for (i, row) in trees.iter().enumerate() {
        highest_left = -1;

        for (j, size) in row.iter().enumerate() {
            // Visible from left
            if *size > highest_left {
                visible.insert((i,j));
                highest_left = *size;
            }

            // Visible from top
            if *size > highest_tops[j] {
                visible.insert((i,j));
                highest_tops[j] = *size;
            }
        }
    }

    for (i, row) in trees.iter().enumerate().rev() {
        highest_right = -1;

        for (j, size) in row.iter().enumerate().rev() {
            // Visible from right
            if *size > highest_right {
                visible.insert((i,j));
                highest_right = *size;
            }

            // Visible from bottom
            if *size > highest_bottoms[j] {
                visible.insert((i,j));
                highest_bottoms[j] = *size;
            }
        }
    }

    println!("{:?}", visible.len());
}

fn part2(trees: Vec<Vec<i32>>) {
    let mut max_product = 0;
    
    for (i, row) in trees.iter().enumerate() {
        for (j, size) in row.iter().enumerate() {
            let mut top = 0;
            let mut right = 0;
            let mut down = 0;
            let mut left = 0;

            for k in (0..i).rev() {
                top += 1;
                if *size <= trees[k][j] {
                    break;
                }
            }

            for k in i+1..trees.len() {
                down += 1;
                if *size <= trees[k][j] {
                    break;
                }
            }

            for k in (0..j).rev() {
                left += 1;
                if *size <= trees[i][k] {
                    break;
                }
            }

            for k in j+1..trees[i].len() {
                right += 1;
                if *size <= trees[i][k] {
                    break;
                }
            }

            let product = top * right * down * left;
            if product > max_product {
                max_product = product
            }
        }
    }

    println!("MAX: {}", max_product);
}
