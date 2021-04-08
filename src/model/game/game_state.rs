use crate::game_engine::Input_t;
use crate::model::board::Board;
use crate::model::pieces::piece::Piece;

pub trait GameState {
    fn update(&self, board: &mut Board, current_piece: &mut Piece, player_input: Input_t);
}