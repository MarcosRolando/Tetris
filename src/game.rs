use crate::pieces::piece::{Piece, Rotation, TakenTiles, Movement};
use crate::pieces::piece::{Position, PieceType};
use crate::pieces::{
    hero::Hero,
    orange_ricky::OrangeRicky,
    blue_ricky::BlueRicky,
    cleveland_z::ClevelandZ,
    rhode_island_z::RhodeIslandZ,
    smashboy::Smashboy,
    teewee::Teewee,
};
use std::collections::HashMap;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

/* This represents the Tetris board, which in classic NES Tetris is 10x20 Tiles*/

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;
const BOARD_BASE: usize = 1; // Base row of the board,
                            //this row is not visible and not part of the playable rows
                            //This acts as a solid foundation


pub type Board = [[TileState; BOARD_WIDTH]; BOARD_HEIGHT + BOARD_BASE];

#[derive(Clone, Copy, Hash)]
enum PieceTypeID {
    Hero,
    Smashboy,
    Teewee,
    OrangeRicky,
    BlueRicky,
    ClevelandZ,
    RhodeIslandZ,
}

/* Returns a random PieceTypeID value, for selecting the next piece */
impl Distribution<PieceTypeID> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceTypeID {
        match rng.gen_range(0..=6) {
            0 => PieceTypeID::Hero,
            1 => PieceTypeID::Smashboy,
            2 => PieceTypeID::Teewee,
            3 => PieceTypeID::OrangeRicky,
            4 => PieceTypeID::BlueRicky,
            5 => PieceTypeID::ClevelandZ,
            _ => PieceTypeID::RhodeIslandZ,
        }
    }
}


const PIECE: HashMap<PieceTypeID, fn () -> Box<dyn PieceType>> = [
    (PieceTypeID::Hero, Hero::new()),
    (PieceTypeID::Smashboy, Smashboy::new()),
    (PieceTypeID::Teewee, Teewee::new()),
    (PieceTypeID::OrangeRicky, OrangeRicky::new()),
    (PieceTypeID::BlueRicky, BlueRicky::new()),
    (PieceTypeID::ClevelandZ, ClevelandZ::new()),
    (PieceTypeID::RhodeIslandZ, RhodeIslandZ::new())].iter().cloned().collect();


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
            current_piece: Piece::new(
                Position{row:0,column:0},
                Box::new(Hero{})
            ),
        };
        game.board[0] = [TileState::Taken; BOARD_WIDTH];
        game
    }

    /* Updates the game state */
    pub fn update(&mut self, delta_t: f32) {
        self.current_piece.try_to_descend(delta_t);
        self._check_collision();
    }

    /* Rotates the piece based on the Rotation given */
    pub fn rotate_piece(&mut self, rotation: Rotation) {
        self.current_piece.rotate(rotation);
    }

    /* Moves the piece based on the Movement given */
    pub fn move_piece(&mut self, movement: Movement) {
        self.current_piece.move_to(movement);
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

    //todo Refactor
    fn _check_for_lines_removal(&mut self) {
        let mut row_number = 0;
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
        let offset = first_line + lines_to_remove;
        for i in offset..(BOARD_HEIGHT - 1) {
            self.board[i - lines_to_remove] = self.board[i]; //Aca pierdo algo de eficiencia porque copio vacias tambien, pero bueno
        }
    }
}
