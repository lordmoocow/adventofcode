mod direction;
mod submarine;

use crate::{direction::Direction, submarine::Submarine};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let mut sub = Submarine::default();
    if let Ok(directions) = read_input("input") {
        for dir in &directions {
            sub.drive(dir);
        }
    }

    println!("Part 1: {}", sub.location());

    Ok(())
}

fn read_input(path: &str) -> Result<Vec<Direction>, Error> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let data = reader
        .lines()
        .into_iter()
        .filter_map(|raw_input| {
            if let Ok(raw_input) = raw_input {
                if let Ok(dir) = raw_input.parse() {
                    return Some(dir);
                }
            }
            None
        })
        .collect();

    Ok(data)
}
