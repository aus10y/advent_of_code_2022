use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let ranges: Vec<((u32, u32), (u32, u32))> = input
        .split("\n")
        .into_iter()
        .filter(|line| *line != "")
        .map(|line| parse_ranges(line))
        .collect();

    part1(&ranges)?;
    part2(&ranges)?;

    Ok(())
}

fn parse_ranges(line: &str) -> ((u32, u32), (u32, u32)) {
    let parts: Vec<&str> = line.split(",").collect();
    match &parts[..] {
        &[a, b, ..] => (parse_range(a), parse_range(b)),
        _ => unreachable!(),
    }
}

fn parse_range(range: &str) -> (u32, u32) {
    let parts: Vec<&str> = range.split("-").collect();
    match &parts[..] {
        &[start, stop, ..] => (start.parse::<u32>().unwrap(), stop.parse::<u32>().unwrap()),
        _ => unreachable!(),
    }
}

fn a_contains_b_end(a: (u32, u32), b: (u32, u32)) -> bool {
    b.0 <= a.0 && a.0 <= b.1
}

fn part1(input: &Vec<((u32, u32), (u32, u32))>) -> Result<()> {
    let total = input
        .iter()
        .filter(|(range1, range2)| {
            (range1.0 >= range2.0 && range1.1 <= range2.1)
                || (range1.0 <= range2.0 && range1.1 >= range2.1)
        })
        .count();

    println!("Number of pairs containing the other: {}", total);

    Ok(())
}

fn part2(input: &Vec<((u32, u32), (u32, u32))>) -> Result<()> {
    let total = input
        .iter()
        .filter(|(range1, range2)| {
            a_contains_b_end(*range1, *range2) || a_contains_b_end(*range2, *range1)
        })
        .count();

    println!("Number of pairs overlapping: {}", total);

    Ok(())
}
