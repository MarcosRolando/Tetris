use crate::pieces::piece::{PieceType, Position};
use crate::game::{Board, TileState};

/* This is the I piece */
/* This piece position is fixed on the left corner on the Default rotation */
/*
Default rotation

* * * *

 */

pub struct Hero {}

impl PieceType for Hero {
    /*
    PUBLIC
     */

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_default_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        for i in position.column..(position.column + 4) {
            if board[position.row - 1][i] == TileState::Taken {
                let r = Some([*position,
                             Position{column:position.column+1,..*position},
                             Position{column:position.column+2,..*position},
                             Position{column:position.column+3,..*position},
               ]);
                return r;
            }
        }
        None
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_inverted_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        for i in (position.column-3)..(position.column+1) {
            if board[position.row - 1][i] == TileState::Taken {
                let r = Some([*position,
                              Position{column:position.column-1,..*position},
                              Position{column:position.column-2,..*position},
                              Position{column:position.column-3,..*position},
                ]);
                return r; //Rust does not let me return by functional expression in an If if I don't
                //define an else clause that return None for example, but then I would be
                //exiting the function after just the first iteration. That is why I use r
            }
        }
        None
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_right_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        if board[position.row - 4][position.column] == TileState::Taken {
            Some([*position,
                Position{row:position.row-1,..*position},
                Position{row:position.row-2,..*position},
                Position{row:position.row-3,..*position},
            ])
        } else {
            None
        }
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_left_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        if board[position.row - 1][position.column] == TileState::Taken {
            Some([*position,
                Position{row:position.row+1,..*position},
                Position{row:position.row+2,..*position},
                Position{row:position.row+3,..*position},
            ])
        } else {
            None
        }
    }
}


