use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

fn main() {
    let input = include_str!("../inputs/12.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut grid = vec![];
    let mut root = (0,0);
    let mut end = (0,0);

    for (i, line) in input.lines().enumerate() {
        grid.push(vec![]);

        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                root = (i, j);
            }
            if c == 'E' {
                end = (i, j);
            }
            grid[i].push(c);
        }
    }

    let shortest_path = run_astar(root, end, &grid);
    // Minus the start pos
    println!("{}", shortest_path.len() - 1);
}

fn part2(input: &str) {
    let mut grid = vec![];
    let mut starts = vec![];
    let mut end = (0,0);

    for (i, line) in input.lines().enumerate() {
        grid.push(vec![]);

        for (j, c) in line.chars().enumerate() {
            if c == 'S' || c == 'a' {
                starts.push((i, j));
            }
            if c == 'E' {
                end = (i, j);
            }
            grid[i].push(c);
        }
    }

    let shortest = starts.iter().map(|s| run_astar(*s, end, &grid)).filter(|path| path.len() > 0).map(|path| path.len() - 1).min();
    println!("{:?}", shortest);
}

fn run_astar(start: Pos, end: Pos, grid: &Vec<Vec<char>>) -> Vec<Pos> {
    let mut came_from = HashMap::<Pos, Pos>::new();

    let mut cost_so_far = HashMap::<Pos, u32>::new();
    cost_so_far.insert(start, 0);

    let mut open = BinaryHeap::new();
    open.push(PosWithPriority {
        pos: start,
        priority: 0,
    });


    while !open.is_empty() {
        let current = open.pop().unwrap();

        if current.pos == end {
            return reconstruct_path(end, &came_from);
        }

        for neighbor in accessible_neighbors(&grid, current.pos) {
            let new_cost = cost_so_far.get(&current.pos).unwrap() + 1;
            let neighbhor_cost = cost_so_far.get(&neighbor);

            if neighbhor_cost.is_none() || &new_cost < neighbhor_cost.unwrap() {
                cost_so_far.insert(neighbor, new_cost);
                let priority = new_cost + heuristic(neighbor, end) as u32;
                open.push(PosWithPriority { pos: neighbor, priority });
                came_from.insert(neighbor, current.pos);
            }
        }
    }

    vec![]
}

fn reconstruct_path(
    end: Pos,
    came_from: &HashMap<Pos, Pos>,
    ) -> Vec<Pos> {
    let mut path = vec![];
    path.push(end);

    let mut current = &end;

    while let Some(next) = came_from.get(&current) {
        current = next;
        path.push(next.clone());
    }

    path.reverse();
    path
}


// Manhattan distance
pub fn heuristic(a: Pos, b: Pos) -> i32 {
    let x1: i32 = a.0.try_into().unwrap();
    let y1: i32 = a.1.try_into().unwrap();
    let x2: i32 = b.0.try_into().unwrap();
    let y2: i32 = b.1.try_into().unwrap();
    i32::abs(x1 - x2) + i32::abs(y1 - y2)
}

pub fn accessible_neighbors(grid: &Vec<Vec<char>>, pos: Pos) -> Vec<Pos> {
    let neighbor_deltas: Vec<(i32, i32)> = vec![
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
    ];

    let mut result = vec![];

    for (i, j) in &neighbor_deltas {
        let neighbor_location = (pos.0 as i32 + i, pos.1 as i32 + j);

        let inbounds = neighbor_location.0 >= 0
            && neighbor_location.0 < grid.len().try_into().unwrap()
            && neighbor_location.1 >= 0
            && neighbor_location.1 < grid[0].len().try_into().unwrap();

        if inbounds {
            let x: usize = neighbor_location.0.try_into().unwrap();
            let y: usize = neighbor_location.1.try_into().unwrap();

            let neighbor_char = grid[x][y];
            let our_char = grid[pos.0][pos.1];

            if neighbor_char == 'E' {
                if our_char == 'z' {
                    result.push((x, y));
                }
            } else if our_char == 'S' {
                result.push((x, y));
            } else if neighbor_char as i8 - our_char as i8 <= 1 {
                result.push((x, y));
            }
        }
    }

    result
}

type Pos = (usize, usize);

#[derive(Debug)]
struct PosWithPriority {
    pos: Pos,
    priority: u32
}


impl Ord for PosWithPriority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl PartialOrd for PosWithPriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PosWithPriority {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for PosWithPriority {}
