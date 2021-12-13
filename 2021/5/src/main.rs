#![feature(slice_group_by,array_methods)]

mod hydro;

use crate::hydro::VentLine;
use crate::hydro::{Vent};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let vents = read_input("/workspaces/advent/2021/5/input")?;
    let processor = hydro::VentProcessor::from(vents);

    println!("Danger Vents: {:?}", processor.calc());

    Ok(())
}

fn read_input(path: &str) -> Result<Vec<hydro::VentLine>, Error> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let data = reader
        .lines()
        .into_iter()
        .map(|raw_input| {
            let raw_input = raw_input.unwrap();
            let mut parsed = raw_input
                .split(" -> ")
                .map(|x| x.parse::<Vent>().unwrap());
            VentLine::from(parsed.next().unwrap(), parsed.next().unwrap())
        })
        .collect();

    Ok(data)
}
