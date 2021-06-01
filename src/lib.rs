extern crate lazy_static;

mod board;
mod game;

use board::{Coordinate, GamePiece, Move};
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
        Ok(p) => p.into(),
        Err(_) => -1,
    }
}

pub extern "C" fn get_current_turn() -> i32 {
    let engine = GAME_ENGINE.read().unwrap();

    GamePiece::new(engine.current_turn()).into()
}

pub extern "C" fn move_piece(fx: i32, fy: i32, tx: i32, ty: i32) -> i32 {
    let mut engine = GAME_ENGINE.write().unwrap();

    let mov = Move::new((fx as usize, fy as usize), (tx as usize, ty as usize));
    let res = engine.move_piece(&mov);

    match res {
        Ok(result) => {
            unsafe {
                notify_piecemoved(fx, fy, tx, ty);
            }

            if result.crowned {
                unsafe {
                    notify_piececrowned(tx, ty);
                }
            }
            1
        }
        Err(_) => 0,
    }
}

extern "C" {
    fn notify_piecemoved(fromX: i32, fromY: i32, toX: i32, toY: i32);
    fn notify_piececrowned(x: i32, y: i32);
}
