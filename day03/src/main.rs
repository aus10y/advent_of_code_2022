use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

const ITEMS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let item_values: HashMap<char, usize> =
        HashMap::from_iter(ITEMS.chars().enumerate().map(|(i, v)| (v, i + 1)));

    part1(&input, &item_values)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str, values: &HashMap<char, usize>) -> Result<()> {
    let lines: Vec<&str> = input
        .split("\n")
        .into_iter()
        .filter(|line| *line != "")
        .collect();

    let score: usize = lines
        .iter()
        .map(|line| {
            let length = line.len();

            let compartment1: HashSet<char> = HashSet::from_iter(line.chars().take(length / 2));
            let compartment2: HashSet<char> = HashSet::from_iter(line.chars().skip(length / 2));

            let intersection: Vec<&char> = compartment1.intersection(&compartment2).collect();

            values.get(intersection[0]).unwrap()
        })
        .sum();

    println!("Part1 - Sum of priorities: {}", score);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    Ok(())
}
