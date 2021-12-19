use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let heightmap = read_input("/workspaces/advent/2021/9/input")?;

    println!("{:?}", &heightmap);
    println!("{:?}", part1(&heightmap));
    println!("{:?}", part2(&heightmap));

    Ok(())
}

fn part1(heightmap: &Vec<Vec<usize>>) -> usize {
    let lowpoints = lowpoints(heightmap);
    lowpoints
        .iter()
        .fold(0, |acc, (x, y)| acc + heightmap[*x][*y] + 1)
}

fn part2(heightmap: &Vec<Vec<usize>>) -> usize {
    let lowpoints = lowpoints(&heightmap);
    let mut visited = HashSet::default(); // need keep track of the points that have already been counted
    let mut basins = Vec::default();

    for point in lowpoints {
        basins.push(search_basin(&point, &heightmap, &mut visited))
    }

    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

fn search_basin(
    lowpoint: &(usize, usize),
    heightmap: &Vec<Vec<usize>>,
    mut visited: &mut HashSet<(usize, usize)>,
) -> usize {
    let (x, y) = *lowpoint;
    let mut count = 0;
    if !visited.contains(&(x, y)) {
        // since we have now visited this, count it (we don't visit edges at all)
        visited.insert(*lowpoint);
        count = 1;

        if y > 0 && heightmap[x][y - 1] < 9 {
            count += search_basin(&(x, y - 1), &heightmap, &mut visited);
        }
        if y + 1 < heightmap[x].len() && heightmap[x][y + 1] < 9 {
            count += search_basin(&(x, y + 1), &heightmap, &mut visited);
        }
        if x > 0 && heightmap[x - 1][y] < 9 {
            count += search_basin(&(x - 1, y), &heightmap, &mut visited);
        }
        if x + 1 < heightmap.len() && heightmap[x + 1][y] < 9 {
            count += search_basin(&(x + 1, y), &heightmap, &mut visited);
        }
    }
    count
}

fn lowpoints(heightmap: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut lowpoints = Vec::default();
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
            lowpoints.push((x, y));
        }
    }
    lowpoints
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
