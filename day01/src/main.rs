use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse_calories_by_elf(input: &str) -> Vec<Vec<u32>> {
    let calories_by_elf: Vec<Vec<u32>> = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    calories_by_elf
}

fn part1(input: &str) -> Result<()> {
    let calories_by_elf = parse_calories_by_elf(input);
    let max_calories = calories_by_elf
        .iter()
        .map(|elf_calories| elf_calories.iter().sum::<u32>())
        .max()
        .unwrap();

    println!("Max calories: {}", max_calories);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let calories_by_elf = parse_calories_by_elf(input);
    let mut summed: Vec<u32> = calories_by_elf
        .iter()
        .map(|elf_calories| elf_calories.iter().sum::<u32>())
        .collect();
    summed.sort();

    let total: u32 = summed.iter().rev().take(3).sum();

    println!("Top 3 calories: {}", total);

    Ok(())
}
