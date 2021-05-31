use crate::board::Coordinate;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move {
  pub from: Coordinate,
  pub to: Coordinate,
}

impl Move {
  pub fn new(from: (usize, usize), to: (usize, usize)) -> Move {
    Move {
      from: Coordinate(from.0, from.1),
      to: Coordinate(to.0, to.1),
    }
  }
}
