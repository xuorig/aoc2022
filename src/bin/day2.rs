use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Round {
    them: Move,
    us: Move,
}

impl Round {
    fn score(&self) -> u64 {
       self.us.shape_score() + self.outcome().points()
    }

    fn score2(&self) -> u64 {
        let outcome = RoundOutcome::from_code(self.us);

        let our_shape = match (self.them, outcome) {
            (Move::Rock, RoundOutcome::DRAW) => Move::Rock,
            (Move::Rock, RoundOutcome::LOSS) => Move::Scissors,
            (Move::Rock, RoundOutcome::WIN) => Move::Paper,
            (Move::Paper, RoundOutcome::DRAW) => Move::Paper,
            (Move::Paper, RoundOutcome::LOSS) => Move::Rock,
            (Move::Paper, RoundOutcome::WIN) => Move::Scissors,
            (Move::Scissors, RoundOutcome::DRAW) => Move::Scissors,
            (Move::Scissors, RoundOutcome::LOSS) => Move::Paper,
            (Move::Scissors, RoundOutcome::WIN) => Move::Rock,
        };

        our_shape.shape_score() + outcome.points()
    }

    fn outcome(&self) -> RoundOutcome {
        match (self.us, self.them) {
            (Move::Rock, Move::Rock) => RoundOutcome::DRAW,
            (Move::Rock, Move::Paper) => RoundOutcome::LOSS,
            (Move::Rock, Move::Scissors) => RoundOutcome::WIN,
            (Move::Paper, Move::Rock) => RoundOutcome::WIN,
            (Move::Paper, Move::Paper) => RoundOutcome::DRAW,
            (Move::Paper, Move::Scissors) => RoundOutcome::LOSS,
            (Move::Scissors, Move::Rock) => RoundOutcome::LOSS,
            (Move::Scissors, Move::Paper) => RoundOutcome::WIN,
            (Move::Scissors, Move::Scissors) => RoundOutcome::DRAW,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RoundOutcome {
    WIN,
    LOSS,
    DRAW
}

impl RoundOutcome {
    fn from_code(code: Move) -> RoundOutcome {
        match code {
            Move::Rock => RoundOutcome::LOSS,
            Move::Paper => RoundOutcome::DRAW,
            Move::Scissors => RoundOutcome::WIN,
        }
    }

    fn points(&self) -> u64 {
        match self {
            RoundOutcome::WIN => 6,
            RoundOutcome::LOSS => 0,
            RoundOutcome::DRAW => 3,
        }
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let (Some(them), Some(' '), Some(us), _) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(());
        };

        Ok(Self {
            them: them.try_into()?,
            us: us.try_into()?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    fn shape_score(&self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl TryFrom<char> for Move {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(())
        }
    }
}

fn main() {
    let input = include_str!("../inputs/2.txt");

    let sum: u64 = input.lines().map(|line| {
        line.parse::<Round>().unwrap()
    }).map(|round| {
        round.score()
    }).sum();

    println!("SUM: {sum}");

    let sum2: u64 = input.lines().map(|line| {
        line.parse::<Round>().unwrap()
    }).map(|round| {
        round.score2()
    }).sum();

    println!("SUM: {sum2}");
}
