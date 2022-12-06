use std::collections::VecDeque;

pub fn main() {
    let input = include_str!("../inputs/5.txt");

    let mut stacks = vec![];
    let mut orders = vec![];

    for (i, line) in input.lines().enumerate() {
        let mut parser = StackLineParser::new(line);

        if let Some(row) = parser.parse_line() {
            match row {
                ParsedLine::Boxes(boxes) => {
                    for (j, bbox) in boxes.iter().enumerate() {
                        if i == 0 { stacks.push(VecDeque::new()) }

                        match bbox {
                            BBox::Box(c) => {
                                stacks[j].push_front(c.clone());
                            }
                            BBox::Empty => {}
                        }
                    }
                },
                ParsedLine::Order(order) => {
                    orders.push(order);
                },
                _ => {}
            }
        }
    }

    for order in orders {
        let mut items_to_move = vec![];

        for _ in 0..order.quantity {
            items_to_move.push(stacks[order.from].pop_back().expect("Invalid instructions"));
        }

        while let Some(item) = items_to_move.pop() {
            stacks[order.to].push_back(item);
        }
    }

    for stack in stacks {
        print!("{}", stack[stack.len()-1]);
    }

    println!();
}

struct StackLineParser {
    cursor: usize,
    chars: Vec<char>
}

impl StackLineParser {
    fn new(line: &str) -> Self {
        Self { cursor: 0, chars: line.chars().collect() }
    }

    fn parse_line(&mut self) -> Option<ParsedLine> {
        if let Some(first_box) = self.parse_box() {
            let mut boxes = vec![];
            boxes.push(first_box);

            while let Some(bbox) = self.parse_box_with_space() {
                boxes.push(bbox);
            }

            return Some(ParsedLine::Boxes(boxes))
        } else if let Some(crate_number) = self.parse_crate_number() {
            let mut crate_numbers = vec![];

            crate_numbers.push(crate_number);

            while let Some(crate_number) = self.parse_crate_number_with_space() {
                crate_numbers.push(crate_number);
            }

            return Some(ParsedLine::CrateNumbers(crate_numbers))
        } else if let Some(order) = self.parse_order() {
            return Some(ParsedLine::Order(order))
        } else {
            return None

        }
    }

    fn parse_box(&mut self) -> Option<BBox> {
        if self.cursor >= self.chars.len() { return None }

        let current_char = self.chars[self.cursor];
        let next_char = self.chars[self.cursor+1];

        // This is a box
        if current_char == '[' && next_char.is_alphabetic() {
            let label = self.chars[self.cursor + 1];
            self.cursor = self.cursor + 3;
            return Some(BBox::Box(label))
        }

        // This is a space
        if current_char == ' ' && !next_char.is_digit(10) {
            self.cursor = self.cursor + 3;
            return Some(BBox::Empty)
        }

        None
    }

    // Parse box with a preceding " "
    fn parse_box_with_space(&mut self) -> Option<BBox> {
        if self.cursor >= self.chars.len() { return None }

        let current_char = self.chars[self.cursor];

        if current_char == ' ' {
            self.cursor += 1;
            self.parse_box()
        } else {
            None
        }
    }

    fn parse_crate_number(&mut self) -> Option<usize> {
        if self.cursor >= self.chars.len() { return None }

        let current_char = self.chars[self.cursor];
        let next_char = self.chars[self.cursor+1];

        // This is a box
        if current_char == ' ' && next_char.is_digit(10) {
            let number = self.chars[self.cursor + 1];
            self.cursor = self.cursor + 3;
            return Some(number.to_string().parse::<usize>().unwrap())
        }

        None
    }

    // Parse box with a preceding " "
    fn parse_crate_number_with_space(&mut self) -> Option<usize> {
        if self.cursor >= self.chars.len() { return None }

        let current_char = self.chars[self.cursor];

        if current_char == ' ' {
            self.cursor += 1;
            self.parse_crate_number()
        } else {
            None
        }
    }

    fn parse_order(&mut self) -> Option<Order> {
        if self.cursor >= self.chars.len() { return None }

        self.cursor += 5;

        let mut quantity = String::new();

        while let Some(digit) = self.parse_digit() {
            quantity.push(digit);
        }

        if quantity.len() == 0 {
            return None
        }

        self.cursor += 6;

        let mut from = String::new();

        while let Some(digit) = self.parse_digit() {
            from.push(digit);
        }

        if from.len() == 0 {
            return None
        }

        self.cursor += 4;

        let mut to = String::new();

        while let Some(digit) = self.parse_digit() {
            to.push(digit);
        }

        if to.len() == 0 {
            return None
        }

        return Some(Order {
            quantity: quantity.parse::<usize>().unwrap(),
            // 0 indexed
            from: from.parse::<usize>().unwrap() - 1,
            to: to.parse::<usize>().unwrap() - 1,
        })
    }

    fn parse_digit(&mut self) -> Option<char> {
        if self.cursor >= self.chars.len() { return None }

        let digit = self.chars[self.cursor];

        if digit.is_digit(10) {
            self.cursor += 1;
            return Some(digit);
        }

        None
    }

}

enum ParsedLine {
    Boxes(Vec<BBox>),
    CrateNumbers(Vec<usize>),
    Order(Order)
}

#[derive(Debug)]
struct Stack {
    number: usize,
    boxes: Vec<BBox>
}

#[derive(Debug)]
enum BBox {
    Box(char),
    Empty
}

#[derive(Debug)]
struct Order {
    quantity: usize,
    from: usize,
    to: usize
}
