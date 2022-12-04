use sscanf::sscanf;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}
impl FromStr for Choice {
    type Err = ();

    fn from_str(input: &str) -> Result<Choice, Self::Err> {
        match input {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}
impl Choice {
    fn defeater(self) -> Choice {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
    fn loser(self) -> Choice {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
    fn beats(self, opponent: Choice) -> bool {
        opponent != self && opponent != self.defeater()
    }
    fn value(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}
impl FromStr for Outcome {
    type Err = ();

    fn from_str(input: &str) -> Result<Outcome, Self::Err> {
        match input {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}
impl Outcome {
    fn value(self) -> u32 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }

    fn choice_needed(self, opponent: Choice) -> Choice {
        match self {
            Self::Lose => opponent.loser(),
            Self::Draw => opponent,
            Self::Win => opponent.defeater(),
        }
    }

    fn from_choices(opponent: Choice, response: Choice) -> Self {
        if opponent == response {
            return Self::Draw;
        }
        if response.beats(opponent) {
            return Self::Win;
        }
        Self::Lose
    }
}

fn main() {
    let lines = utils::read_arg_file_lines();
    let mut p1_points: u32 = 0;
    let mut p2_points: u32 = 0;
    for line in lines.iter() {
        let (opponent, response) = sscanf!(line, "{String} {String}",).unwrap();
        let opponent = Choice::from_str(&opponent).unwrap();
        let desired_outcome = Outcome::from_str(&response).unwrap();

        // Part 1
        let response = Choice::from_str(&response).unwrap();
        p1_points += response.value() + Outcome::from_choices(opponent, response).value();

        // Part 2
        p2_points += desired_outcome.value() + desired_outcome.choice_needed(opponent).value();
    }
    println!("Total part 1 points: {p1_points}");
    println!("Total part 2 points: {p2_points}");
}
