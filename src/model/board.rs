use crate::model::game_logic::game::TileState;
use crate::model::pieces::piece::{Position, PieceTiles};
use crate::game_engine::{PieceTile_t, PIECETILE_I, PIECETILE_NONE};

/* This represents the Tetris board, which in classic NES Tetris is 10x20 Tiles*/

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 23; //+1 row for the base and +2 rows for the ceiling, none of them
                                    //are visible to the player. Classic NES tetris only uses the 2 extra hidden
                                    // always Free top rows for initial rotation possibilities,
                                    //while I also use an extra hidden always Taken bottom row for avoiding
                                    //having to check bounds, leaving a nicer code

pub const BOARD_BASE: usize = 1; //first playable row
pub const BOARD_CEILING: usize = 20; //last playable row

pub struct Board {
    board: Vec<Vec<TileState>>,
    lines_to_remove: Vec<usize>,
}

impl Default for Board {
    fn default() -> Self {
        let mut b = Board {
            board: vec![vec![TileState::Free; BOARD_WIDTH]; BOARD_HEIGHT], //A vector of vectors (Rust doesn't have matrices) of size 10x20
            lines_to_remove: Vec::new(),
        };
        b.board[0] = vec![TileState::Taken; BOARD_WIDTH];
        b
    }
}

impl Board {
    /*
    PUBLIC
     */

    pub fn new() -> Self {
        Board::default()
    }

    /* Returns true if the the current piece collided, false otherwise */
    pub fn update_board(&mut self, positions: &PieceTiles) -> bool {
        if self._check_collision(positions) {
            for position in positions {
                let p: Position<usize> = From::from(*position);
                if (p.row + 1) <= BOARD_CEILING { //The classic NES Tetris doesn't store the tiles of a piece that lock on the upper 2 hidden rows, it esentially truncates them (see https://tetris.wiki/Tetris_(NES,_Nintendo))
                    self.board[p.row + 1][p.column] = TileState::Taken; //row + 1 because the piece doesn't actually overlap
                }
            }
            true
        } else {
            false
        }
    }

    /* Deletes 2 tiles from the current completed lines. If it is called when no lines have been
        completed then it doesn't do anything
     */
    pub fn clear_some_tiles(&mut self, j: usize) {
        for i in &self.lines_to_remove {
            self.board[*i][4 - j] = TileState::Free;
            self.board[*i][5 + j] = TileState::Free; //This way we clear tiles from the center to the sides
        }
    }

    pub fn positions_are_valid(&self, positions: &PieceTiles) -> bool {
        for p in positions {
            if (p.row < BOARD_BASE as isize) 
                || (p.row > (BOARD_HEIGHT - 1) as isize) 
                || (p.column < 0) || (p.column > (BOARD_WIDTH as isize - 1)) 
                || (self.board[p.row as usize][p.column as usize] == TileState::Taken) {
                return false; //TODO refactorizar esto
            }
        }
        true
    }

    /* Removes the current completed lines. This function will be called after the line clearing
       animation is completed.
    */
    pub fn remove_completed_lines(&mut self) {
        //self.check_for_lines_to_remove(piece_row);
        for (i, row) in self.lines_to_remove.iter().enumerate() {
            self.board.remove(row - i);
            self.board.insert(BOARD_HEIGHT - 1, vec![TileState::Free; BOARD_WIDTH]);
        }
        self.lines_to_remove.clear();
    }

    /* Checks if the are lines to remove and if so adds them to the list. This function should
        be called after every piece collision. If there are lines to remove then it returns true,
        otherwise it returns false
     */
    pub fn check_for_lines_to_remove(&mut self, piece_row: usize) -> bool {
        let initial_row = std::cmp::max(piece_row - 1, 1);
        let final_row = std::cmp::min(piece_row + 2, BOARD_CEILING);
        let mut curr_row = initial_row;
        for row in self.board[initial_row..=final_row].iter() { //Because of the center of the pieces, these rows are the only ones we need to check
            let mut line_completed = true; //We assume the current line is completed
            for tile in row {
                if *tile == TileState::Free {
                    line_completed = false; //It turns out it wasn't completed
                    break;
                }
            }
            if line_completed {
                self.lines_to_remove.push(curr_row);
            }
            curr_row += 1;
        }
        !self.lines_to_remove.is_empty()
    }

    /*
    PRIVATE
     */

    /* Checks if the piece has collided with the board
        If there was a collision then it returns true, otherwise returns false */
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
        let mut state_board: Self = [[PIECETILE_NONE; 10usize]; 20usize];
        for i in BOARD_BASE..(BOARD_CEILING + 1) {
            for j in 0..BOARD_WIDTH {
                match game_board.board[i][j] {
                    TileState::Taken => {state_board[i-1][j] = PIECETILE_I;},
                    TileState::Free => (), //todo cambiar el tilestate para que en realidad sea que tipo de pieza guarda o en su defecto que no guarda ninguan
                }
            }
        }
        state_board
    }
}
