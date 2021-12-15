mod fish;

use crate::fish::Simulator;
use crate::fish::Lanternfish;
use std::fs::read_to_string;
use std::io::Error;

fn main() -> Result<(), Error> {
    let fish = read_input("/workspaces/advent/2021/6/input")?;
    let mut sim = Simulator::from(fish);

    println!("Fish: {:?}", sim.simulate(256));

    Ok(())
}

fn read_input(path: &str) -> Result<Vec<Lanternfish>, Error> {
    let input = read_to_string(path)?;
    let fish = input
        .split(',')
        .filter_map(|age| age.trim().parse().ok())
        .collect();
    Ok(fish)
}
