use std::{fs, io::Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("tmp/2.in")?;

    println!("PART 1: {}", part_1(&input)?);
    println!("PART 2: {}", part_2(&input)?);

    Ok(())
}

fn part_1(input: &str) -> Result<u32> {
    let mut total_score = 0_u32;
    let mut opponent = String::new();
    let mut player = String::new();
    let mut is_prev_space = false;
    for c in input.chars() {
        if c == '\n' {
            continue;
        }
        if c == ' ' {
            is_prev_space = true;
            continue;
        }
        match is_prev_space {
            true => {
                player.push(c);
                total_score += Hand::new(&player)?.play(&Hand::new(&opponent)?);
                player.pop();
                opponent.pop();
                is_prev_space = false;
            }
            false => opponent.push(c),
        }
    }
    Ok(total_score)
}

fn part_2(input: &str) -> Result<u32> {
    let mut total_score = 0_u32;
    let mut opponent = String::new();
    let mut result = String::new();
    let mut is_prev_space = false;
    for c in input.chars() {
        if c == '\n' {
            continue;
        }
        if c == ' ' {
            is_prev_space = true;
            continue;
        }
        match is_prev_space {
            true => {
                result.push(c);
                total_score += Hand::new(&opponent)?.play_by_result(&Outcome::new(&result)?);
                result.pop();
                opponent.pop();
                is_prev_space = false;
            }
            false => opponent.push(c),
        }
    }
    Ok(total_score)
}

#[derive(Copy, Clone, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn new(c: &str) -> Result<Self> {
        match c {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => panic!("Not a valid character"),
        }
    }

    fn beats(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn beaten_by(&self) -> Self {
        match self {
            Self::Scissors => Self::Rock,
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
        }
    }

    fn play(&self, other: &Hand) -> u32 {
        let (self_beats, other_beats) = (self.beats(), other.beats());
        let outcome = match (self, other) {
            _ if self_beats == *other => Outcome::Win,
            _ if *self == other_beats => Outcome::Lose,
            _ => Outcome::Draw,
        };
        self.points() + outcome.points()
    }

    fn play_by_result(&self, result: &Outcome) -> u32 {
        let hand = match result {
            Outcome::Win => self.beaten_by(),
            Outcome::Draw => *self,
            Outcome::Lose => self.beats(),
        };
        return hand.points() + result.points();
    }
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn new(c: &str) -> Result<Self> {
        match c {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => panic!("Not a valid character"),
        }
    }
}

trait Points {
    fn points(&self) -> u32;
}

impl Points for Hand {
    fn points(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl Points for Outcome {
    fn points(&self) -> u32 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}
