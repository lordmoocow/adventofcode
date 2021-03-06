use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;

fn main() -> Result<(), Error> {
    let risk_data = read_input("/workspaces/advent/2021/15/input")?;
    //println!("{:?}", &risk_data);
    println!("{:?}", part1(&risk_data));
    println!("{:?}", part2(&risk_data));

    Ok(())
}

fn part1(risk_data: &HashMap<(isize, isize), usize>) -> usize {
    explore(&risk_data)
}

fn part2(risk_data: &HashMap<(isize, isize), usize>) -> usize {
    let max = &risk_data.keys().max().unwrap_or(&(0, 0));
    let risk_data: HashMap<(isize, isize), usize> = risk_data
        .iter()
        .flat_map(|((x, y), r)| {
            (0..5).flat_map(move |n| {
                let dx = n * (max.0 + 1);
                (0..5).map(move |nn| {
                    let dy = nn * (max.1 + 1);
                    (
                        (*x + dx, *y + dy),
                        (*r + dx as usize + dy as usize - 1) % 9 + 1,
                    )
                })
            })
        })
        .collect();
    explore(&risk_data)
}

fn explore(map: &HashMap<(isize, isize), usize>) -> usize {
    let start = (0, 0);
    let target = &map.keys().max().unwrap_or(&(0, 0));

    // manhatten distance as heuristic
    let h = |(x, y): (isize, isize)| (x - target.0 + y - target.1).abs() as usize;
    //let h = |v| *map.get(&v).unwrap_or(&0);

    let mut queue = BinaryHeap::new();

    // add start position to queue
    queue.push(Reverse(State {
        c: h(start),
        u: start,
    }));

    // store the risk score to get to each point on the map
    let mut dist = HashMap::new();

    // take from the queue until we stop adding to it
    while let Some(Reverse(State { c: _, u })) = queue.pop() {
        // get the current score for this position (if not set we use an arbitrarily large score)
        let dist_u = *dist.entry(u).or_insert(0);

        for v in [u]
            .iter()
            .cycle()
            .zip([(0, 1), (1, 0), (-1, 0), (0, -1)].iter())
            .map(|((x, y), (dx, dy))| (x + dx, y + dy))
        {
            if let Some(risk) = map.get(&v) {
                let alt = dist_u + risk;
                let dist_v = dist.entry(v).or_insert(usize::MAX);
                if alt < *dist_v {
                    *dist_v = alt;
                    if &&v != target {
                        queue.push(Reverse(State {
                            c: alt + h(v),
                            u: v,
                        }))
                    }
                }
            }
        }
    }

    *dist.get(target).unwrap_or(&0)
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    c: usize,
    u: (isize, isize),
}

fn read_input(path: &str) -> Result<HashMap<(isize, isize), usize>, Error> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let data = reader
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(move |(y, risk)| {
                    (
                        (x as isize, y as isize),
                        risk.to_digit(10).unwrap() as usize,
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect();

    Ok(data)
}
