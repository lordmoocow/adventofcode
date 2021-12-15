use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Error;

fn main() -> Result<(), Error> {
    let crabs = read_input("/workspaces/advent/2021/7/input")?;
    let crabs = count(&crabs);

    println!("Crabs: {:?}", &crabs);
    println!("Fuel: {:?}", calc(&crabs));

    Ok(())
}

fn calc(crabs: &HashMap<usize,usize>) -> usize {
    let mut fuel_costs = HashMap::new();
    for pos in 0..*crabs.keys().max().unwrap() {
        fuel_costs.insert(pos, 0);
    }

    for (target, fuel) in fuel_costs.iter_mut() {
        for (pos, submarines) in crabs.iter() {
            let steps = if pos > target {
                *pos - *target
            } else {
                *target - *pos
            };
            *fuel += submarines * steps;
        }
    }

    *fuel_costs.values().min().unwrap()
}

fn count(crabs: &[usize]) -> HashMap<usize,usize> {
    let mut counts = HashMap::new();
    for crab in crabs {
        *counts.entry(*crab).or_insert(0) += 1;
    }
    counts
}

fn read_input(path: &str) -> Result<Vec<usize>, Error> {
    let input = read_to_string(path)?;
    let crabs = input
        .split(',')
        .filter_map(|position| position.trim().parse().ok())
        .collect();
    Ok(crabs)
}
