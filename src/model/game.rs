use crate::model::pieces::piece::{Piece, Movement, Position};
use crate::model::pieces::piece::PieceType;
use crate::model::pieces::piece_factory::PieceFactory;
use crate::model::board::{Board, BOARD_HEIGHT, BOARD_CEILING};
use crate::game_engine::{GameState_t, PIECETILE_I, Input_t};

const STARTING_POSITION: Position = Position {row: BOARD_HEIGHT as isize - 3,
                                            column: 5}; //Classic NES Tetris uses this exact position for tetrominoes spawn


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
            current_piece: PieceFactory::new(STARTING_POSITION, true),
        };
        game
    }

    /* Updates the game state */
    pub fn update(&mut self, player_input: Input_t) {
        match self.current_piece.process_input(player_input) {
            Some(movement) => self.move_piece(movement),
            None => self.current_piece.try_to_descend(),
        }
        if self.board.update_board(&self.current_piece.get_positions()) {
            self.current_piece = PieceFactory::new(STARTING_POSITION, false);
            if !self.board.positions_are_valid(&self.current_piece.get_positions()) {
                panic!("You lost!"); //terminar decente el juego. Dato: el Classic NES Tetris termina cuando no puede spawnear la pieza, no si te pasas del tablero!
            }
        }
    }

    /* Moves the piece based on the Movement given */
    pub fn move_piece(&mut self, movement: Movement) {
        self.current_piece.move_to(movement);
        if  movement != Movement::Down && !self.board.positions_are_valid(&self.current_piece.get_positions()) {
            self.current_piece.move_to(Movement::get_opposite(movement));
        }
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

    pub fn print(&self) {
        self.board.print(&self.current_piece.get_positions());
    }
}
