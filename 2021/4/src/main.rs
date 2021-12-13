mod bingo;

use std::fs::read_to_string;
use std::io::Error;

fn main() -> Result<(), Error> {
    let mut system = read_input("/workspaces/advent/2021/4/input")?;

    println!("Draw: {:?}", system.draw_sequence);
    if let Some(board) = system.winner() {
        println!("{:?}", board);
        println!("Winning Score: {}", board.get_score());
    }
    if let Some(board) = system.loser() {
        println!("{:?}", board);
        println!("Losing Score: {}", board.get_score());
    }

    Ok(())
}

fn read_input(path: &str) -> Result<bingo::System, Error> {
    let input = read_to_string(path)?;
    // each section of the input is separated by double line feed
    let mut iter = input.split("\n\n");

    let mut system = bingo::System::default();
    // the first item in the input is the draw sequence
    system.set_draw_sequence(iter.next().unwrap());
    // everything else defines a bingo board which we can parse
    for data in iter.filter_map(|x| x.parse().ok()) {
        system.add_board(data);
    }

    Ok(system)
}
