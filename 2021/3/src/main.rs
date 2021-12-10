mod diagnostics;

use diagnostics::Diagnostics;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let data = read_input("/workspaces/advent/2021/3/input")?;
    let diagnostics = Diagnostics::from(data);
    println!(
        "power consumption:\t {}",
        diagnostics.get_power_consumption()
    );
    println!(
        "life support:\t\t {}",
        diagnostics.get_life_support_rating()
    );

    Ok(())
}

fn read_input(path: &str) -> Result<Vec<usize>, Error> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let data = reader
        .lines()
        .into_iter()
        .filter_map(|raw_input| {
            if let Ok(raw_input) = raw_input {
                let mut reading = 0;
                for (i, c) in raw_input.chars().rev().enumerate() {
                    if c == '1' {
                        reading |= 1 << i;
                    }
                }
                return Some(reading);
            }
            None
        })
        .collect();

    Ok(data)
}
