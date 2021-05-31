#[macro_use]
extern crate lazy_static;

mod board;
mod game;

use board::{Coordinate, GamePiece, Move, PieceColor};
use game::GameEngine;
use lazy_static::lazy_static;
use mut_static::MutStatic;

lazy_static! {
  pub static ref GAME_ENGINE: MutStatic<GameEngine> = MutStatic::from(GameEngine::new());
}

pub extern "C" fn get_piece(x: i32, y: i32) -> i32 {
  let engine = GAME_ENGINE.read().unwrap();

  let piece = engine.get_piece(Coordinate(x as usize, y as usize));
  match piece {
    Ok(Some(p)) => p.into(),
    Ok(None) => -1,
    Err(_) => -1,
  }
}

pub extern "C" fn get_current_turn() -> i32 {
  let engine = GAME_ENGINE.read().unwrap();

  GamePiece::new(engine.current_turn()).into();
}