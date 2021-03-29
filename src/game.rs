use crate::pieces::piece;
use piece::Piece;
use crate::pieces::piece::{Position, PieceType};
use crate::pieces::hero::Hero;

/* This represents the Tetris board, which in classic NES Tetris is 10x20 Tiles*/

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

pub type Board = [[TileState; BOARD_WIDTH]; BOARD_HEIGHT];

#[derive(Clone, Copy, PartialEq)] //Automatically generates the impl of Clone, Copy and PartialEq
                                    //for TileState
pub enum TileState {
    Free,
    Taken,
}

pub struct Game {
    board: Board, //An array of arrays (Rust doesn't have matrices) of
                                                        //size 10x20
    current_piece: Piece<dyn PieceType>,
}

impl Game {
    /*
    PUBLIC
     */

    pub fn new_default() -> Game {
        Game {
            board: [[TileState::Free; BOARD_WIDTH]; BOARD_HEIGHT],
            current_piece: Piece::new(
                Position{row:0,column:0},
                Box::new(Hero{})
            ),
        }
    }

    /* Updates to next game state */
    pub fn update(&self, delta_time: i32) {
        self._check_collision();
    }

    /*
    PRIVATE
     */

    /* Checks if the piece has collided with the board and if so, sets the tiles as taken*/
    fn _check_collision(&self) {
        self.current_piece.check_collision(&self.board);
    }
}
