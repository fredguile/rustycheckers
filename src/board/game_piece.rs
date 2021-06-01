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

const PIECEFLAG_BLACK: u8 = 1;
const PIECEFLAG_WHITE: u8 = 2;
const PIECEFLAG_CROWN: u8 = 4;

impl Into<i32> for GamePiece {
    fn into(self) -> i32 {
        let mut val: u8 = 0;

        if self.color == PieceColor::Black {
            val += PIECEFLAG_BLACK;
        } else if self.color == PieceColor::White {
            val += PIECEFLAG_WHITE;
        }

        if self.crowned {
            val += PIECEFLAG_CROWN;
        }

        val as i32
    }
}
