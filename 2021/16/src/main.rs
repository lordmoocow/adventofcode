mod bits;

use crate::bits::{Packet};
use std::fs::read_to_string;
use std::io::Error;

fn main() -> Result<(), Error> {
    let transmission = read_input("/workspaces/advent/2021/16/input")?;
    let packet = Packet::from_hex(&transmission);

    println!("{:?}", part1(&packet));
    println!("{:?}", part2(&packet));

    Ok(())
}

fn part1(packet: &Packet) -> usize {
    packet.version_sum()
}

fn part2(packet: &Packet) -> usize {
    packet.value()
}

fn read_input(path: &str) -> Result<String, Error> {
    let input = read_to_string(path)?;
    Ok(input.trim().to_owned())
}
