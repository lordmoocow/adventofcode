use crate::Direction;

#[derive(Default,Debug)]
pub struct Submarine {
    x: i32, // horizontal
    y: i32, // depth
}

impl Submarine {
    pub fn drive(&mut self, dir: &Direction) {
        match dir {
            Direction::Forward(distance) => self.x += distance,
            Direction::Up(distance) => self.y -= distance,
            Direction::Down(distance) => self.y += distance,
        }
    }

    pub fn location(&self) -> i32 {
        self.x * self.y
    }
}
