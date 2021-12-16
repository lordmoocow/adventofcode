use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let signals = read_input("/workspaces/advent/2021/8/input")?;

    //println!("{:?}", &signals);
    println!("{:?}", part1(&signals));
    // println!("Fuel 2: {:?}", part2(&crabs));

    Ok(())
}

fn part1(signals: &[Signal]) -> usize {
    signals.iter().fold(0, |total, s| {
        total + s.output.iter().filter(|x| (x.len() >= 2 && x.len() <= 4) || x.len() == 7).count()
    })
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
