use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Error;

fn main() -> Result<(), Error> {
    let (template, rules) = read_input("/workspaces/advent/2021/14/input")?;
    println!("{:?}", &template);
    println!("{:?}", part1(&template, &rules));

    Ok(())
}

fn part1(polymer: &Vec<char>, rules: &[Rule]) -> usize {
    let mut extended_polymer = polymer.clone();
    for _ in 0..10 {
        extended_polymer = polymerize(&extended_polymer, rules);
    }

    let mut counts = HashMap::new();
    for element in extended_polymer {
        let counter = counts.entry(element).or_insert(0);
        *counter += 1;
    }
    let mut counts: Vec<_> = counts.values().collect();
    counts.sort_unstable();
    *counts.last().unwrap_or(&&0) - *counts.first().unwrap_or(&&0)
}

fn polymerize(polymer: &[char], rules: &[Rule]) -> Vec<char> {
    // a place to store the extended polymer result
    let mut extensions = Vec::default();

    // record each extension to the polymer and the position it should be inserted
    for (i, pair) in polymer.windows(2).enumerate() {
        for rule in rules {
            if rule.pair == pair {
                extensions.push((rule.result, i + 1));
            }
        }
    }

    // collect all extensions into the final polymer,
    // offset each index as we increase the size of it with each addition
    // we collated the extensions in order of insertion so it will always be sequential
    extensions.iter().enumerate().fold(
        polymer.to_vec(),
        |mut extended_polymer, (offset, (extension, i))| {
            extended_polymer.insert(i + offset, *extension);
            extended_polymer
        },
    )
}

#[derive(Debug, Default)]
struct Rule {
    pair: [char; 2],
    result: char,
}

fn read_input(path: &str) -> Result<(Vec<char>, Vec<Rule>), Error> {
    let input = read_to_string(path)?;
    // each section of the input is separated by double line feed
    let mut iter = input.split("\n\n");

    let template = iter.next().unwrap().chars().collect();

    let rules = iter
        .next()
        .unwrap()
        .split('\n')
        .filter_map(|x| {
            let input: Vec<Vec<char>> = x.split(" -> ").map(|x| x.chars().collect()).collect();
            if input[0].len() == 2 {
                return Some(Rule {
                    pair: input[0][..2].try_into().unwrap(),
                    result: input[1][0],
                });
            }
            None
        })
        .collect();

    Ok((template, rules))
}
