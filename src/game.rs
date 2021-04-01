use crate::pieces::piece::{Piece, Rotation, Movement, Position, PieceTiles};
use crate::pieces::piece::PieceType;
use crate::pieces::piece_factory::PieceFactory;
use crate::board::{Board, BOARD_HEIGHT, BOARD_WIDTH, BOARD_BASE, BOARD_CEILING};

const STARTING_POSITION: Position = Position {row: BOARD_HEIGHT - 3, column: BOARD_WIDTH / 2}; //todo ver bien el tema del spawn


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

        /*
        self.current_piece.move_to(movement);
        if !self.board.positions_are_valid(&self.current_piece.get_positions()) {
            self.current_piece.move_to(Movement::get_opposite(movement));
        }
        */
    }

    /* Rotates the piece based on the Rotation given */
    pub fn rotate_piece(&mut self, rotation: Rotation) {
        self.current_piece.rotate(rotation);
        if !self.board.positions_are_valid(&self.current_piece.get_positions()) {
            self.current_piece.rotate(Rotation::get_opposite(rotation));
        }
    }

    pub fn print(&self) {
        self.board.print(&self.current_piece.get_positions());
    }
}
