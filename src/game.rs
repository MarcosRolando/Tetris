use crate::pieces::piece::{Piece, Rotation, PieceTiles, Movement, Position};
use crate::pieces::piece::PieceType;
use crate::pieces::piece_factory::PieceFactory;

/* This represents the Tetris board, which in classic NES Tetris is 10x20 Tiles*/

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
const BOARD_BASE: usize = 1; // Base row of the board,
                            //this row is not visible and not part of the playable rows
                            //This acts as a solid foundation

const STARTING_POSITION: Position = Position {row: BOARD_HEIGHT / 2, column: BOARD_WIDTH / 2}; //todo ver bien el tema del spawn

pub type Board = [[TileState; BOARD_WIDTH]; BOARD_HEIGHT + BOARD_BASE];

#[derive(Clone, Copy, PartialEq)]
pub enum TileState {
    Free,
    Taken,
}

pub struct Game {
    board: Board, //An array of arrays (Rust doesn't have matrices) of size 10x20 (not counting the BASE)
    current_piece: Piece<dyn PieceType>,
}

impl Game {
    /*
    PUBLIC
     */

    pub fn new_default() -> Game {
        let mut game = Game {
            board: [[TileState::Free; BOARD_WIDTH]; BOARD_HEIGHT + BOARD_BASE],
            current_piece: PieceFactory::new(STARTING_POSITION),
        };
        game.board[0] = [TileState::Taken; BOARD_WIDTH];
        game
    }

    /* Updates the game state */
    pub fn update(&mut self, delta_t: f32) {
        self.current_piece.try_to_descend(delta_t);
        if self._check_collision() {
            self.current_piece = PieceFactory::new(STARTING_POSITION);
        }
    }

    /* Rotates the piece based on the Rotation given */
    pub fn rotate_piece(&mut self, rotation: Rotation) {
        self.current_piece.rotate(rotation);
    }

    /* Moves the piece based on the Movement given */
    pub fn move_piece(&mut self, movement: Movement) {
        self.current_piece.move_to(movement);
    }

    pub fn print(&self) {
        print!("\x1B[2J\x1B[1;1H"); //clears the screen
        let mut board = self.board;
        for position in &self.current_piece.get_positions() {
            board[position.row][position.column] = TileState::Taken;
        }
        for row in board.iter().rev() {
            for tile in row {
                match tile {
                    TileState::Free => print!(" - "),
                    TileState::Taken => print!(" + "),
                }
            }
            print!("\n");
        }
    }

    /*
    PRIVATE
     */

    /* Checks if the piece has collided with the board and if so, sets the tiles as taken
        If there was a collision then it return true, otherwise returns false */
    fn _check_collision(&mut self) -> bool {
        let piece_positions = self.current_piece.get_positions();
        for position in &piece_positions {
            if self.board[position.row][position.column] == TileState::Taken {
                self._update_board(&piece_positions);
                return true;
            }
        }
        false
    }

    /* Removes the completed lines and updates the player score */
    fn _update_board(&mut self, positions: &PieceTiles) {
        for position in positions {
            self.board[position.row + 2][position.column] = TileState::Taken; //row + 1 because the piece doesn't actually overlap
        }
        self._check_for_lines_removal();
    }

    //todo Refactor
    fn _check_for_lines_removal(&mut self) {
        let mut row_number = 0 + BOARD_BASE;
        let mut lines_to_remove = 0;
        for row in self.board.iter().skip(1) { //This way we skip the BASE
            let mut empty_line_tiles = 0; //If we find a fully empty line then we are done checking
            let mut found_a_taken_tile = false;
            lines_to_remove += 1; //We assume the current row/line is fully taken
            for tile in row {
                if found_a_taken_tile && (empty_line_tiles > 0) {
                    lines_to_remove -= 1; //If it turns out it's not then we cancel the operation
                    break;
                } else {
                    if *tile == TileState::Free {
                        empty_line_tiles += 1;
                    } else {
                        found_a_taken_tile = true;
                    }
                }
            }
            if ( (lines_to_remove > 0) && (empty_line_tiles > 0) ) || (lines_to_remove == 4) ||
                ( (lines_to_remove > 0) && (row_number == BOARD_HEIGHT - 1) ) {
                return self._remove_lines(row_number - lines_to_remove, lines_to_remove);
            }
            if empty_line_tiles == BOARD_WIDTH {
                return;
            }
            row_number += 1;
        }
    }

    fn _remove_lines(&mut self, first_line: usize, lines_to_remove: usize) {
        println!("hello!");
        let offset = first_line + lines_to_remove;
        for i in offset..BOARD_HEIGHT {
            self.board[i - lines_to_remove] = self.board[i]; //Aca pierdo algo de eficiencia porque copio vacias tambien, pero bueno
        }
    }
}
