use crate::pieces::piece::{PieceType, Position};
use crate::game::{Board, SquareState};

/* This is the I piece */
/* This piece position is fixed on the lower square in the Straight orientation */

pub struct Hero {

}

impl Hero {}

impl PieceType for Hero {
    fn check_straight_collision(&self, board: &Board, position: &Position) -> [Position; 4] {
        //if board[position.row - 1][position.column] == SquareState::Taken {

        //}
        println!("hello hero!");
        [Position{row:0, column:0}; 4]
    }

    fn check_inverted_collision(&self, board: &Board, position: &Position) -> [Position; 4] {
        [Position{row:0, column:0}; 4]
    }

    fn check_right_collision(&self, board: &Board, position: &Position) -> [Position; 4] {
        [Position{row:0, column:0}; 4]
    }

    fn check_left_collision(&self, board: &Board, position: &Position) -> [Position; 4] {
        [Position{row:0, column:0}; 4]
    }
}


