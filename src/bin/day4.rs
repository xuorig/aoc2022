use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Pair {
    first: Assignment,
    second: Assignment
}

impl Pair {
    fn has_fully_containing_assignment(&self) -> bool {
        (self.first.from <= self.second.from && self.first.to >= self.second.to) ||
            (self.second.from <= self.first.from && self.second.to >= self.first.to)
    }

    fn has_overlap(&self) -> bool {
        (self.first.from >= self.second.from && self.first.from <= self.second.to) ||
            (self.second.from >= self.first.from && self.second.from <= self.first.to)
    }
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");
        let first = split.next().unwrap().parse::<Assignment>()?;
        let second = split.next().unwrap().parse::<Assignment>()?;

        return Ok(Pair {
            first,
            second,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Assignment {
    from: u32,
    to: u32
}

impl FromStr for Assignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("-");
        let from = split.next().unwrap().parse::<u32>().expect("Expected valid integer ranges");
        let to = split.next().unwrap().parse::<u32>().expect("Expected valid integer ranges");

        return Ok(
            Assignment {
                from,
                to,
            }
        )
    }
}

fn main() {
    let input = include_str!("..//inputs/4.txt");

    let pairs = input.lines().map(|l| l.parse::<Pair>());

    let part1 = pairs.clone().filter(|pair| {
        pair.expect("Malformed Pair").has_fully_containing_assignment()
    }).count();

    let part2 = pairs.filter(|pair| {
        pair.expect("Malformed Pair").has_overlap()
    }).count();

    println!("PART1: {part1}");
    println!("PART2: {part2}");
}
