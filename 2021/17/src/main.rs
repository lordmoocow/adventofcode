mod probe;

use crate::probe::Velocity;
use crate::probe::Probe;
use crate::probe::Position;
use nalgebra::Vector2;
use std::fs::read_to_string;
use std::io::Error;

fn main() -> Result<(), Error> {
    let target_area = read_input("/workspaces/advent/2021/17/input")?;
    println!("{:?}", &target_area);

    println!("{:?}", part1(&target_area));
    println!("{:?}", part2(&target_area));

    Ok(())
}

fn part1(target: &(Position, Position)) -> isize {
    // we need to hit the minimum y coordinate on the nearest edge
    let mut n = target.0.y;

    // the highest y position we can reach is at the point where
    // the y velocity = 0, and therefore begins to succumb to gravity
    // because of the linearity of gravity to steps, we can do this as a function of the number of steps
    // i think.. honestly i'm not even sure why this works anymore
    n = n * (n + 1);

    // for the peak, we need the half way point
    n / 2
}

fn part2(target: &(Position, Position)) -> usize {
    let mut count = 0;
    let mut vel = Velocity::new(0, 0);
    
    // for possible y range, outside of this it's impossible to hit target
    for y in target.0.y..=target.0.y.abs() {
        vel.y = y;

        // for possible x range, also impossible to hit outside of this range
        // the min x velocity is actually higher than 1 but I cba to figure out the formula...
        // when target.0.x = 20, min_x = 4
        // when target.0.x = 155, min_x = 18
        for x in 1..=target.1.x {
            vel.x = x;
            if Probe::launch(&vel, target) {
                count += 1;
            }
        }
    }

    count
}

fn read_input(path: &str) -> Result<(Position, Position), Error> {
    let input = read_to_string(path)?;
    let input: Vec<Vector2<isize>> = input
        .trim()
        .split(", ")
        .flat_map(|x| {
            x.split('=')
                .filter_map(|s| s.split_once(".."))
                .map(|(v1, v2)| Vector2::new(v1.parse().unwrap(), v2.parse().unwrap()))
        })
        .collect();

    let min = Position::new(
        std::cmp::min(input[0][0], input[0][1]),
        std::cmp::min(input[1][0], input[1][1]),
    );
    let max = Position::new(
        std::cmp::max(input[0][0], input[0][1]),
        std::cmp::max(input[1][0], input[1][1]),
    );
    Ok((min, max))
}
