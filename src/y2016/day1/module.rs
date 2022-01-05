use core::fmt::Display;
use std::collections::BTreeSet;

#[allow(dead_code)]
pub enum Direction {
    North,
    West,
    East,
    South,
}

pub struct Position {
    _dir: Direction,
    _x: i32,
    _y: i32,
    _visited: BTreeSet<(i32, i32)>,
}

impl Position {
    #[allow(unused)]
    pub fn new(dir: Direction, x: i32, y: i32) -> Self {
        Position {
            _dir: dir,
            _x: x,
            _y: y,
            _visited: BTreeSet::new(),
        }
    }

    #[allow(unused)]
    pub fn change_direction(&mut self, input: char) {
        match self._dir {
            Direction::East => match input {
                'R' => {
                    self._dir = Direction::South;
                }
                'L' => {
                    self._dir = Direction::North;
                }
                _ => panic!("Invalid input!"),
            },
            Direction::North => match input {
                'R' => {
                    self._dir = Direction::East;
                }
                'L' => {
                    self._dir = Direction::West;
                }
                _ => panic!("Invalid input!"),
            },
            Direction::South => match input {
                'R' => {
                    self._dir = Direction::West;
                }
                'L' => {
                    self._dir = Direction::East;
                }
                _ => panic!("Invalid input!"),
            },
            Direction::West => match input {
                'R' => {
                    self._dir = Direction::North;
                }
                'L' => {
                    self._dir = Direction::South;
                }
                _ => panic!("Invalid input!"),
            },
        }
    }

    #[allow(unused)]
    pub fn advance(&mut self, command: &str) {
        self.change_direction(command.chars().nth(0).unwrap());
        let steps = command[1..].parse::<i32>().unwrap();
        let mut check_for_visit = |pos: &mut Position, x: i32, y: i32| {
            if (pos._visited.insert((x, y)) == false) {
                println!("First 2x visited x:{}, y:{}", x, y);
            };
        };
        match self._dir {
            Direction::East => {
                for x in self._x + 1..self._x + steps {
                    println!("Checking x: {}, y: {}", x, self._y);
                    check_for_visit(self, x, self._y);
                }
                self._x = self._x + steps;
            }
            Direction::West => {
                for x in self._x - steps..self._x {
                    println!("Checking x: {}, y: {}", x, self._y);
                    check_for_visit(self, x, self._y);
                }
                self._x = self._x - steps;
            }
            Direction::South => {
                for y in self._y - steps..self._y {
                    println!("Checking x: {}, y: {}", self._x, y);
                    check_for_visit(self, self._x, y);
                }
                self._y = self._y - steps;
            }
            Direction::North => {
                for y in self._y + 1..self._y + steps {
                    println!("Checking x: {}, y: {}", self._x, y);
                    check_for_visit(self, self._x, y);
                }
                self._y = self._y + steps;
            }
        }
    }

    #[allow(unused)]
    pub fn get_distance_from(&self, reference: Position) -> i32 {
        (self._x - reference._x).abs() + (self._y - reference._y).abs()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir_to_str = |dir: &Direction| -> &str {
            match dir {
                Direction::East => return "East",
                Direction::West => return "West",
                Direction::South => return "South",
                Direction::North => return "North",
            };
        };
        write!(
            f,
            "Direction: {}, x: {}, y: {}",
            dir_to_str(&self._dir),
            &self._x,
            &self._y
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let commands = "R2\nR3".lines();
        let mut position = Position::new(Direction::North, 0, 0);
        for command in commands {
            position.advance(command);
        }
        let distance = position.get_distance_from(Position::new(Direction::North, 0, 0));
        assert_eq!(distance, 5);
        assert_eq!(position._x, 2);
        assert_eq!(position._y, -3);
    }

    #[test]
    fn test_2() {
        let l1 = Line {
            x1: 7,
            x2: 10,
            y1: 5,
            y2: 3,
        };
        let l2 = Line {
            x1: 1,
            x2: 10,
            y1: 5,
            y2: 3,
        };
        check_cross(&l1, &l2);
    }
}
