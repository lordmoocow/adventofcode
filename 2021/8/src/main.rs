use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let signals = read_input("/workspaces/advent/2021/8/input")?;

    println!("{:?}", part1(&signals));
    println!("{:?}", part2(&signals));

    Ok(())
}

fn part1(signals: &[Signal]) -> usize {
    signals.iter().fold(0, |total, s| {
        total
            + s.output
                .iter()
                .filter(|x| (x.len() >= 2 && x.len() <= 4) || x.len() == 7)
                .count()
    })
}

fn part2(signals: &[Signal]) -> usize {
    let mut tmp = vec![vec![' '; 0]; 11];
    signals.iter().fold(0, |total, s| {
        // determine known input patterns for this signal
        for input in &s.input {
            match input.len() {
                2 => tmp[1] = input.chars().collect(),
                3 => tmp[7] = input.chars().collect(),
                4 => tmp[4] = input.chars().collect(),
                7 => tmp[8] = input.chars().collect(),
                _ => (),
            };
        }
        // get output value
        let display = &s
            .output
            .iter()
            .map(|digit| digit.chars().collect())
            .map(|digit: Vec<char>| match digit.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                5 => {
                    if slice_contains(&digit, &tmp[1]) {
                        3
                    } else if slice_remains(&digit, &tmp[4]) == 1 {
                        5
                    } else {
                        2
                    }
                }
                6 => {
                    if slice_contains(&digit, &tmp[4]) {
                        9
                    } else if slice_contains(&digit, &tmp[1]) {
                        0
                    } else {
                        6
                    }
                }
                7 => 8,
                _ => 0_usize,
            })
            // concat the array of digits into a 4 digit int
            .fold(0, |acc, i| acc * 10 + i);
        total + display
    })
}

fn slice_contains(a: &[char], b: &[char]) -> bool {
    for x in b {
        if !a.contains(x) {
            return false;
        }
    }
    true
}

fn slice_remains(a: &[char], b: &[char]) -> usize {
    b.iter()
        .fold(0, |total, c| total + if a.contains(c) { 0 } else { 1 })
}

fn read_input(path: &str) -> Result<Vec<Signal>, Error> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let data = reader
        .lines()
        .into_iter()
        .filter_map(|line| {
            if let Ok(line) = line {
                let mut signal = Signal::default();
                let mut line = line.split('|');
                for (i, input) in line.next().unwrap().trim().split(' ').enumerate() {
                    signal.input[i] = input.trim().to_string();
                }
                for (i, output) in line.next().unwrap().trim().split(' ').enumerate() {
                    signal.output[i] = output.trim().to_string();
                }
                return Some(signal);
            };
            None
        })
        .collect();

    Ok(data)
}

#[derive(Default, Debug)]
struct Signal {
    input: [String; 10],
    output: [String; 4],
}
