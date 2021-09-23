use crate::model::game_logic::game_state::GameState;
use crate::model::board::Board;
use crate::model::pieces::piece::Piece;
use crate::model::pieces::piece_factory::PieceFactory;
use crate::model::game_logic::game::STARTING_POSITION;
use crate::model::game_logic::playing::Playing;
use crate::game_engine::GameState_t;

pub struct LineClearingAnimation {
    frame_delay: usize, //The amount of frames that have to pass between each update
    curr_frame: usize, //The current frame, it resets to 0 once it reaches 3
    column_counter: usize //We will use this value to know which column tiles to delete
}

impl LineClearingAnimation {
    /*
    PUBLIC
     */

    pub fn new() -> Self {
        LineClearingAnimation {frame_delay: 0, curr_frame: 0, column_counter: 0}
    }
}

impl GameState for LineClearingAnimation {
    fn update(&mut self, board: &mut Board, current_piece: &mut Piece, _: u32) -> Option<Box<dyn GameState>> {
        if self.curr_frame % 4 == 0 {
            board.clear_some_tiles(self.column_counter);
            self.column_counter += 1;
        }
        self.curr_frame += 1;
        if self.column_counter == 5 {
            board.remove_completed_lines();
            *current_piece = PieceFactory::new_piece(STARTING_POSITION, false);
            Some(Box::new(Playing {}))
        } else {
            None
        }
    }

    fn get_piece_state(&self, current_piece: &Piece, game_state: &mut GameState_t) {
        //do nothing
    }
}
