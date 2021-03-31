use crate::pieces::piece::{Piece, Rotation, PieceTiles, Movement, Position};
use crate::pieces::piece::PieceType;
use crate::pieces::piece_factory::PieceFactory;
use crate::board::{Board, BOARD_HEIGHT, BOARD_WIDTH};

const STARTING_POSITION: Position = Position {row: BOARD_HEIGHT / 2, column: BOARD_WIDTH / 2}; //todo ver bien el tema del spawn


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
        let mut game = Game {
            board: Board::new(),
            current_piece: PieceFactory::new(STARTING_POSITION),
        };
        game
    }

    /* Updates the game state */
    pub fn update(&mut self, delta_t: f32) {
        self.current_piece.try_to_descend(delta_t);
        if self._check_collision() {
            self.current_piece = PieceFactory::new(STARTING_POSITION);
        }
    }

    /* Rotates the piece based on the Rotation given */
    pub fn rotate_piece(&mut self, rotation: Rotation) {
        self.current_piece.rotate(rotation);
    }

    /* Moves the piece based on the Movement given */
    pub fn move_piece(&mut self, movement: Movement) {
        self.current_piece.move_to(movement);
    }

    pub fn print(&self) {
        self.board.print(&self.current_piece.get_positions());
    }

    /*
    PRIVATE
     */

    /* Checks if the piece has collided with the board and if so, sets the tiles as taken
        If there was a collision then it return true, otherwise returns false */
    fn _check_collision(&mut self) -> bool {
        let piece_positions = self.current_piece.get_positions();
        for position in &piece_positions {
            if self.board.is_tile_taken(position) {
                self.board.update_board(&piece_positions);
                return true;
            }
        }
        false
    }

}
