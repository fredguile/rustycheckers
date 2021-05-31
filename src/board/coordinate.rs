#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {
  pub fn on_board(self) -> bool {
    let Coordinate(x, y) = self;
    x <= 7 && y <= 7
  }

  pub fn jumps_target_from(&self) -> impl Iterator<Item = Coordinate> {
    let mut jumps = Vec::new();
    let Coordinate(x, y) = *self;

    if y > 2 {
      jumps.push(Coordinate(x + 2, y - 2));
    }
    jumps.push(Coordinate(x + 2, y + 2));

    if x >= 2 && y >= 2 {
      jumps.push(Coordinate(x - 2, y - 2))
    }

    if x >= 2 {
      jumps.push(Coordinate(x - 2, y + 2))
    }

    jumps.into_iter()
  }

  pub fn moves_target_from(&self) -> impl Iterator<Item = Coordinate> {
    let mut moves = Vec::new();
    let Coordinate(x, y) = *self;

    if x >= 1 {
      moves.push(Coordinate(x - 1, y + 1));
    }
    moves.push(Coordinate(x + 1, y + 1));

    if y >= 1 {
      moves.push(Coordinate(x + 1, y - 1));
    }

    if x >= 1 && y >= 1 {
      moves.push(Coordinate(x - 1, y - 1));
    }
    
    moves.into_iter()
  }
}
