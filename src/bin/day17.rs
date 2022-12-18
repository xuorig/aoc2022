use std::collections::HashSet;

const CHAMBER_WIDTH: i64 = 7;
const ROCKS_STOP: i64 = 2022;
const ROCKS_STOP_2: i64 = 1000000000000;

fn main() {
    let input = include_str!("../inputs/17.txt");
    part1(input);
    part2(input);
}

// TODO: Find cycles programatically, but this will do for now...
fn part2(input: &str) {
    // After the two first rocks, there is a cycle of lenght(jets).
    // 2647 height is added every 1690 rocks.

    // 1688 rocks before cycle
    // then every 1690 rocks + 2647 height
    // then compute the rest

    let mut height = 2645;
    let remaining = ROCKS_STOP_2 - 1688;
    let cycle_len = 1690;
    height += remaining / cycle_len * 2647;
    let remaining = remaining % cycle_len;
    println!("height: {}", height);
    println!("Remaining: {}", remaining);

    // The rest is part 1 until the start of the cycle + remaining
    // minus the rocks before the cycle.
    println!("height: {}", height + 882);
}

fn part1(input: &str) {
    let jets: Vec<Direction> = input.trim().chars().map(|c| {
        match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => todo!()
        }
    }).collect();
    
    let mut jet_index = 0;
    let mut rocks = 0;
    let mut highest = 0;
    let mut chamber: HashSet<Position> = HashSet::new();
    let mut current_shape = spawn(rocks, highest);

    loop {
        if rocks == ROCKS_STOP {
            println!("HIGHEST {}", highest);
            break;
        }

        let jet = &jets[jet_index % jets.len()];
        current_shape.push(&jet, &chamber);
        jet_index += 1;

        let moved = current_shape.push(&Direction::Down, &chamber);

        if !moved {
            let positions = current_shape.cristalize();
            chamber.extend(&positions);

            let top = current_shape.top() + 1;
            highest = i64::max(top, highest);

            rocks += 1;
            current_shape = spawn(rocks, highest);
        }
    }
}

fn print_chamber(chamber: &HashSet<Position>, current: &Box<dyn Collider>) {
    let positions = current.cristalize();

    for y in (0..200).rev() {
        print!("|");
        for x in 0..CHAMBER_WIDTH {
            if chamber.contains(&(x,y)) {
                print!("#");
            } else if positions.contains(&(x,y)) {
                print!("%");
            } else {
                print!(".");
            }
        }
        print!("|");
        println!();
    }
}

fn spawn(shape_index: i64, highest: i64) -> Box<dyn Collider> {

    match shape_index % 5 {
        0 => Box::new(HorizontalLine { y: highest + 3, x: 2 }),
        1 => Box::new(Plus { y: highest + 4, x: 3 }),
        2 => Box::new(L { y: highest + 5, x: 4 }),
        3 => Box::new(VerticalLine { y: highest + 6, x: 2 }),
        4 => Box::new(Square { y: highest + 4, x: 3 }),
        _ => unreachable!()
    }
}

// An horizontal line with a width of 4
// ####
struct HorizontalLine {
    // track the left most coordinate
    y: i64,
    x: i64
}

impl Collider for HorizontalLine {
    fn push(&mut self, direction: &Direction, occupied: &HashSet<Position>) -> bool {
        match direction {
            Direction::Right => {
                let current_right = self.x + 3;
                let moving_to = current_right + 1;

                if moving_to < CHAMBER_WIDTH && !occupied.contains(&(moving_to, self.y)) {
                    self.x += 1;
                    true
                } else {
                    false
                }
            },
            Direction::Left => {
                let moving_to = self.x - 1;

                if moving_to >= 0 && !occupied.contains(&(moving_to, self.y)) {
                    self.x -= 1;
                    true
                } else {
                    false
                }
            },
            Direction::Down => {
                let moving_to = vec![
                    (self.x, self.y - 1),
                    (self.x + 1, self.y - 1),
                    (self.x + 2, self.y - 1),
                    (self.x + 3, self.y - 1)
                ];

                if moving_to.iter().all(|position| {
                    position.1 >= 0 && !occupied.contains(position)
                }) {
                    self.y -= 1;
                    true
                } else {
                    false
                }
            },
        }
    }

    fn top(&self) -> i64 {
        self.y
    }

    fn cristalize(&self) -> Vec<Position> {
        vec![(self.x,self.y),(self.x+1,self.y),(self.x+2, self.y),(self.x+3, self.y)]
    }
}

// A plus shaped rock
// .#.
// ###
// .#.
struct Plus {
    // track the mid point
    y: i64,
    x: i64
}

impl Collider for Plus {
    fn push(&mut self, direction: &Direction, occupied: &HashSet<Position>) -> bool {
        match direction {
            Direction::Right => {
                let moving_to = vec![
                    (self.x + 1, self.y + 1),
                    (self.x + 2, self.y),
                    (self.x + 1, self.y - 1)
                ];

                if moving_to.iter().all(|position| {
                    position.0 < CHAMBER_WIDTH && !occupied.contains(position)
                }) {
                    self.x += 1;
                    true
                } else {
                    false
                }
            },
            Direction::Left => {
                let moving_to = vec![
                    (self.x - 1, self.y + 1),
                    (self.x -2, self.y),
                    (self.x - 1, self.y - 1)
                ];

                if moving_to.iter().all(|position| {
                    position.0 >= 0 && !occupied.contains(position)
                }) {
                    self.x -= 1;
                    true
                } else {
                    false
                }
            },
            Direction::Down => {
                let moving_to = vec![
                    (self.x - 1, self.y - 1),
                    (self.x, self.y - 2),
                    (self.x + 1, self.y - 1)
                ];

                if moving_to.iter().all(|position| {
                    position.1 >= 0 && !occupied.contains(position)
                }) {
                    self.y -= 1;
                    true
                } else {
                    false
                }
            },
        }
    }

    fn top(&self) -> i64 {
        self.y + 1
    }

    fn cristalize(&self) -> Vec<Position> {
        vec![(self.x,self.y),(self.x+1,self.y),(self.x-1, self.y),(self.x, self.y+1),(self.x, self.y-1)]
    }
}

// An L shaped rock
// ..#
// ..#
// ###
struct L {
    // track the top position
    y: i64,
    x: i64
}

impl Collider for L {
    fn push(&mut self, direction: &Direction, occupied: &HashSet<Position>) -> bool {
        match direction {
            Direction::Right => {
                let moving_to = vec![
                    (self.x + 1, self.y),
                    (self.x + 1, self.y - 1),
                    (self.x + 1, self.y - 2)
                ];

                if moving_to.iter().all(|position| {
                    position.0 < CHAMBER_WIDTH && !occupied.contains(position)
                }) {
                    self.x += 1;
                    true
                } else {
                    false
                }
            },
            Direction::Left => {
                let moving_to = vec![
                    (self.x - 1, self.y),
                    (self.x - 1, self.y - 1),
                    (self.x - 3, self.y - 2)
                ];

                if moving_to.iter().all(|position| {
                    position.0 >= 0 && !occupied.contains(position)
                }) {
                    self.x -= 1;
                    true
                } else {
                    false
                }
            },
            Direction::Down => {
                let moving_to = vec![
                    (self.x, self.y - 3),
                    (self.x - 1, self.y - 3),
                    (self.x - 2, self.y - 3)
                ];

                if moving_to.iter().all(|position| {
                    position.1 >= 0 && !occupied.contains(position)
                }) {
                    self.y -= 1;
                    true
                } else {
                    false
                }
            },
        }
    }

    fn top(&self) -> i64 {
        self.y
    }

    fn cristalize(&self) -> Vec<Position> {
        vec![(self.x,self.y),(self.x,self.y-1),(self.x, self.y-2),(self.x-1, self.y-2),(self.x-2, self.y-2)]
    }
}


// A vertical line  shaped rock
// ..#
// ..#
// ..#
// ..#
struct VerticalLine {
    // track the top position
    y: i64,
    x: i64
}

impl Collider for VerticalLine {
    fn push(&mut self, direction: &Direction, occupied: &HashSet<Position>) -> bool {
        match direction {
            Direction::Right => {
                let moving_to = vec![
                    (self.x + 1, self.y),
                    (self.x + 1, self.y - 1),
                    (self.x + 1, self.y - 2),
                    (self.x + 1, self.y - 3)
                ];

                if moving_to.iter().all(|position| {
                    position.0 < CHAMBER_WIDTH && !occupied.contains(position)
                }) {
                    self.x += 1;
                    true
                } else {
                    false
                }
            },
            Direction::Left => {
                let moving_to = vec![
                    (self.x - 1, self.y),
                    (self.x - 1, self.y - 1),
                    (self.x - 1, self.y - 2),
                    (self.x - 1, self.y - 3),
                ];

                if moving_to.iter().all(|position| {
                    position.0 >= 0 && !occupied.contains(position)
                }) {
                    self.x -= 1;
                    true
                } else {
                    false
                }
            },
            Direction::Down => {
                let moving_to = vec![
                    (self.x, self.y - 4),
                ];

                if moving_to.iter().all(|position| {
                    position.1 >= 0 && !occupied.contains(position)
                }) {
                    self.y -= 1;
                    true
                } else {
                    false
                }
            },
        }
    }

    fn top(&self) -> i64 {
        self.y
    }

    fn cristalize(&self) -> Vec<Position> {
        vec![(self.x,self.y),(self.x,self.y-1),(self.x, self.y-2),(self.x, self.y-3)]
    }
}

// A square shaped rock
// ##
// ##
struct Square {
    // track the top position
    y: i64,
    x: i64
}

impl Collider for Square {
    fn push(&mut self, direction: &Direction, occupied: &HashSet<Position>) -> bool {
        match direction {
            Direction::Right => {
                let moving_to = vec![
                    (self.x + 1, self.y),
                    (self.x + 1, self.y - 1),
                ];

                if moving_to.iter().all(|position| {
                    position.0 < CHAMBER_WIDTH && !occupied.contains(position)
                }) {
                    self.x += 1;
                    true
                } else {
                    false
                }
            },
            Direction::Left => {
                let moving_to = vec![
                    (self.x - 2, self.y),
                    (self.x - 2, self.y - 1),
                ];

                if moving_to.iter().all(|position| {
                    position.0 >= 0 && !occupied.contains(position)
                }) {
                    self.x -= 1;
                    true
                } else {
                    false
                }
            },
            Direction::Down => {
                let moving_to = vec![
                    (self.x, self.y - 2),
                    (self.x - 1, self.y - 2),
                ];

                if moving_to.iter().all(|position| {
                    position.1 > 0 && !occupied.contains(position)
                }) {
                    self.y -= 1;
                    true
                } else {
                    false
                }
            },
        }
    }

    fn top(&self) -> i64 {
        self.y
    }

    fn cristalize(&self) -> Vec<Position> {
        vec![(self.x,self.y),(self.x-1,self.y),(self.x, self.y-1),(self.x-1, self.y-1)]
    }
}

trait Collider {
    // Push the collider towards a direction given a certain set of occupied positions.
    // Returns true if it moved and false if it can't.
    fn push(&mut self, direction: &Direction, occupied: &HashSet<Position>) -> bool;

    fn top(&self) -> i64;

    fn cristalize(&self) -> Vec<Position>;
}

// (x, y)
type Position = (i64, i64);

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Down
}
