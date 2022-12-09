use core::str::FromStr;
use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    let input = include_str!("../inputs/9.txt");
    let motions: Vec<Motion> = input.lines().map(|l| l.parse::<Motion>().unwrap()).collect();

    part1(motions.clone());
    part2(motions);
}

fn part1(motions: Vec<Motion>) {
    let mut current_head_position = Position::new(0,0);
    let mut current_tail_position = Position::new(0,0);
    let mut positions = HashSet::new();
    positions.insert(current_head_position.clone());

    for m in motions {
        for _ in 0..m.length {
            current_head_position.walk(&m.direction);

            if !current_head_position.is_touching(&current_tail_position) {
                current_tail_position.follow(&current_head_position);
            }

            positions.insert(current_tail_position.clone());
        }
    }

    println!("{}", positions.len());
}

fn part2(motions: Vec<Motion>) {
    let mut positions = vec![Position::new(0,0); 10];
    let mut tail_positions: HashSet<Position> = HashSet::new();

    for m in motions {
        for _ in 0..m.length {
            positions[9].walk(&m.direction);

            for i in (0..9).rev() {
                let following = positions[i + 1].clone();
                let follower = &mut positions[i];
                if !follower.is_touching(&following) {
                    follower.follow(&following);
                }
            }

            tail_positions.insert(positions[0].clone());
        }
    }

    println!("{}", tail_positions.len());
}

#[derive(Debug, Clone)]
struct Motion {
    direction: Direction,
    length: usize
}

impl FromStr for Motion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split_whitespace();
        let direction = splitted.next().unwrap().parse::<Direction>().unwrap();
        let length = splitted.next().unwrap().parse::<usize>().unwrap();

        Ok(Motion {
            direction,
            length
        })
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..1] {
            "U" => Ok(Direction::Up),
            "R" => Ok(Direction::Right),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn is_touching(&self, other: &Position) -> bool {
        i32::abs(self.x - other.x) < 2 && i32::abs(self.y - other.y) < 2
    }

    fn walk(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => {
                self.y += 1;
            },
            Direction::Right => {
                self.x += 1;
            },
            Direction::Down => {
                self.y -= 1;
            },
            Direction::Left => {
                self.x -= 1;
            },
        }
    }

    fn follow(&mut self, other: &Position) {
        let diff_x = other.x - self.x;
        let diff_y = other.y - self.y;

        if diff_x > 0 {
            self.x += 1;
        } else if diff_x < 0 {
            self.x -= 1;
        }

        if diff_y > 0 {
            self.y += 1;
        } else if diff_y < 0 {
            self.y -= 1;
        }

    }
}
