use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let input = File::open("input")?;
    let reader = BufReader::new(input);

    let mut count = 0;
    let mut previous_measurement = 0;
    reader.lines().into_iter().skip(1).for_each(|raw_value| {
        if let Ok(value) = raw_value.unwrap().parse() {
            if value > previous_measurement {
                count += 1;
            }
            previous_measurement = value;
        };
    });

    println!("{}", count);

    Ok(())
}