use std::str::FromStr;

const ADULT: usize = 6;
const NEWBORN: usize = 8;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Lanternfish {
    age: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseFishErr;

impl FromStr for Lanternfish {
    type Err = ParseFishErr;

    fn from_str(data: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        if let Ok(age) = data.parse() {
            return Ok(Self { age });
        }
        Err(ParseFishErr)
    }
}

#[derive(Debug)]
pub struct Simulator {
    fish: Vec<Lanternfish>,
}

impl Simulator {
    pub fn from(fish: Vec<Lanternfish>) -> Self {
        Self { fish }
    }

    pub fn simulate(&mut self, days: usize) -> u128 {
        let mut counts: [u128; NEWBORN + 1] = [0; NEWBORN + 1];
        // count how many of each age we have in our seed pool
        for fish in &self.fish {
            counts[fish.age] += 1;
        }

        for _ in 0..days {
            // shift all the counts down by 1 index each day (0 wraps to the end to create new borns)
            counts.rotate_left(1);
            // the number of new borns is also the number of fish that need to reset as adults
            counts[ADULT] += counts[NEWBORN];
        }
        counts.iter().sum()
    }
}
