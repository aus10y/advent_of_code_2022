use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input.strip_suffix("\n").unwrap();

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(PartialEq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn points(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

impl RPS {
    fn from_str(given: &str) -> RPS {
        match given {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            blah => {
                println!(">> Unexpected: '{}'", blah);
                panic!()
            }
        }
    }

    fn play(&self, other: RPS) -> Outcome {
        match self {
            RPS::Rock => match other {
                RPS::Rock => Outcome::Draw,
                RPS::Paper => Outcome::Lose,
                RPS::Scissors => Outcome::Win,
            },
            RPS::Paper => match other {
                RPS::Rock => Outcome::Win,
                RPS::Paper => Outcome::Draw,
                RPS::Scissors => Outcome::Lose,
            },
            RPS::Scissors => match other {
                RPS::Rock => Outcome::Lose,
                RPS::Paper => Outcome::Win,
                RPS::Scissors => Outcome::Draw,
            },
        }
    }

    fn points(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

fn to_rps(input: &str) -> Vec<Vec<RPS>> {
    let x: Vec<Vec<RPS>> = input
        .split("\n")
        .map(|line| line.split(" ").map(|c| RPS::from_str(c)).collect())
        .collect();
    x
}

fn part1(input: &str) -> Result<()> {
    let games = to_rps(input);

    let total_score: u32 = games
        .iter()
        .map(|game| {
            let (opponent, me) = (game[0], game[1]);
            me.points() + me.play(opponent).points()
        })
        .sum();

    println!("Total Score: {}", total_score);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    Ok(())
}
