use crate::game::TileState;
use crate::pieces::piece::{Position, PieceTiles};

/* This represents the Tetris board, which in classic NES Tetris is 10x20 Tiles*/

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 25; //+1 row for the base and +4 rows for the ceiling, none of them
//are visible to the player
pub const BOARD_BASE: usize = 1; //first playable row
pub const BOARD_CEILING: usize = 20; //last playable row

pub struct Board {
    board: [[TileState; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    /*
    PUBLIC
     */

    pub fn new() -> Self {
        let mut b = Board {board: [[TileState::Free; BOARD_WIDTH]; BOARD_HEIGHT]}; //An array of arrays (Rust doesn't have matrices) of size 10x20
        b.board[0] = [TileState::Taken; BOARD_WIDTH];
        b
    }

    /* Returns true if the tile is taken, false otherwise */
    pub fn is_tile_taken(&self, position: &Position) -> bool {
        self.board[position.row][position.column] == TileState::Taken
    }

    pub fn print(&self, positions: &PieceTiles) {
        print!("\x1B[2J\x1B[1;1H"); //clears the screen
        let mut board = self.board;
        for position in positions {
            board[position.row][position.column] = TileState::Taken;
        }
        for row in board.iter().rev().skip(4) {
            for tile in row {
                match tile {
                    TileState::Free => print!("   "),
                    TileState::Taken => print!(" + "),
                }
            }
            print!("\n");
        }
    }

    /* Removes the completed lines and updates the player score */
    pub fn update_board(&mut self, positions: &PieceTiles) {
        for position in positions {
            if position.row + 1 <= BOARD_CEILING {
                self.board[position.row + 1][position.column] = TileState::Taken; //row + 1 because the piece doesn't actually overlap
            } else {
                panic!("You lost!"); //todo properly end the game
            }
        }
        self._check_for_lines_removal();
    }

    /*
    PRIVATE
     */

    //todo Refactor
    fn _check_for_lines_removal(&mut self) {
        let mut row_number = BOARD_BASE;
        let mut lines_to_remove = 0;
        for row in self.board.iter().skip(1) { //This way we skip the BASE
            let mut empty_line_tiles = 0; //If we find a fully empty line then we are done checking
            let mut found_a_taken_tile = false;
            lines_to_remove += 1; //We assume the current row/line is fully taken
            for tile in row {
                if *tile == TileState::Free {
                    empty_line_tiles += 1;
                } else {
                    found_a_taken_tile = true;
                }
                if found_a_taken_tile && (empty_line_tiles > 0) {
                    lines_to_remove -= 1; //If it turns out it's not then we cancel the operation
                    break;
                }
            }
            if empty_line_tiles == BOARD_WIDTH {
                return;
            }
            if ( (lines_to_remove > 0) && (empty_line_tiles > 0) ) || (lines_to_remove == 4) ||
                ( (lines_to_remove > 0) && (row_number == BOARD_CEILING) ) {
                return self._remove_lines(row_number - lines_to_remove, lines_to_remove);
            }
            row_number += 1;
        }
    }

    fn _remove_lines(&mut self, first_line: usize, lines_to_remove: usize) {
        let offset = first_line + lines_to_remove;
        for i in offset..(BOARD_HEIGHT - 1) {
            self.board[i - lines_to_remove] = self.board[i];
        }
    }

}