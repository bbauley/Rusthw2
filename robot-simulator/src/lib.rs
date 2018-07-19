// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
  coordinate: (isize, isize),
  facing: Direction,
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot {coordinate: (x, y), facing: d}
    }

    pub fn turn_right(self) -> Self {
      let mut direction = self.facing;
        match direction {
          Direction::North =>  direction = Direction::East,
          Direction::East  =>  direction = Direction::South,  
          Direction::South =>  direction = Direction::West,
          Direction::West  =>  direction = Direction::North,
        }
      Robot {coordinate: self.coordinate, facing: direction}
    }

    pub fn turn_left(self) -> Self {
      let mut direction = self.facing;
        match direction {
          Direction::North =>  direction = Direction::West,
          Direction::East  =>  direction = Direction::North,  
          Direction::South =>  direction = Direction::East,
          Direction::West  =>  direction = Direction::South,
        }
      Robot {coordinate: self.coordinate, facing: direction}
    }

    pub fn advance(self) -> Self {
      let mut coordinate = self.coordinate; 
        match self.facing { //The direction will tell me which axis to add/subtract from
          Direction::North =>  coordinate.1 += 1,
          Direction::East  =>  coordinate.0 += 1,  
          Direction::South =>  coordinate.1 -= 1,
          Direction::West  =>  coordinate.0 -= 1,
        }
      Robot {coordinate: coordinate, facing: self.facing}
    }

    pub fn instructions(self, instructions: &str) -> Self {
      let mut robot = Robot{coordinate: self.coordinate, facing: self.facing};
      for letter in instructions.chars() {
        match letter {
          'A' => robot = robot.advance(),
          'L' => robot = robot.turn_left(),
          'R' => robot = robot.turn_right(),
           _  => ()
        }
      }
      robot
    }

    pub fn position(&self) -> (isize, isize) {
        self.coordinate
    }

    pub fn direction(&self) -> &Direction {
        &self.facing
    }
}
