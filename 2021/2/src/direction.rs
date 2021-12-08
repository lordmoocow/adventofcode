use std::str::FromStr;

pub enum Direction {
    Up(i32),
    Down(i32),
    Forward(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseDirError;

impl FromStr for Direction {
    type Err = ParseDirError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        // normalise casing to be safe
        let value = value.to_lowercase();

        // attempt split into direction / distance
        let mut parts = value.split_whitespace();
        let dir = parts.next();
        let dist = parts.next();

        // nothing to do if either of these are missing
        if dir.is_none() || dist.is_none() {
            return Err(ParseDirError);
        }

        // now we can safely unwrap knowing we have values
        let dir = dir.unwrap();
        let dist = dist.unwrap();

        // attempt to parse distance value
        if let Ok(dist) = dist.parse() {
            match dir {
                "forward" => Ok(Direction::Forward(dist)),
                "up" => Ok(Direction::Up(dist)),
                "down" => Ok(Direction::Down(dist)),
                _ => Err(ParseDirError),
            }
        } else {
            Err(ParseDirError)
        }
    }
}
