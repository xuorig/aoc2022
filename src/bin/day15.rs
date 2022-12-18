use std::collections::HashSet;

use itertools::Itertools;
use rayon::prelude::*;

use nom::{sequence::{preceded, tuple}, character::complete::digit1, bytes::complete::tag, combinator::opt, IResult};

fn main() {
    println!("{}", part1());
    println!("{:?}", part2());
    //println!("{:?}", parallel_part_2());
}

const ROW: i64 = 2000000; 
const UPPER_BOUND: i64 = 4000000; 

fn part1() -> usize {
    let input = include_str!("../inputs/15.txt");
    let coords: Vec<CoordinatePair> = input.lines().map(|l| parse_coordinates(l).unwrap().1 ).collect();

    let beacons: HashSet<Coord> = HashSet::from_iter(
        coords
        .iter()
        .filter(|c| c.beacon.1 == ROW)
        .map(|c| c.beacon)
    );

    let mut visited: HashSet<Coord> = HashSet::new();

    for coord in coords {
        let distance = man(coord.sensor, coord.beacon);
        let sensor_to_row = i64::abs(ROW - coord.sensor.1);
        let x_diff = distance - sensor_to_row;
        for x in (coord.sensor.0 - x_diff)..(coord.sensor.0 + x_diff + 1) {
            visited.insert((x, ROW));
        }
    }

    visited.iter().filter(|c| !beacons.contains(c)).collect::<Vec<&Coord>>().len()
}

fn part2() -> Option<i64> {
    let input = include_str!("../inputs/15.txt");
    let coords: Vec<CoordinatePair> = input.lines().map(|l| parse_coordinates(l).unwrap().1 ).collect();

    let ranges: Vec<(Coord, i64)> = coords
        .iter()
        .map(|c| (c.sensor, man(c.sensor, c.beacon)))
        .sorted()
        .collect();

    let mut y: i64 = 0;
    let mut x: i64 = 0;

    while y <= UPPER_BOUND {
        while x <= UPPER_BOUND {
            let sensor = ranges
                .iter()
                .map(|(sensor, range)| (sensor, man((x,y), *sensor), range))
                .find(|(_, distance, range)| distance <= *range)
                .map(|(sensor, _, range)| (sensor, range));

            match sensor {
                Some((sensor, range)) => {
                    let y_diff = i64::abs(sensor.1 - y);
                    x = i64::min(sensor.0 + range - y_diff + 1, UPPER_BOUND + 1);
                },
                None => {
                    println!("{},{}", x, y);
                    return Some(x * 4000000 + y)
                },
            }
        }

        x = 0;
        y += 1;
    }

    None
}

fn parallel_part_2() {
    let input = include_str!("../inputs/15.txt");
    let coords: Vec<CoordinatePair> = input.lines().map(|l| parse_coordinates(l).unwrap().1 ).collect();

    let ranges: Vec<(Coord, i64)> = coords
        .iter()
        .map(|c| (c.sensor, man(c.sensor, c.beacon)))
        .sorted()
        .collect();


    (0..UPPER_BOUND + 1).into_par_iter().find_any(|y| {
        let mut x = 0;

        while x <= UPPER_BOUND {
            let sensor = ranges
                .iter()
                .map(|(sensor, range)| (sensor, man((x,*y), *sensor), range))
                .find(|(_, distance, range)| distance <= *range)
                .map(|(sensor, _, range)| (sensor, range));

            match sensor {
                Some((sensor, range)) => {
                    let y_diff = i64::abs(sensor.1 - y);
                    x = i64::min(sensor.0 + range - y_diff + 1, UPPER_BOUND + 1);
                },
                None => {
                    println!("FOUND: {},{} FREQ: {}", x, y, x * 4000000 + y);
                    return true
                },
            }
        }

        false
    });

}

fn man(a: Coord, b: Coord) -> i64 {
    i64::abs(a.0 - b.0) + i64::abs(a.1 - b.1)
}

fn parse_coordinates(input: &str) -> IResult<&str, CoordinatePair> {
    let (i, sensor_x) = preceded(tag("Sensor at x="), parse_number)(input)?;
    let (i, sensor_y) = preceded(tag(", y="), parse_number)(i)?;
    let (i, beacon_x) = preceded(tag(": closest beacon is at x="), parse_number)(i)?;
    let (i, beacon_y) = preceded(tag(", y="), parse_number)(i)?;
    Ok((i, CoordinatePair { sensor: (sensor_x, sensor_y), beacon: (beacon_x, beacon_y) }))
}

fn parse_number(input: &str) -> IResult<&str, i64> {
    let (i, (sign, number)) = tuple((opt(tag("-")), digit1))(input)?;

    let number = number.parse::<i64>().unwrap();

    match sign {
        Some(_) => Ok((i, number * -1)),
        None => Ok((i, number)),
    }
}

#[derive(Debug)]
struct CoordinatePair {
    sensor: Coord,
    beacon: Coord
}

type Coord = (i64, i64);
