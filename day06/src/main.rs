use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn first_unique_n(n: usize, input: &str) -> u32 {
    let mut marker: u32 = 0;
    let mut window: VecDeque<char> = VecDeque::with_capacity(n);
    let mut memory: HashMap<char, u32> = HashMap::with_capacity(n);

    // Prime the window
    for c in input.chars().take(n) {
        window.push_front(c);
        match memory.get(&c) {
            Some(v) => memory.insert(c, v + 1),
            None => memory.insert(c, 1),
        };
    }

    for (i, c) in input
        .chars()
        .filter(|c| *c != '\n')
        .skip(n)
        .enumerate()
        .map(|(i, c)| (i + n, c))
    {
        match memory.len() {
            len if len == n => {
                marker = i as u32;
                break;
            }
            _ => {
                // Maintain the window
                let oldest = window.pop_back().unwrap();
                window.push_front(c);

                // If the old and new char are the same, we can skip inc / dec operations.
                if oldest == c {
                    continue;
                }

                // Decrement or remove the old char
                match memory.get(&oldest) {
                    Some(v) => match v {
                        1 => memory.remove(&oldest),
                        _ => memory.insert(oldest, v - 1),
                    },
                    None => panic!(),
                };

                // Add or increment the new char
                match memory.get(&c) {
                    Some(v) => memory.insert(c, v + 1),
                    None => memory.insert(c, 1),
                };
            }
        }
    }

    marker
}

fn part1(input: &str) -> Result<()> {
    println!("First marker after character: {}", first_unique_n(4, input));

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    println!("Start of message after character: {}", first_unique_n(14, input));

    Ok(())
}
