use crate::Direction;

#[derive(Default, Debug)]
pub struct Submarine {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    pub fn drive(&mut self, dir: &Direction) {
        match dir {
            Direction::Forward(distance) => {
                self.horizontal += distance;
                self.depth += self.aim * distance;
            }
            Direction::Up(distance) => self.aim -= distance,
            Direction::Down(distance) => self.aim += distance,
        }
    }

    pub fn location(&self) -> i32 {
        self.horizontal * self.depth
    }
}
