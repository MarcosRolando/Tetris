use crate::pieces::piece::{PieceType, Position};
use crate::game::{Board, TileState};

/* This is the square piece */
/* This piece position is fixed on the lower left corner on the Default rotation */
/* This is the only piece that does not actually rotate */
/*
Default rotation

* *
X *

 */

pub struct Smashboy {}

impl PieceType for Smashboy {
    /*
    PUBLIC
     */

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_default_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        if (board[position.row - 1][position.column] == TileState::Taken) ||
            (board[position.row - 1][position.column + 1] == TileState::Taken) {
            Some([*position,
                Position{row:position.row+1,..*position},
                Position{column:position.column+1,..*position},
                Position{row:position.row+1,column:position.column+1},
            ])
        } else {
            None
        }
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_inverted_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        self.check_default_collision(board, position)
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_right_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        self.check_default_collision(board, position)
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_left_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        self.check_default_collision(board, position)
    }
}


