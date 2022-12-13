use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../inputs/12.txt");

    let mut grid = vec![];
    let mut start: Coord = (0,0);

    for (i, line) in input.lines().enumerate() {
        grid.push(vec![]);

        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (i as i32, j as i32);
            }
            grid[i].push(c);
        }
    }

    let map = Map::new(grid.clone());

    // PART I
    let paths_to_end = find_paths(&map, start, 'E', |a,b| { b - a <= 1 });
    let shortest_path = paths_to_end.iter().map(|p| p.len()).min();
    println!("Shortest path to end: {}", shortest_path.unwrap() - 1);

    // PART 2
    let end = paths_to_end[0].last().unwrap();
    let paths_to_as = find_paths(&map, *end, 'a', |a,b| { a - b <= 1 });
    let shortest_path = paths_to_as.iter().map(|p| p.len()).min();
    println!("Shortest from end to a: {}", shortest_path.unwrap() - 1);

    // Just for fun
    let mut paths_to_end = paths_to_end.clone();
    paths_to_end.sort_by(|a ,b| a.len().partial_cmp(&b.len()).unwrap());
    let shortest_path = HashSet::from_iter(paths_to_end.first().unwrap().iter().copied());
    print_path(shortest_path, grid.clone());
}

fn print_path(path: HashSet<Coord>, grid: Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c = grid[i][j];
            if path.contains(&(i.try_into().unwrap() ,j.try_into().unwrap())) {
                print!("\u{1b}[38;5;{}m{} ", 244, "*");
            } else {
                let color = elevation(c) % 256 + 17;
                print!("\u{1b}[38;5;{}m{} ", color, c);
            }
        }
        println!();
    }
    println!("\x1b[0m"); // clear color
}

fn find_paths(map: &Map, start: Coord, end_char: char, is_accessible: fn(i32, i32) -> bool) -> Vec<Vec<Coord>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut paths = Vec::new();

    queue.push_back(vec![start]);
    
    while !queue.is_empty() {
        let path = queue.pop_front().unwrap();
        let node = path.last().unwrap();
        let current_char = map.at(&node);

        if current_char == end_char {
            paths.push(path.clone());
        } else if !visited.contains(node) {
            for neighbor in map.neighbors(&node) {
                let our_elevation = elevation(map.at(&node));
                let elevation = elevation(map.at(&neighbor));

                if is_accessible(our_elevation, elevation) {
                    let mut new_path = path.clone();
                    new_path.push(neighbor.clone());
                    queue.push_back(new_path);
                }
            }

            visited.insert(node.clone());
        }
    }

    return paths;
}

fn elevation(c: char) -> i32 {
  match c {
      'S' => 0,
      'E' => 26,
      _ => (1 + c as u8 - b'a').into()
  }
}

type Coord = (i32, i32);

struct Map {
    grid: Vec<Vec<char>>,
}

impl Map {
    fn new(grid: Vec<Vec<char>>) -> Self {
        Self { grid }
    }

    fn neighbors(&self, coord: &Coord) -> Vec<Coord> {
        vec![
            (coord.0 + 1, coord.1),
            (coord.0 - 1, coord.1),
            (coord.0, coord.1 + 1),
            (coord.0, coord.1 - 1),
        ]
            .iter()
            .filter(|c| self.inbounds(c))
            .copied()
            .collect()

    }

    fn inbounds(&self, coord: &Coord) -> bool {
        (coord.0 >= 0 && coord.0 < self.grid.len() as i32) &&
            (coord.1 >= 0 && coord.1 < self.grid[0].len() as i32)
    }

    fn at(&self, coord: &Coord) -> char {
        self.grid[coord.0 as usize][coord.1 as usize]
    }
}
