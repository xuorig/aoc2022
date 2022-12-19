use std::collections::{HashSet, VecDeque};

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
    part1(input);
    part2(input);
}

fn part1(input: &str) {
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

fn part2(input: &str) {
    let cubes: HashSet<Coord> = input
        .lines()
        .map(|l| {
            parse_cube(l)
        })
        .collect();

    let flood_filled = flood_fill(&cubes);

    println!("Flood: {:?}", flood_filled);

    let sides: usize = cubes
        .iter()
        .map(|c| {
            SIDES
                .iter()
                .map(|(dx, dy, dz)| (c.0 + dx, c.1 + dy, c.2 + dz))
                .filter(|c| !cubes.contains(&c))
                .collect::<Vec<Coord>>()
                .len()
        })
        .sum();    

    println!("Sides: {}", sides);

    let sides: usize = cubes
        .iter()
        .map(|c| {
            SIDES
                .iter()
                .map(|(dx, dy, dz)| (c.0 + dx, c.1 + dy, c.2 + dz))
                .filter(|c| !cubes.contains(&c) && flood_filled.contains(&c))
                .collect::<Vec<Coord>>()
                .len()
        })
        .sum();    

    println!("Sides: {}", sides);
}

fn flood_fill(cubes: &HashSet<Coord>) -> HashSet<Coord> {
    let bounds = bounds(cubes);
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    // Lets start 2 away from the actual cube bounds
    let start = (bounds.max_x + 2, bounds.max_y + 2, bounds.max_z + 2);
    queue.push_back(start);

    while !queue.is_empty() {
        println!("{}", queue.len());

        let c = queue.pop_front().unwrap();

        if !bounds.is_outside(&c) && !cubes.contains(&c) {
            visited.insert(c);

            for neighbor in SIDES
                .iter()
                    .map(|(dx, dy, dz)| (c.0 + dx, c.1 + dy, c.2 + dz)) {
                            if !queue.contains(&neighbor) && !visited.contains(&neighbor) {
                                queue.push_back(neighbor)
                            }
                    }
        }

    }

    return visited
}

fn bounds(cubes: &HashSet<Coord>) -> Bounds {
    let mut max_z = i32::MIN;
    let mut min_z = i32::MAX;
    let mut max_y = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_x = i32::MAX;

    for c in cubes {
        max_z = i32::max(max_z, c.2);
        min_z = i32::min(min_z, c.2);
        max_y = i32::max(max_y, c.1);
        min_y = i32::min(min_y, c.1);
        max_x = i32::max(max_x, c.0);
        min_x = i32::min(min_x, c.0);
    }

    Bounds {
        max_z,
        min_z,
        max_y,
        min_y,
        max_x,
        min_x,
    }
}

#[derive(Debug)]
struct Bounds {
    max_z: i32,
    min_z: i32,
    max_y: i32,
    min_y: i32,
    max_x: i32,
    min_x: i32,
}

impl Bounds {
    fn is_outside(&self, cube: &Coord) -> bool {
        // Artificially extend bounds for flood fill
        cube.0 > self.max_x + 2 ||
        cube.0 < self.min_x - 2 ||
        cube.1 > self.max_y + 2 ||
        cube.1 < self.min_y - 2  ||
        cube.2 > self.max_z + 2 ||
        cube.2 < self.min_z - 2
    }
}

type Coord = (i32,i32,i32);

fn parse_cube(i: &str) -> Coord {
    let mut coords = i.split(",");
    let x = coords.next().unwrap().parse::<i32>().unwrap();
    let y = coords.next().unwrap().parse::<i32>().unwrap();
    let z = coords.next().unwrap().parse::<i32>().unwrap();
    (x, y, z)
}
