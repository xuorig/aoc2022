use std::collections::{HashMap, VecDeque, HashSet};

use nom::{IResult, sequence::preceded, bytes::complete::{tag, take}, character::complete::digit1, multi::separated_list1, combinator::map_res, branch::alt};

fn main() {
    //part1();
    part2();
}

fn part1() {
    let input = include_str!("../inputs/16.txt");

    let valves: HashMap<String, Valve> = input
        .lines()
        .map(|l| {
            let valve = parse_valve(l).unwrap().1;
            (valve.id.clone(), valve)
        })
        .collect();

    let valves_to_visit: Vec<String> = valves
        .iter()
        .filter(|(_, v)| v.flow_rate > 0 || v.id == "AA")
        .map(|(k, _)| k.clone())
        .collect();

    let mut costs: HashMap<String, HashMap<String, u64>> = HashMap::new();

    for valve in &valves_to_visit {
        costs.insert(valve.clone(), HashMap::new());

        let dist = distances(&valve, &valves);

        for target in &valves_to_visit {
            costs.get_mut(valve).map(|val| val.insert(target.clone(), dist[target] + 1));
        }
    }

    let mut stack = vec![];
    let mut max_released = 0;
    let mut max_path = (vec![], 26, 0, 0);
    let mut initial = HashSet::new();
    initial.insert("AA".to_string());
    stack.push((vec!["AA".to_string()], 26, 0, 0, initial));

    println!("{:?}", costs);

    while !stack.is_empty() {
      let (path, time, released, release_per_min, opened) = stack.pop().unwrap();

      let current = path.last().unwrap();

      let final_released = released + release_per_min * time;
      if final_released > max_released {
          max_released = final_released;
          max_path = (path.clone(), 0, final_released, release_per_min);
      }
      
      for neighbor in costs[current].keys() {
          if !opened.contains(neighbor) {

              let cost = costs[current][neighbor];

              if cost > time {
                  continue;
              }

              let released_during_open = cost * release_per_min;
              let flow_rate = valves[neighbor].flow_rate;

              let mut new_opened = opened.clone();
              new_opened.insert(neighbor.to_string());

              let mut new_path = path.clone();
              new_path.push(neighbor.to_string());

              stack.push((
                  new_path,
                  time - cost,
                  released + released_during_open,
                  release_per_min + flow_rate,
                  new_opened
              ));
          }
      }

    }

    println!("MAX {}", max_released);
    println!("MAX {:?}", max_path);
}

fn part2() {
    let input = include_str!("../inputs/16.txt");

    let valves: HashMap<String, Valve> = input
        .lines()
        .map(|l| {
            let valve = parse_valve(l).unwrap().1;
            (valve.id.clone(), valve)
        })
        .collect();

    let valves_to_visit: Vec<String> = valves
        .iter()
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(k, _)| k.clone())
        .collect();

    let mut costs: HashMap<String, HashMap<String, u64>> = HashMap::new();
    costs.insert("AA".to_string(), HashMap::new());
    let dist_a = distances(&"AA".to_string(), &valves);
    for target in &valves_to_visit {
        costs.get_mut(&"AA".to_string()).map(|val| val.insert(target.clone(), dist_a[target] + 1));
    }

    for valve in &valves_to_visit {
        costs.insert(valve.clone(), HashMap::new());

        let dist = distances(&valve, &valves);

        for target in &valves_to_visit {
            costs.get_mut(valve).map(|val| val.insert(target.clone(), dist[target] + 1));
        }
    }

    let mut max = 0;

    let indices: HashMap<String, u32> = HashMap::from_iter(valves_to_visit.iter().cloned().enumerate().map(|(i, v)| (v, i as u32)));

    // Find all partitions
    let num_of_subsets = (1 << valves_to_visit.len()) - 1;

    for i in 0..num_of_subsets + 1 {
        let sum = dfs(&costs, &valves, i, &indices) + dfs(&costs, &valves, num_of_subsets ^ i, &indices);
        if sum > max {
            max = sum;
        }
    }

    println!("MAX {}", max);
}

fn dfs(costs: &HashMap<String, HashMap<String, u64>>, valves: &HashMap<String, Valve>, bitmask: u32, indices: &HashMap<String, u32>) -> u64 {
    let mut stack = vec![];
    let start = &"AA".to_string();
    stack.push((start, 26, 0, 0, bitmask));
    let mut max_released = 0;

    while !stack.is_empty() {
        let (current, time, released, release_per_min, bitmask) = stack.pop().unwrap();

        let final_released = released + release_per_min * time;
        if final_released > max_released {
            max_released = final_released;
        }

        for neighbor in costs[current].keys() {
            if neighbor == start {
                continue;
            }

            let bit = 1 << indices[neighbor];

            if bitmask & bit != 0 {
                continue;
            }


            let cost = costs[current][neighbor];

            if cost > time {
                continue;
            }

            let released_during_open = cost * release_per_min;
            let flow_rate = valves[neighbor].flow_rate;

            stack.push((
                    neighbor,
                    time - cost,
                    released + released_during_open,
                    release_per_min + flow_rate,
                    bitmask | bit
                    ));
        }
    }

    max_released
}


fn distances(from: &String, valves: &HashMap<String, Valve>) -> HashMap<String, u64> {
    let mut queue = VecDeque::new();
    let mut dist = HashMap::new();
    let mut visited = HashSet::new();

    queue.push_back(from);
    dist.insert(from.clone(), 0);

    while !queue.is_empty() {
        let valve_id = queue.pop_front().unwrap();
        visited.insert(valve_id.clone());
        let valve = &valves[valve_id];

        for target in &valve.tunnels {
            if visited.contains(target) {
                continue
            }

            let old_dist = dist.get(target).map_or(u64::MAX, |dist| *dist);
            let new_dist = dist[valve_id] + 1;

            if new_dist < old_dist {
                dist.insert(target.to_string(), new_dist);
                queue.push_back(target);
            }
        }
    }

    dist
}

fn parse_valve(input: &str) -> IResult<&str, Valve> {
    let (i, id) = preceded(tag("Valve "), take(2usize))(input)?;
    let (i, flow_rate) = map_res(preceded(tag(" has flow rate="), digit1), str::parse::<u64>)(i)?;

    let (i, neighbors) = preceded(
        alt((tag("; tunnels lead to valves "), tag("; tunnel leads to valve "))),
        separated_list1(
            tag(", "),
            take(2usize)
        )

    )(i)?;
    return Ok((i, Valve { id: id.to_string(), flow_rate, tunnels: neighbors.iter().map(|n| n.to_string()).collect() }))
}

#[derive(Debug, Clone)]
struct Valve {
    id: String,
    flow_rate: u64,
    tunnels: Vec<String>
}

#[cfg(test)]
mod tests {
    use super::parse_valve;

    #[test]
    fn parse_test() {
        let test_input = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

        let (_, valve) = parse_valve(test_input.lines().next().unwrap()).expect("Failed to parse");
        assert_eq!("AA".to_string(), valve.id);
        assert_eq!(0, valve.flow_rate);
        assert_eq!(vec!["DD", "II", "BB"], valve.tunnels);
    }
}
