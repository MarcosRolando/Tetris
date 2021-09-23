use crate::model::game_logic::game_state::GameState;
use crate::model::board::{Board, BOARD_CEILING};
use crate::model::pieces::piece::{Piece, Movement, Position};
use crate::model::pieces::piece_factory::PieceFactory;
use crate::game_engine::{Input_t, GameState_t, PIECETILE_I};
use crate::model::game_logic::game::STARTING_POSITION;
use crate::model::game_logic::line_clearing_animation::LineClearingAnimation;

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
    fn update(&mut self, board: &mut Board, current_piece: &mut Piece, player_input: Input_t) -> Option<Box<dyn GameState>> {
        match current_piece.process_input(player_input) {
            Some(movement) => self._move_piece(board, current_piece, movement),
            None => current_piece.try_to_descend(),
        }
        if board.update_board(&current_piece.get_positions()) {
            return if board.check_for_lines_to_remove(current_piece.get_center_position().row as usize + 1) {
                Some(Box::new(LineClearingAnimation::new()))
            } else {
                *current_piece = PieceFactory::new_piece(STARTING_POSITION, false);
                if !board.positions_are_valid(&current_piece.get_positions()) {
                    panic!("You lost!"); //todo terminar decente el juego. Dato: el Classic NES Tetris termina cuando no puede spawnear la pieza, no si te pasas del tablero!
                }
                None
            }
        }
        None
    }

    fn get_piece_state(&self, current_piece: &Piece, game_state: &mut GameState_t) {
        for position in &current_piece.get_positions() {
            let p: Position<usize> = From::from(*position);
            if p.row <= BOARD_CEILING {
                game_state.board_config[p.row-1][p.column] = PIECETILE_I; //todo devolver cada tipo de pieza segun corresponda
            }
        }
    }
}