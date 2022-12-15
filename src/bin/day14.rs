use std::{collections::HashMap, io::{Write, self}, time, fmt};
use colored::*;
use termion::{raw::IntoRawMode, clear, cursor, screen::IntoAlternateScreen};

fn main() {
    let input = include_str!("../inputs/14.txt");
    //println!("Sand Units: {}", part1(input));
    //println!("Sand Units 2: {}", part2(input));
    let mut cave = Cave::from_traces(input, false);
    cave.animate();
}

fn part1(input: &str) -> u64 {
    let mut cave = Cave::from_traces(input, false);
    let units = cave.fill();
    return units;
}

fn part2(input: &str) -> u64 {
    let mut cave = Cave::from_traces(input, true);
    let units = cave.fill();
    println!("{:?}", cave.bounds);
    return units;
}

struct Cave {
    map: HashMap<Coord, ScanItem>,
    bounds: MapBounds,
    has_floor: bool,
    sand_moves: Vec<(i32, i32)>
}

impl Cave {
    fn from_traces(input: &str, with_floor: bool) -> Self {
        let mut paths = vec![];

        let mut left = i32::MAX;
        let mut right = 0;
        let mut bottom = 0;

        for line in input.lines() {
            let rock_lines: Vec<Coord> = line.split(" -> ").map(|line| {
                let mut coords = line.split(",");

                let x = coords.next().unwrap().parse::<i32>().unwrap();
                let y = coords.next().unwrap().parse::<i32>().unwrap();

                left = i32::min(left, x);
                right = i32::max(right, x);
                bottom = i32::max(bottom, y);

                (x, y)
            }).collect();

            paths.push(rock_lines);
        }

        let bottom = if with_floor {
            bottom + 2
        } else {
            bottom
        };

        Self {
            map: Self::build_map(paths),
            bounds: MapBounds { left, right, bottom },
            has_floor: with_floor,
            sand_moves: vec![(0,1),(-1,1),(1,1)]
        }
    }
    
    fn animate(&mut self) {
        let mut stdout = std::io::stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();

        write!(
            stdout,
            "{clear}{goto}{cave}{hide}",
            clear = clear::All,
            goto = cursor::Goto(1, 1),
            cave = self,
            hide = cursor::Hide,
            )
            .unwrap();

        stdout.flush().unwrap();

        loop {
            let mut current = (500, 0);

            if !self.is_free(&current) {
                break
            }

            loop {
                if current.1 >= self.bounds.bottom {
                    return
                }

                let move_to = self.sand_moves
                    .iter()
                    .map(|m| (current.0 + m.0, current.1 + m.1))
                    .find(|p| self.is_free(p));

                match move_to {
                    Some(new_position) => {
                        let x: u16 = (current.0 - self.bounds.left + 1).try_into().unwrap();
                        let y: u16 = (current.1 + 1).try_into().unwrap();

                        write!(
                            stdout,
                            "{goto}{c}{hide}",
                            goto = cursor::Goto(x, y),
                            c = ".".white(),
                            hide = cursor::Hide,
                            )
                            .unwrap();

                        let x: u16 = (new_position.0 - self.bounds.left + 1).try_into().unwrap();
                        let y: u16 = (new_position.1 + 1).try_into().unwrap();

                        write!(
                            stdout,
                            "{goto}{c}{hide}",
                            goto = cursor::Goto(x, y),
                            c = "~".yellow(),
                            hide = cursor::Hide,
                            )
                            .unwrap();
                        current = new_position;
                    }
                    None => {
                        self.rest(current.clone());
                        let x: u16 = (current.0 - self.bounds.left + 1).try_into().unwrap();
                        let y: u16 = (current.1 + 1).try_into().unwrap();
                        write!(
                            stdout,
                            "{goto}{c}{hide}",
                            goto = cursor::Goto(x, y),
                            c = "o".red(),
                            hide = cursor::Hide,
                            )
                            .unwrap();
                        break;
                    },
                }

                stdout.flush().unwrap();
                std::thread::sleep(std::time::Duration::from_micros(500));
            }
        }
    }

    fn fill(&mut self) -> u64 {
        let mut sand_units = 0;
        while self.produce_sand() {
            sand_units += 1;
        }
        sand_units
    }

    fn produce_sand(&mut self) -> bool {

        let mut current = (500, 0);

        if !self.is_free(&current) {
            return false
        }

        loop {
            if current.1 >= self.bounds.bottom {
                return false;
            }

            let move_to = self.sand_moves
                .iter()
                .map(|m| (current.0 + m.0, current.1 + m.1))
                .find(|p| self.is_free(p));

            match move_to {
                Some(new_position) => {
                    current = new_position;
                }
                None => {
                    self.rest(current.clone());
                    return true
                },
            }
        }
    }

    fn build_map(rock_paths: Vec<Vec<Coord>>) -> HashMap<Coord, ScanItem> {
        let mut scan: HashMap<Coord, ScanItem> = HashMap::new();

        for path in rock_paths {
            for line in path.windows(2) {
                let a = line[0];
                let b = line[1];

                if a.0 < b.0 {
                    for x in a.0..b.0 {
                        scan.insert((x,a.1), ScanItem::Rock);
                    }
                } else if a.0 > b.0 {
                    for x in b.0..a.0+1 {
                        scan.insert((x,a.1), ScanItem::Rock);
                    }
                } else if a.1 < b.1 {
                    for y in a.1..b.1 {
                        scan.insert((a.0,y), ScanItem::Rock);
                    }
                } else if a.1 > b.1 {
                    for y in b.1..a.1+1{
                        scan.insert((a.0,y), ScanItem::Rock);
                    }
                }
            }

            scan.insert(*path.last().unwrap(), ScanItem::Rock);
        }

        return scan;
    }

    fn is_free(&self, coord: &Coord) -> bool {
        if self.has_floor {
            !self.map.contains_key(coord) && coord.1 < self.bounds.bottom
        } else {
            !self.map.contains_key(coord)
        }
    }

    fn rest(&mut self, coord: Coord) {
        self.map.insert(coord, ScanItem::Sand);
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.bounds.bottom + 1 {
            for x in self.bounds.left..self.bounds.right + 1 {
                match self.map.get(&(x, y)) {
                    Some(item) => match item {
                        ScanItem::Rock => write!(f, "{}", "#".green())?,
                        ScanItem::Sand => write!(f, "{}", "O".yellow())?,
                    },
                    None => {
                        if self.has_floor && y == self.bounds.bottom {
                            write!(f, "{}", "#".green())?
                        } else {
                            write!(f, "{}", ".".white())?
                        }
                    },
                }
            }

            write!(f, "\n\r")?;
        }
        Ok(())
    }
}


#[derive(Debug)]
struct MapBounds {
    left: i32,
    right: i32,
    bottom: i32
}


#[derive(Debug)]
enum ScanItem {
    Rock,
    Sand,
}

type Coord = (i32, i32);

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_test() {
        let test_input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

        assert_eq!(24, part1(test_input));
    }

    #[test]
    fn part2_test() {
        let test_input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

        assert_eq!(93, part2(test_input));
    }
}
