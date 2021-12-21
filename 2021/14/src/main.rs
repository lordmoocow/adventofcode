use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Error;

fn main() -> Result<(), Error> {
    let (template, rules) = read_input("/workspaces/advent/2021/14/input")?;
    println!("{:?}", &template);
    println!("{:?}", part1(template.clone(), &rules));
    println!("{:?}", part2(template.clone(), &rules));

    Ok(())
}

fn part1(polymer: Vec<char>, rules: &HashMap<String, Rule>) -> u128 {
    polymerize(polymer, rules, 10)
}

fn part2(polymer: Vec<char>, rules: &HashMap<String, Rule>) -> u128 {
    polymerize(polymer, rules, 40)
}

fn polymerize(polymer: Vec<char>, rules: &HashMap<String, Rule>, steps: usize) -> u128 {
    let mut cache = HashMap::new();

    // initial count of each element
    let mut counts = HashMap::new();
    for element in &polymer {
        counts.entry(*element).and_modify(|v| *v += 1).or_insert(1);
    }

    // add extended counts
    let pattern_counts = polymerize_step(&polymer, rules, steps, &mut cache);
    merge_counts(&mut counts, &pattern_counts);

    // calculate score
    let mut counts: Vec<_> = counts.values().collect();
    counts.sort_unstable();
    *counts.last().unwrap_or(&&0) - *counts.first().unwrap_or(&&0)
}

fn polymerize_step(
    polymer: &Vec<char>,
    rules: &HashMap<String, Rule>,
    steps: usize,
    cache: &mut HashMap<(String, usize), HashMap<char, u128>>,
) -> HashMap<char, u128> {
    let mut counts = HashMap::new();

    for (i, pair) in polymer.windows(2).enumerate() {
        // cache to stop going to the moon,
        // a given pair and number of iterations should always
        // produce the same additional no. of polymers
        let cache_key = (pair.iter().collect(), steps);
        if cache.contains_key(&cache_key) {
            merge_counts(&mut counts, &cache.entry(cache_key).or_default());
            continue;
        }

        let mut window_count = HashMap::new();
        let mut extended_polymer = polymer.clone();
        if let Some(rule) = rules.get(&cache_key.0) {
            extended_polymer.insert(i + 1, rule.result);

            window_count
                .entry(rule.result)
                .and_modify(|v| *v += 1)
                .or_insert(1);

            if steps > 1 {
                let next: Vec<char> = extended_polymer[i..i + 3]
                    .iter()
                    .map(|x| *x)
                    .collect();
                merge_counts(&mut window_count, &polymerize_step(&next, rules, steps - 1, cache));
            }
        }
        
        cache.insert(cache_key, window_count.clone());
        merge_counts(&mut counts, &window_count);
    }
    counts
}

fn merge_counts(a: &mut HashMap<char, u128>, b: &HashMap<char, u128>) {
    for kvp in b.iter() {
        a.entry(*kvp.0)
            .and_modify(|v| *v += *kvp.1)
            .or_insert(*kvp.1);
    }
}

#[derive(Debug, Default)]
struct Rule {
    pair: [char; 2],
    result: char,
}

fn read_input(path: &str) -> Result<(Vec<char>, HashMap<String, Rule>), Error> {
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
                return Some((
                    input[0][..2].iter().collect::<String>(),
                    Rule {
                        pair: input[0][..2].try_into().unwrap(),
                        result: input[1][0],
                    },
                ));
            }
            None
        })
        .collect();

    Ok((template, rules))
}
