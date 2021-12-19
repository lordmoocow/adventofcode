use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let heightmap = read_input("/workspaces/advent/2021/9/input")?;

    println!("{:?}", &heightmap);
    println!("{:?}", part1(&heightmap));
    // println!("Fuel 2: {:?}", part2(&crabs));

    Ok(())
}

fn part1(heightmap: &Vec<Vec<usize>>) -> usize {
    let mut total = 0;
    for (x, row) in heightmap.iter().enumerate() {
        for (y, digit) in row.iter().enumerate() {
            if y > 0 && &row[y - 1] <= digit {
                continue;
            } else if y + 1 < row.len() && &row[y + 1] <= digit {
                continue;
            } else if x > 0 && &heightmap[x - 1][y] <= digit {
                continue;
            } else if x + 1 < heightmap.len() && &heightmap[x + 1][y] <= digit {
                continue;
            }
            println!("{:?}", &digit);
            total += digit + 1;
        }
    }
    total
}

fn read_input(path: &str) -> Result<Vec<Vec<usize>>, Error> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let data = reader
        .lines()
        .into_iter()
        .filter_map(|line| {
            if let Ok(line) = line {
                return Some(
                    line.chars()
                        .filter_map(|reading| {
                            if let Some(digit) = reading.to_digit(10) {
                                Some(digit as usize)
                            } else {
                                None
                            }
                        })
                        .collect(),
                );
            }
            None
        })
        .collect();

    Ok(data)
}
