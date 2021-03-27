/* This represents the Tetris board, which in classic NES Tetris is 10x20 squares*/

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

#[derive(Clone, Copy)] //Esto como que genera automaticamente el impl de Clone y Copy para el enum
enum SquareState {
    Free,
    Taken,
}

pub struct Board {
    board: [[SquareState; BOARD_WIDTH]; BOARD_HEIGHT], //An array of arrays (Rust doesn't have matrices) of
                                                //size 10x20
}

impl Board {
    pub fn new_default() -> Board {
        Board {board: [[SquareState::Free; BOARD_WIDTH]; BOARD_HEIGHT]}
    }
}
