use crate::pieces::piece;
use piece::Piece;

/* This represents the Tetris board, which in classic NES Tetris is 10x20 squares*/

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

#[derive(Clone, Copy, PartialEq)] //Automatically generates the impl of Clone, Copy and PartialEq
                                    //for SquareState
enum SquareState {
    Free,
    Taken,
}

pub struct Game {
    board: [[SquareState; BOARD_WIDTH]; BOARD_HEIGHT], //An array of arrays (Rust doesn't have matrices) of
                                                        //size 10x20
}

impl Game {
    /*
    PUBLIC
     */

    pub fn new_default() -> Game {
        Game {board: [[SquareState::Free; BOARD_WIDTH]; BOARD_HEIGHT]}
    }

    /* Updates to next game state */
    pub fn update(delta_time: i32) {

    }

    /*
    PRIVATE
     */

    /* Checks if the piece has collided with the board and if so, sets the tiles as taken*/
    fn _check_collision(&self) {


    }
}
