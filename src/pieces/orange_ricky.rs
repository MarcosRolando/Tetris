use crate::pieces::piece::{PieceType, Position};
use crate::game::{Board, SquareState};

/* This is the inversed L piece */

pub struct OrangeRicky {}

impl PieceType for OrangeRicky {
    /*
    PUBLIC
     */

    fn check_straight_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        //if board[position.row - 1][position.column] == SquareState::Taken {

        //}
        println!("hello orange ricky!");
        [Position{row:0, column:0}; 4]
    }

    fn check_inverted_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        [Position{row:0, column:0}; 4]
    }

    fn check_right_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        [Position{row:0, column:0}; 4]
    }

    fn check_left_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        [Position{row:0, column:0}; 4]
    }
}

