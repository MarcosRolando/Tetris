use crate::model::game::game_state::GameState;
use crate::model::board::Board;
use crate::model::pieces::piece::{Piece, PieceType, Movement};
use crate::model::pieces::piece_factory::PieceFactory;
use crate::game_engine::Input_t;
use crate::model::game::game::STARTING_POSITION;

pub struct Playing {}

impl Playing {
    /*
    PRIVATE
     */

    /* Moves the piece based on the Movement given */
    pub fn _move_piece(&self, board: &mut Board, current_piece: &mut Piece, movement: Movement) {
        current_piece.move_to(movement);
        if  movement != Movement::Down && !board.positions_are_valid(&current_piece.get_positions()) {
            current_piece.move_to(Movement::get_opposite(movement));
        }
    }
}

impl GameState for Playing {
    fn update(&self, board: &mut Board, current_piece: &mut Piece, player_input: Input_t) {
        match current_piece.process_input(player_input) {
            Some(movement) => self._move_piece(board, current_piece, movement),
            None => current_piece.try_to_descend(),
        }
        if board.update_board(&current_piece.get_positions(),
                                   &current_piece.get_center_position()) {
            *current_piece = PieceFactory::new(STARTING_POSITION, false);
            if !board.positions_are_valid(&current_piece.get_positions()) {
                panic!("You lost!"); //terminar decente el juego. Dato: el Classic NES Tetris termina cuando no puede spawnear la pieza, no si te pasas del tablero!
            }
        }
    }
}