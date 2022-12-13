use std::cmp::Ordering;

use nom::{IResult, sequence::delimited, character::complete::digit1, combinator::map, branch::alt, bytes::complete::tag, multi::separated_list0};

fn main() {
    let input = include_str!("../inputs/13.txt");
    println!("SUM: {}", part1(input));
    println!("Decoder Key: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let packets: Vec<Vec<PacketValue>> = input.split("\n\n").map(|packet| {
        packet.split_whitespace().map(|packet| {
          let (_, values) = packet_values(packet).unwrap();
          PacketValue::List(values)
        }).collect()
    }).collect();

    return packets
        .iter()
        .enumerate()
        .filter(|(_, pair)| pair[0] <= pair[1])
        .map(|(i, _)| i + 1)
        .sum();
}

fn part2(input: &str) -> usize {
    let mut packets: Vec<PacketValue> = input.lines().filter(|l| !l.is_empty()).map(|packet| {
        let (_, values) = packet_values(packet).unwrap();
        PacketValue::List(values)
    }).collect();

    let tracer_a = tracer_packet(2);
    let tracer_b = tracer_packet(6);
    packets.push(tracer_a.clone());
    packets.push(tracer_b.clone());
    packets.sort();

    return packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| **packet == tracer_a || **packet == tracer_b )
        .map(|(i, _)| i + 1)
        .product();
}

fn tracer_packet(id: u64) -> PacketValue {
    PacketValue::List(vec![PacketValue::List(vec![PacketValue::Integer(id)])])
}

#[derive(Debug, Clone)]
enum PacketValue {
    Integer(u64),
    List(Vec<PacketValue>)
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketValue::Integer(x), PacketValue::Integer(y)) => {
                x.cmp(&y)
            },
            (PacketValue::Integer(_), PacketValue::List(_)) => {
                PacketValue::List(vec![self.clone()]).cmp(&other)
            },
            (PacketValue::List(_), PacketValue::Integer(_)) => {
                self.cmp(&PacketValue::List(vec![other.clone()]))
            },
            (PacketValue::List(x), PacketValue::List(y)) => {
                for i in 0..x.len() {
                    if i >= y.len() {
                        return Ordering::Greater
                    }

                    match x[i].cmp(&y[i]) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => {},
                        Ordering::Greater => return Ordering::Greater,
                    }
                }

                if x.len() == y.len() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            },
        }
    }
}

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PacketValue {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for PacketValue { }

fn packet_values(input: &str) -> IResult<&str, Vec<PacketValue>> {
  delimited(
    tag("["),
    separated_list0(tag(","), packet_value),
    tag("]"),
  )(input)
}

fn packet_value(input: &str) -> IResult<&str, PacketValue> {
  alt((
    map(digit1, |d: &str| { PacketValue::Integer(d.parse::<u64>().unwrap()) }),
    map(packet_values, PacketValue::List),
  ))(input)
}


#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_test() {
        let test_input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

        let result = part1(test_input);
        assert_eq!(13, result);
    }


    #[test]
    fn part2_test() {
        let test_input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

        let result = part2(test_input);
        assert_eq!(140, result);
    }
}
