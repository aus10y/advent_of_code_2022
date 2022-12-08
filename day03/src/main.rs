use std::collections::btree_set::Intersection;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

const ITEMS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<&str> = input
        .split("\n")
        .into_iter()
        .filter(|line| *line != "")
        .collect();

    let item_values: HashMap<char, usize> =
        HashMap::from_iter(ITEMS.chars().enumerate().map(|(i, v)| (v, i + 1)));

    part1(&lines, &item_values)?;
    part2(&lines, &item_values)?;

    Ok(())
}

fn part1(lines: &Vec<&str>, values: &HashMap<char, usize>) -> Result<()> {
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

fn part2(lines: &Vec<&str>, values: &HashMap<char, usize>) -> Result<()> {
    let score: usize = lines
        .iter()
        .step_by(3)
        .zip(lines.iter().skip(1).step_by(3))
        .zip(lines.iter().skip(2).step_by(3))
        .map(|v| {
            let ((a, b), c) = v;
            (*a, *b, *c)
        })
        .map(|v| {
            let (a, b, c) = v;
            let a: HashSet<char> = HashSet::from_iter(a.chars());
            let b: HashSet<char> = HashSet::from_iter(b.chars());
            let c: HashSet<char> = HashSet::from_iter(c.chars());

            let ab: HashSet<&char> = a.intersection(&b).collect();
            let bc: HashSet<&char> = b.intersection(&c).collect();
            let abc: Vec<&char> = ab.intersection(&bc).map(|c| *c).collect();

            values.get(abc[0]).unwrap()
        })
        .sum();

    println!("Part2 - Sum of priorities: {}", score);

    Ok(())
}
