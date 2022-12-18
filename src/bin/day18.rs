use std::{str::FromStr, collections::HashSet};

const SIDES: [Coord; 6] = [
    (1,0,0),
    (-1,0,0),
    (0,1,0),
    (0,-1,0),
    (0,0,1),
    (0,0,-1),
];

fn main() {
    let input = include_str!("../inputs/18.txt");
    let cubes: HashSet<Coord> = input
        .lines()
        .map(|l| {
            parse_cube(l)
        })
        .collect();

    let sides: usize = cubes
        .iter()
        .map(|c| {
            SIDES
                .iter()
                .filter(|(dx, dy, dz)| {
                    !cubes.contains(&(c.0 + dx, c.1 + dy, c.2 + dz))
                })
                .collect::<Vec<&Coord>>()
                .len()
        })
        .sum();

    println!("Sides: {}", sides);
}

type Coord = (i32,i32,i32);

fn parse_cube(i: &str) -> Coord {
    let mut coords = i.split(",");
    let x = coords.next().unwrap().parse::<i32>().unwrap();
    let y = coords.next().unwrap().parse::<i32>().unwrap();
    let z = coords.next().unwrap().parse::<i32>().unwrap();
    (x, y, z)
}
