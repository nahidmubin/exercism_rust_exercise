// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use crate::Direction::{East, North, South, West};

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North = 90,
    East = 180,
    South = 270,
    West = 360,
}

pub struct Robot{
    x: i32,
    y: i32,
    d: Direction
}

impl Robot {
        pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let direction_value = (self.d as i32 + 90) % 360;
        let direction = Direction::try_from(direction_value).unwrap();
        Self { x: self.x, y: self.y, d: direction }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let direction_value = (self.d as i32 - 90) % 360;
        let direction = Direction::try_from(direction_value).unwrap();
        Self { x: self.x, y: self.y, d: direction }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let mut x = self.x;
        let mut y = self.y;

        if self.d == North {
            y += 1;
        } else if self.d == South {
            y -= 1;
        } else if self.d == East {
            x += 1;
        } else {
            x -= 1;
        }

        Self { x, y, d: self.d }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut current_status = self;
        for ch in instructions.chars() {
            if ch == 'R' {
                current_status = current_status.turn_right();
            }
            else if ch == 'L' {
                current_status = current_status.turn_left();
            }
            else if ch == 'A' {
                current_status = current_status.advance();
            }
            else {
                ();
            }
        }

        current_status
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}


impl TryFrom<i32> for Direction {
    
    type Error = String;

    fn try_from(direction_value: i32) -> Result<Self, Self::Error> {
        match direction_value {
            90 => Ok(North),
            180 => Ok(East),
            270 => Ok(South),
            0 => Ok(West),
            _ => Err(format!("Invalid Direction value: {}", direction_value))
        }
    
    }
}