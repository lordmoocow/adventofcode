use itertools::izip;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let input = File::open("input")?;
    let reader = BufReader::new(input);

    // Read input and parse lines to ints
    let data: Vec<_> = reader
        .lines()
        .into_iter()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect();

    // Get slices of data with an offset for each window
    let a = &data[..];
    let b = &data[1..];
    let c = &data[2..];

    let mut count = 0;
    let mut previous_window: Option<i32> = None;

    // Zip 'em all together
    for (x, y, z) in izip!(a, b, c) {
        let window = x + y + z;
        // If we have a previous value to compare, i.e. not the first window, then do so
        if let Some(previous_window) = previous_window {
            if window > previous_window {
                count += 1;
            }
        }
        previous_window.replace(window);
    }

    println!("{}", count);

    Ok(())
}
