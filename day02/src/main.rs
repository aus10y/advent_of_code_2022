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

    fn from_str_part2(given: &str) -> Self {
        match given {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!(),
        }
    }
}

impl RPS {
    fn from_str(given: &str) -> Self {
        match given {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => panic!(),
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

    fn make_choice(&self, outcome: Outcome) -> Self {
        match self {
            RPS::Rock => match outcome {
                Outcome::Win => Self::Paper,
                Outcome::Lose => Self::Scissors,
                Outcome::Draw => Self::Rock,
            },
            RPS::Paper => match outcome {
                Outcome::Win => Self::Scissors,
                Outcome::Lose => Self::Rock,
                Outcome::Draw => Self::Paper,
            },
            RPS::Scissors => match outcome {
                Outcome::Win => Self::Rock,
                Outcome::Lose => Self::Paper,
                Outcome::Draw => Self::Scissors,
            },
        }
    }
}

fn to_rps_part1(input: &str) -> Vec<Vec<RPS>> {
    let x: Vec<Vec<RPS>> = input
        .split("\n")
        .map(|line| line.split(" ").map(|c| RPS::from_str(c)).collect())
        .collect();
    x
}

fn to_rps_part2(input: &str) -> Vec<(RPS, RPS)> {
    let x: Vec<(RPS, RPS)> = input
        .split("\n")
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            match &parts[..] {
                &[opponent, me, ..] => {
                    let opp_rps = RPS::from_str(opponent);
                    (opp_rps, opp_rps.make_choice(Outcome::from_str_part2(me)))
                }
                _ => unreachable!(),
            }
        })
        .collect();
    x
}

fn part1(input: &str) -> Result<()> {
    let games = to_rps_part1(input);

    let total_score: u32 = games
        .iter()
        .map(|game| {
            let (opponent, me) = (game[0], game[1]);
            me.points() + me.play(opponent).points()
        })
        .sum();

    println!("Part 1 - Total Score: {}", total_score);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let games = to_rps_part2(input);

    let total_score: u32 = games
        .iter()
        .map(|game| {
            let (opponent, me) = game;
            me.points() + me.play(*opponent).points()
        })
        .sum();

    println!("Part 2 - Total Score: {}", total_score);

    Ok(())
}
