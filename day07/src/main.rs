use std::collections::VecDeque;
use std::io::{self, Read};

use regex::Regex;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

/*

# Thoughts

- Directory names are not globally unique across the "filesystem"
- Need to maintain a stack of directories.
  - $ cd the_dir, pushes "the_dir" to the stack
  - $ cd .., pops from the stack
  - Every entry of the stack is a parent of the current directory.

# Questions

- Can a directory be entered more than once?

# Plan

- Iterate over the lines
- `$ ls` lines may be ignored
- `$ cd <...>` lines push / pop from the directory stack
  - When we pop from the directory stack, store the directory in a Vec so that
    we may later compare sizes.
- `<size> <file>` lines mean that we need to add the files size to every parent
  directory in the stack.
*/

struct Search {
    curr: Regex,
    child: Regex,
    file: Regex,
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<&str> = input.split("\n").filter(|line| *line != "").collect();

    let search = Search {
        curr: Regex::new(r"^\$ cd ([/\w.]*)$").unwrap(), // Capture the current directory
        child: Regex::new(r"^dir ([\w]*)$").unwrap(),    // Capture a child directory
        file: Regex::new(r"^(\d*) (\w+\.*\w+)$").unwrap(), // Capture a file size and name
    };

    let directories = directory_totals(&lines, &search);

    part1(&directories)?;
    part2(&directories)?;

    Ok(())
}

fn directory_totals<'a>(lines: &Vec<&'a str>, search: &Search) -> Vec<(&'a str, u32)> {
    let mut stack: VecDeque<(&str, u32)> = VecDeque::with_capacity(100);
    let mut finished: Vec<(&str, u32)> = Vec::with_capacity(1000);

    for line in lines.iter() {
        // What to do when chanding directory
        match search.curr.captures(*line) {
            Some(captures) => {
                if captures.len() != 0 {
                    match captures.get(1).unwrap().as_str() {
                        ".." => {
                            finished.push(stack.pop_front().unwrap());
                        }
                        dir => {
                            stack.push_front((dir, 0));
                        }
                    }

                    continue;
                }
            }
            None => (),
        }

        // What to do with a child directory listing
        match search.child.captures(*line) {
            Some(captures) => {
                if captures.len() != 0 {
                    let _child_dir = captures.get(1).unwrap().as_str();

                    continue;
                }
            }
            None => (),
        }

        // What to do with a child file
        match search.file.captures(*line) {
            Some(captures) => {
                if captures.len() != 0 {
                    let file_size = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let _file = captures.get(2).unwrap().as_str();

                    for (_dir, dir_size) in stack.iter_mut() {
                        *dir_size += file_size;
                    }

                    continue;
                }
            }
            None => (),
        }
    }

    while let Some(t) = stack.pop_front() {
        finished.push(t);
    }

    finished
}

fn part1(directories: &Vec<(&str, u32)>) -> Result<()> {
    let mut below_threshold: Vec<(&str, u32)> = directories
        .iter()
        .filter(|x| x.1 <= 100_000)
        .map(|x| *x)
        .collect();
    below_threshold.sort_by(|a, b| a.1.cmp(&b.1));

    let total_below_threshold: u32 = below_threshold.iter().map(|x| x.1).sum();

    println!("\nDir sum: {:?}", total_below_threshold);

    Ok(())
}

fn part2(directories: &Vec<(&str, u32)>) -> Result<()> {
    let mut directories = directories.clone();
    directories.sort_by(|a, b| a.1.cmp(&b.1));

    let (_, size_used) = directories.last().unwrap().clone();

    for (_dir, size) in directories {
        if 30_000_000 <= (70_000_000 - size_used + size) {
            println!("Size of threshold dir: {:?}", size);
            break;
        }
    }

    Ok(())
}
