use crate::board::{Coordinate, GamePiece, Move, PieceColor};

pub struct GameEngine {
    board: [[Option<GamePiece>; 8]; 8],
    current_turn: PieceColor,
    move_count: u32,
}

pub struct MoveResult {
  pub mov: Move,
  pub crowned: bool,
}

impl GameEngine {
    pub fn new() -> GameEngine {
        let mut engine = GameEngine {
            board: [[None; 8]; 8],
            current_turn: PieceColor::Black,
            move_count: 0,
        };
        engine.initialize_pieces();
        engine
    }

    pub fn initialize_pieces(&mut self) {
        [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7]
            .iter()
            .zip([0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2].iter())
            .map(|(a, b)| (*a as usize, *b as usize))
            .for_each(|(x, y)| {
                self.board[x][y] = Some(GamePiece::new(PieceColor::White));
            });

        [0, 2, 4, 6, 1, 3, 5, 7, 0, 2, 4, 6]
            .iter()
            .zip([5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7].iter())
            .map(|(a, b)| (*a as usize, *b as usize))
            .for_each(|(x, y)| {
                self.board[x][y] = Some(GamePiece::new(PieceColor::Black));
            });
    }

    pub fn move_piece(&mut self, mov: &Move) -> Result<MoveResult, ()> {
      let legal_moves = self.legal_moves();

      if !legal_moves.contains(mov) {
        return Err(());
      }

      let Coordinate(fx, fy) = mov.from;
      let Coordinate(tx, ty) = mov.to;

      let piece = self.board[fx][fy].unwrap();
      let midpiece_coords = self.midpiece_coords(fx, fy, tx, ty);

      if let Some(Coordinate(x,y)) = midpiece_coords {
        self.board[x][y] = None; // removed jumped piece
      }

      // move piece to dest
      self.board[tx][ty] = Some(piece);
      self.board[fx][fy] = None;

      let crowned = if self.should_crown(piece, mov.to) {
        self.crown_piece(mov.to);
        true
      } else {
        false
      };

      self.advance_turn();

      Ok(MoveResult {
        mov: mov.clone(),
        crowned
      })
    }

    fn advance_turn(&mut self) {

    }

    fn legal_moves(&self) -> Vec<Move> {
      let mut moves: Vec<Move> = Vec::new();

      for col in 0..8 {
        for row in 0..8 {
          if let Some(piece) = self.board[col][row] {
            if piece.color == self.current_turn {
              let loc = Coordinate(col, row);
              let mut vmoves = self.valid_moves_from(loc);
              moves.append(&mut vmoves);
            }
          }
        }
      }

      moves
    }

    fn valid_moves_from(&self, loc: Coordinate) -> Vec<Move> {
      let Coordinate(x, y) = loc;

      if let Some(p) = self.board[x][y] {
        let mut jumps = loc
          .jumps_target_from()
          .filter(|t| self.valid_move(&p, &loc, &t))
          .map(|ref t| Move {
            from: loc.clone(),
            to: t.clone(),
          })
          .collect::<Vec<Move>>();
        
        let mut moves = loc
          .moves_target_from()
          .filter(|t| self.valid_jump(&p, &loc, &t))
          .map(|ref t| Move {
            from: loc.clone(),
            to: t.clone(),
          })
          .collect::<Vec<Move>>();

        jumps.append(&mut moves);
        jumps
      } else {
        Vec::new()
      }
    }

    fn valid_move(&self, piece: &GamePiece, loc: &Coordinate, target: &Coordinate) -> bool {
      false
    }

    fn valid_jump(&self, piece: &GamePiece, loc: &Coordinate, target: &Coordinate) -> bool {
      false
    }

    fn midpiece_coords(&self, fx: usize, fy: usize, tx: usize, ty: usize) -> Option<Coordinate> {
      None
    }

    fn should_crown(&self, piece: GamePiece, to: Coordinate) -> bool {
      false
    }

    fn crown_piece(&self, to: Coordinate) {

    }
}
