/* This represents the Tetris board, which in classic NES Tetris is 10x20 squares*/

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

pub struct Board {
    board: [[i32; BOARD_HEIGHT]; BOARD_WIDTH], //An array of arrays (Rust doesn't have matrices) of
                                                //size 10x20
}

impl Board {
    pub fn new_default() -> Board {
        Board {board: [[0; BOARD_HEIGHT]; BOARD_WIDTH]}
    }
}
