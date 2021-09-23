use crate::model::pieces::piece::{Piece, Position};
use crate::model::pieces::piece_factory::PieceFactory;
use crate::model::board::{Board, BOARD_HEIGHT};
use crate::game_engine::{GameState_t, Input_t};
use crate::model::game_logic::game_state::GameState;
use crate::model::game_logic::playing::Playing;

pub (super) const STARTING_POSITION: Position = Position {row: BOARD_HEIGHT as isize - 3,
                                            column: 5}; //Classic NES Tetris uses this exact position for tetrominoes spawn


#[derive(Clone, Copy, PartialEq)]
pub enum TileState {
    Free,
    Taken,
}

pub struct Game {
    board: Board,
    current_piece: Piece,
    state: Box<dyn GameState>,
}

impl Game {
    /*
    PUBLIC
     */

    pub fn new_default() -> Self {
        Game {
            board: Board::new(),
            current_piece: PieceFactory::new_piece(STARTING_POSITION, true),
            state: Box::new(Playing {}), //board and current_piece are not yet created so we can't intialize the Playing state here
        }
    }

    /* Updates the game state */
    pub fn update(&mut self, player_input: Input_t) {
        if let Some(next_state) = self.state.update(&mut self.board, &mut self.current_piece, player_input) {
           self.state = next_state;
        }
    }

    /* Returns a GameState_t which will be used by the SDL view-controller module to render the current frame */
    pub fn get_state(&self) -> GameState_t {
        let mut game_state = GameState_t {board_config: From::from(&self.board)};
        self.state.get_piece_state(&self.current_piece, &mut game_state);
        game_state
    }
}
