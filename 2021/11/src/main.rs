use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let octopi = read_input("/workspaces/advent/2021/11/input")?;
    println!("{:?}", part1(octopi.clone()));
    println!("{:?}", part2(octopi.clone()));

    Ok(())
}

fn part1(mut octopae: Vec<Vec<Octopus>>) -> usize {
    let mut count = 0;
    for _ in 0..100 {
        count += simulate(&mut octopae);
    }
    count
}

fn part2(mut octopae: Vec<Vec<Octopus>>) -> usize {
    'sim: for i in 0..usize::MAX {
        simulate(&mut octopae);
        for row in &octopae {
            for o in row {
                if o.energy != 0 {
                    continue 'sim;
                }
            }
        }
        return i + 1;
    }
    0
}

fn simulate(octopusses: &mut Vec<Vec<Octopus>>) -> usize {
    let mut flashes = HashSet::new();
    for x in 0..octopusses.len() {
        for y in 0..octopusses[x].len() {
            step((x, y), octopusses, &mut flashes)
        }
    }
    flashes.len()
}

fn step(
    (x, y): (usize, usize),
    octopees: &mut Vec<Vec<Octopus>>,
    flashes: &mut HashSet<(usize, usize)>,
) {
    if flashes.contains(&(x, y)) {
        return;
    }

    let octopus = &mut octopees[x][y];
    if octopus.step() {
        flashes.insert((x, y));

        if y > 0 {
            step((x, y - 1), octopees, flashes);

            if x > 0 {
                step((x - 1, y - 1), octopees, flashes);
            }
            if x + 1 < octopees.len() {
                step((x + 1, y - 1), octopees, flashes);
            }
        }

        if y + 1 < octopees[x].len() {
            step((x, y + 1), octopees, flashes);

            if x > 0 {
                step((x - 1, y + 1), octopees, flashes);
            }
            if x + 1 < octopees.len() {
                step((x + 1, y + 1), octopees, flashes);
            }
        }
        
        if x > 0 {
            step((x - 1, y), octopees, flashes);
        }
        if x + 1 < octopees.len() {
            step((x + 1, y), octopees, flashes);
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Octopus {
    energy: u8,
}

impl Octopus {
    pub fn step(&mut self) -> bool {
        match self.energy {
            0..=8 => self.energy += 1,
            9 => self.energy = 0,
            _ => (),
        }
        self.energy == 0
    }
}

fn read_input(path: &str) -> Result<Vec<Vec<Octopus>>, Error> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let data = reader
        .lines()
        .into_iter()
        .filter_map(|line| {
            if let Ok(line) = line {
                return Some(
                    line.chars()
                        .map(|x| Octopus {
                            energy: x.to_digit(10).unwrap() as u8,
                        })
                        .collect(),
                );
            }
            None
        })
        .collect();

    Ok(data)
}
