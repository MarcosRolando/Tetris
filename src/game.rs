use crate::pieces::piece::{Piece, Rotation, TakenTiles};
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

    /* Updates the game state */
    pub fn update(&mut self, delta_time: i32) {
        self._check_collision();
    }

    /* Rotates the piece based on the Rotation given */
    pub fn rotate_piece(&mut self, rotation: Rotation) {
        self.current_piece.rotate(rotation);
    }

    /*
    PRIVATE
     */

    /* Checks if the piece has collided with the board and if so, sets the tiles as taken*/
    fn _check_collision(&mut self) {
        match self.current_piece.check_collision(&self.board) {
            Some(positions) => self._update_board(&positions),
            None => (), //Do nothing because it hasn't collided
        }
    }

    /* Removes the completed lines and updates the player score */
    fn _update_board(&mut self, positions: &TakenTiles) {
        for position in positions {
            self.board[position.row][position.column] = TileState::Taken;
        }
        self._check_for_lines_removal();
    }

    fn _check_for_lines_removal(&self) {
        let mut row_number = 0;
        let mut lines_to_remove = 0;
        let mut found_lines_to_remove = false;
        for row in &self.board {
            let mut empty_line_tiles = 0; //If we find a fully empty line then we are done checking
            lines_to_remove += 1; //We assume the current row/line is fully taken
            for tile in row {
                if tile != TileState::Taken {
                    lines_to_remove -= 1; //If it turns out it's not then we cancel the operation
                    empty_line_tiles += 1;
                    break;
                }
                if found_lines_to_remove {
                    self._remove_lines(row_number - lines_to_remove, lines_to_remove);
                    return;
                }
            }
            if lines_to_remove == 1 { //We found at least 1 fully taken row
                found_lines_to_remove = true;
            } else if empty_line_tiles == BOARD_WIDTH {
                return;
            }
            row_number += 1;
        }
    }

    fn _remove_lines(&self, first_line: usize, lines_to_remove: usize) {

    }
}
