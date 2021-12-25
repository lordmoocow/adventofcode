mod snailfish;

//use crate::snailfish::Number;
use crate::snailfish::Number;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::io::Error;
use std::ops::Range;

fn main() -> Result<(), Error> {
    let snailfish = read_input("/workspaces/advent/2021/18/input")?;
    //println!("{:?}", &snailfish);

    println!("{:?}", part1(&snailfish));
    println!("{:?}", part2(&snailfish));

    Ok(())
}

fn part1(snailfish: &[String]) -> usize {
    let n = process(snailfish);
    println!("{:?}", n);
    n.magnitude()
}

fn part2(snailfish: &[String]) -> usize {
    let mut max = 0;
    for x in snailfish {
        for y in snailfish {
            if x != y {
                let n = process(&[x.to_string(), y.to_string()]);
                max = max.max(n.magnitude());
            }
        }
    }
    max
}

fn process(snailfish: &[String]) -> Number {
    let mut result: String = snailfish[0].to_owned();

    for s in snailfish.iter().skip(1) {
        // add next number to current sum
        if !s.is_empty() && !result.is_empty() {
            result = format!("[{},{}]", result, s);
        }

        let mut depth;
        let mut dbuf;
        let mut sbuf;
        let mut can_split;
        let mut reductions = VecDeque::new();
        // loop until all actions resulting from the addition are processed
        loop {
            depth = 0;
            dbuf = 0;
            sbuf = 0;
            can_split = false;
            for (i, c) in result.chars().enumerate() {
                match c {
                    '[' | ']' | ',' => {
                        match c {
                            '[' => {
                                depth += 1;
                                dbuf = 0;
                            }
                            ']' => depth -= 1,
                            _ => (),
                        }
                        // sbuf counts the number of numeric chars without interruption
                        can_split = sbuf > 1;
                        // allow continue if we need to split
                        if !can_split && depth <= 4 {
                            sbuf = 0;
                            continue;
                        }
                    }
                    _ => sbuf += 1,
                }

                // if explosion depth has been reached track position until we come back up
                if depth > 4 {
                    if dbuf == 0 {
                        dbuf = i;
                    }
                    continue;
                } else if dbuf > 0 {
                    // explode!
                    reductions.push_front((true, dbuf..i + 1));
                    // queuing didn't work because the indexes change when processing
                    // could probably do something clever with slices and offsets?
                    // just break the loop instead
                    break;
                } else if can_split {
                    reductions.push_back((false, i - sbuf..i));
                }
            }

            if reductions.len() == 0 {
                // no more explosions!
                break;
            } else {
                if let Some((ex, range)) = reductions.pop_front() {
                    if ex {
                        result = explode(&range, &result);
                    } else {
                        result = split(&range, &result);
                    }
                    //println!("{}", result);
                }
                reductions.clear();
            }
        }
    }

    result.parse().unwrap()
}

fn explode(action: &Range<usize>, result: &str) -> String {
    let mut exploded = Vec::new();
    let (a, b) = result.split_at(action.start);
    let (b, c) = b.split_at(action.end - action.start);

    // parse left and right value
    let values: Vec<_> = b[1..b.len() - 1]
        .split(',')
        .take(2)
        .map(|v| v.parse::<u8>().unwrap())
        .collect();

    let pattern = |c| c == '[' || c == ']' || c == ',';

    // add left to nearest left if available
    // ffs this started simple
    let mut ai = 0;
    for (i, x) in a.chars().rev().enumerate() {
        if x.is_digit(10) {
            ai = i;
            break;
        }
    }
    exploded.push(if ai > 0 {
        let (a1, a3) = a.split_at(a.len() - ai);
        let (a1, a2) = a1.split_at(a1.rfind(pattern).unwrap() + 1);
        let a2 = a2.parse::<u8>().unwrap() + values[0];
        format!("{}{}{}", a1, a2, a3)
    } else {
        a.to_string()
    });

    // add right to nearest right if available
    // ffs this started simple
    let mut ci = 0;
    for (i, x) in c.chars().enumerate() {
        if x.is_digit(10) {
            ci = i;
            break;
        }
    }
    exploded.push(if ci > 0 {
        let (c1, c2) = c.split_at(ci);
        let (c2, c3) = c2.split_at(c2.find(|c| c == ']' || c == ',').unwrap());
        let c2 = c2.parse::<u8>().unwrap() + values[1];
        format!("{}{}{}", c1, c2, c3)
    } else {
        c.to_string()
    });

    // exploded value becomes 0
    exploded.join("0")
}

fn split(action: &Range<usize>, result: &str) -> String {
    //let mut split = Vec::new();
    let (a, b) = result.split_at(action.start);
    let (b, c) = b.split_at(action.end - action.start);

    let b = b.parse::<f32>().unwrap() / 2f32;

    format!("{}[{},{}]{}", a, b.floor() as u8, b.ceil() as u8, c)
}

fn read_input(path: &str) -> Result<Vec<String>, Error> {
    let input = read_to_string(path)?;

    let input = input.trim().split('\n').map(|s| s.to_owned()).collect();

    Ok(input)
}
