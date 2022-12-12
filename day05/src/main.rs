use std::{
    io::{self, Read},
    num,
};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let data: Vec<&str> = input.split("\n\n").collect();
    let layout: Vec<&str> = data[0].split("\n").collect();
    let raw_procedure: Vec<&str> = data[1].split("\n").filter(|line| *line != "").collect();

    let mut stacks = parse_stacks_layout(&layout);
    let procedure = parse_procedure(&raw_procedure);

    part1(&mut stacks, procedure)?;
    part2(&input)?;

    Ok(())
}

fn char_index_to_stack(i: usize) -> usize {
    (i - 1) / 4
}

fn parse_stacks_layout(layout: &Vec<&str>) -> Vec<Vec<char>> {
    let num_stacks = (*layout.last().unwrap())
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .last()
        .unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::from_iter((0..num_stacks).map(|_| Vec::new()));

    let crate_iter = layout.iter().rev().skip(1).map(|line| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| c.is_ascii_alphabetic())
    });

    for line_iter in crate_iter {
        for (i, c) in line_iter {
            stacks.get_mut(char_index_to_stack(i)).unwrap().push(c);
        }
    }

    stacks
}

fn parse_procedure(procedure: &Vec<&str>) -> Vec<(usize, usize, usize)> {
    procedure
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            match &parts[..] {
                &[_, a, _, b, _, c, ..] => (
                    a.parse::<usize>().unwrap(),
                    b.parse::<usize>().unwrap() - 1,
                    c.parse::<usize>().unwrap() - 1,
                ),
                _ => panic!(),
            }
        })
        .collect()
}

fn part1(stacks: &mut Vec<Vec<char>>, procedure: Vec<(usize, usize, usize)>) -> Result<()> {
    for (num, src, dst) in procedure {
        for _ in 0..num {
            let thing = stacks[src].pop().unwrap();
            stacks[dst].push(thing);
        }
    }

    for i in 0..stacks.len() {
        print!("{}", stacks[i].pop().unwrap());
    }
    println!("");

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    Ok(())
}
