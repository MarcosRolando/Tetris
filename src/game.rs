use crate::pieces::piece;
use piece::Piece;

/* This represents the Tetris board, which in classic NES Tetris is 10x20 squares*/

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

pub type Board = [[SquareState; BOARD_WIDTH]; BOARD_HEIGHT];

#[derive(Clone, Copy, PartialEq)] //Automatically generates the impl of Clone, Copy and PartialEq
                                    //for SquareState
pub enum SquareState {
    Free,
    Taken,
}

pub struct Game {
    board: Board, //An array of arrays (Rust doesn't have matrices) of
                                                        //size 10x20
    current_piece: Piece,
}

impl Game {
    /*
    PUBLIC
     */

    pub fn new_default() -> Game {
        Game {
            board: [[SquareState::Free; BOARD_WIDTH]; BOARD_HEIGHT],
            current_piece: Piece::new_default(),
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
