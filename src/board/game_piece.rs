#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
  White,
  Black,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GamePiece {
  pub color: PieceColor,
  pub crowned: bool,
}

impl GamePiece {
  pub fn new(color: PieceColor) -> Self {
    Self {
      color,
      crowned: false,
    }
  }

  pub fn crowned(p: Self) -> Self {
    Self {
      color: p.color,
      crowned: true,
    }
  }
}
