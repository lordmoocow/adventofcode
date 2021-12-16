use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Error;

fn main() -> Result<(), Error> {
    let crabs = read_input("/workspaces/advent/2021/7/input")?;
    let crabs = count(&crabs);

    //println!("Crabs: {:?}", &crabs);
    println!("Fuel 1: {:?}", part1(&crabs));
    println!("Fuel 2: {:?}", part2(&crabs));

    Ok(())
}

fn part1(crabs: &HashMap<usize, usize>) -> usize {
    calc(crabs, |n| n )
}

fn part2(crabs: &HashMap<usize, usize>) -> usize {
    calc(crabs, |n| n * (n + 1) / 2  )
}

fn calc<F>(crabs: &HashMap<usize, usize>, fuel_cost: F) -> usize
where
    F: Fn(usize) -> usize,
{
    let mut fuel_costs = vec![0; *crabs.keys().max().unwrap()];

    for (target, fuel) in fuel_costs.iter_mut().enumerate() {
        for (pos, count) in crabs.iter() {
            let steps = if pos > &target {
                pos - target
            } else {
                target - pos
            };
            *fuel += count * fuel_cost(steps);
        }
    }

    *fuel_costs.iter().min().unwrap()
}

fn count(crabs: &[usize]) -> HashMap<usize, usize> {
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
