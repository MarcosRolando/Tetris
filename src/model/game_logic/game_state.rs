use crate::game_engine::{Input_t, GameState_t};
use crate::model::board::Board;
use crate::model::pieces::piece::Piece;

pub trait GameState {
    fn update(&mut self, board: &mut Board, current_piece: &mut Piece, player_input: Input_t) -> Option<Box<dyn GameState>>;
    fn get_piece_state(&self, current_piece: &Piece, game_state: &mut GameState_t);
}