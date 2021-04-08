use crate::model::pieces::piece::{Piece, Movement, Position};
use crate::model::pieces::piece::PieceType;
use crate::model::pieces::piece_factory::PieceFactory;
use crate::model::board::{Board, BOARD_HEIGHT, BOARD_CEILING};
use crate::game_engine::{GameState_t, PIECETILE_I, Input_t};
use crate::model::game::game_state::GameState;
use crate::model::game::playing::Playing;

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
        let mut game = Game {
            board: Board::new(),
            current_piece: PieceFactory::new(STARTING_POSITION, true),
            state: Box::new(Playing {}), //board and current_piece are not yet created so we can't intialize the Playing state here
        };
        game
    }

    /* Updates the game state */
    pub fn update(&mut self, player_input: Input_t) {
        self.state.update(&mut self.board, &mut self.current_piece, player_input);
    }

    /* Returns a GameState_t which will be used by the SDL view-controller module to render the current frame */
    pub fn get_state(&self) -> GameState_t {
        let mut game_state = GameState_t {board_config: From::from(&self.board)};
        for position in &self.current_piece.get_positions() {
            let p: Position<usize> = From::from(*position);
            if p.row <= BOARD_CEILING {
                game_state.board_config[p.row-1][p.column] = PIECETILE_I; //todo devolver cada tipo de pieza segun corresponda
            }
        }
        game_state
    }
}
