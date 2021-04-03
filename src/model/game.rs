use crate::model::pieces::piece::{Piece, Rotation, Movement, Position};
use crate::model::pieces::piece::PieceType;
use crate::model::pieces::piece_factory::PieceFactory;
use crate::model::board::{Board, BOARD_HEIGHT, BOARD_WIDTH, BOARD_CEILING};
use crate::game_engine::{GameState_t, PIECETILE_HERO};

const STARTING_POSITION: Position = Position {row: BOARD_HEIGHT as isize - 3,
                                            column: BOARD_WIDTH as isize / 2}; //todo ver bien el tema del spawn


#[derive(Clone, Copy, PartialEq)]
pub enum TileState {
    Free,
    Taken,
}

pub struct Game {
    board: Board,
    current_piece: Piece<dyn PieceType>,
}

impl Game {
    /*
    PUBLIC
     */

    pub fn new_default() -> Game {
        let game = Game {
            board: Board::new(),
            current_piece: PieceFactory::new(STARTING_POSITION),
        };
        game
    }

    /* Updates the game state */
    pub fn update(&mut self, delta_t: f32) {
        self.current_piece.try_to_descend(delta_t);
        if self.board.update_board(&self.current_piece.get_positions()) {
            self.current_piece = PieceFactory::new(STARTING_POSITION);
        }
    }

    /* Moves the piece based on the Movement given */
    pub fn move_piece(&mut self, movement: Movement) {
        self.current_piece.move_to(movement);
        if !self.board.positions_are_valid(&self.current_piece.get_positions()) {
            self.current_piece.move_to(Movement::get_opposite(movement));
        }
    }

    /* Rotates the piece based on the Rotation given */
    pub fn rotate_piece(&mut self, rotation: Rotation) {
        self.current_piece.rotate(rotation);
        if !self.board.positions_are_valid(&self.current_piece.get_positions()) {
            self.current_piece.rotate(Rotation::get_opposite(rotation));
        }
    }

    /* Returns a GameState_t which will be used by the SDL view-controller module to render the current frame */
    pub fn get_state(&self) -> GameState_t {
        let mut game_state = GameState_t {board_config: From::from(&self.board)};
        for position in &self.current_piece.get_positions() { // The current piece position todo emprolijar este codigo
            let p: Position<usize> = From::from(*position);
            if p.row <= BOARD_CEILING {
                game_state.board_config[p.row-1][p.column] = PIECETILE_HERO;
            }
        }
        game_state
    }

    pub fn print(&self) {
        self.board.print(&self.current_piece.get_positions());
    }
}
