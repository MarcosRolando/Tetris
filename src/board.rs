use crate::game::TileState;
use crate::pieces::piece::{Position, PieceTiles};
use crate::view_unit::{PieceTile_t, PieceTile_HERO, PieceTile_NONE};

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

    pub fn print(&self, positions: &PieceTiles) {
        print!("\x1B[2J\x1B[1;1H\r"); //clears the screen
        let mut board = self.board;
        for position in positions {
            let p: Position<usize> = From::from(*position);
            board[p.row][p.column] = TileState::Taken;
        }
        for row in board.iter().rev().skip(4) {
            for tile in row {
                match tile {
                    TileState::Free => print!("   "),
                    TileState::Taken => print!(" + "),
                }
            }
            print!("\n\r");
        }
    }

    /* Removes the completed lines, returns true if the the current piece collided, false otherwise */
    pub fn update_board(&mut self, positions: &PieceTiles) -> bool {
        return if self._check_collision(positions) {
            for position in positions {
                let p: Position<usize> = From::from(*position);
                if (p.row + 1) <= BOARD_CEILING {
                    self.board[p.row + 1][p.column] = TileState::Taken; //row + 1 because the piece doesn't actually overlap
                } else {
                    panic!("You lost!"); //todo properly end the game
                }
            }
            self._check_for_lines_removal();
            true
        } else {
            false
        }
    }

    pub fn positions_are_valid(&self, positions: &PieceTiles) -> bool {
        for p in positions {
            if (p.row < BOARD_BASE as isize) || (p.row > (BOARD_HEIGHT - 1) as isize) || (p.column < 0) ||
                (p.column > (BOARD_WIDTH as isize - 1)) {
                return false;
            } else if self.board[p.row as usize][p.column as usize] == TileState::Taken {
                return false;
            }
        }
        true
    }

    /*
    PRIVATE
     */

    fn _check_for_lines_removal(&mut self) {
        let mut row_number = BOARD_BASE;
        let mut lines_to_remove = 0;
        for row in self.board.iter().skip(1) { //This way we skip the BASE
            let mut empty_line_tiles = 0; //If we find a fully empty line then we are done checking
            self._process_line(row, &mut lines_to_remove, &mut empty_line_tiles);
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

    fn _process_line(&self, row: &[TileState; 10], lines_to_remove: &mut usize,
                     empty_line_tiles: &mut usize) {
        let mut found_a_taken_tile = false;
        *lines_to_remove += 1; //We assume the current row/line is fully taken
        for tile in row {
            if *tile == TileState::Free {
                *empty_line_tiles += 1;
            } else {
                found_a_taken_tile = true;
            }
            if found_a_taken_tile && (*empty_line_tiles > 0) {
                *lines_to_remove -= 1; //If it turns out it's not then we cancel the operation
                break;
            }
        }
    }

    fn _remove_lines(&mut self, first_line: usize, lines_to_remove: usize) {
        let offset = first_line + lines_to_remove;
        for i in offset..(BOARD_HEIGHT - 1) {
            self.board[i - lines_to_remove] = self.board[i];
        }
    }

    /* Checks if the piece has collided with the board and if so, sets the tiles as taken
        If there was a collision then it return true, otherwise returns false */
    fn _check_collision(&self, positions: &PieceTiles) -> bool {
        for position in positions {
            if self._is_tile_taken(position) {
                return true;
            }
        }
        false
    }

    /* Returns true if the tile is taken, false otherwise */
    fn _is_tile_taken(&self, position: &Position) -> bool {
        let p: Position<usize> = From::from(*position);
        self.board[p.row][p.column] == TileState::Taken
    }
}

impl From<&Board> for [[PieceTile_t; 10usize]; 20usize] {
    fn from(game_board: &Board) -> Self {
        let mut state_board: Self = [[PieceTile_NONE; 10usize]; 20usize]; //todo arreglar que esta funcion me esta matando el juego
        for i in BOARD_BASE..BOARD_CEILING {
            for j in 0..(BOARD_WIDTH - 1) {
                match game_board.board[i][j] {
                    TileState::Taken => {state_board[i-1][j] = PieceTile_HERO;},
                    TileState::Free => (), //todo cambiar el tilestate para que en realidad sea que tipo de pieza guarda o en su defecto que no guarda ninguan
                }
            }
        }
        state_board
    }
}
