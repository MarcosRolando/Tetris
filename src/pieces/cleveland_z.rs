use crate::pieces::piece::{PieceType, Position};
use crate::game::{Board, TileState};

/* This is the Z piece */
/* This piece position is fixed on the lower center corner on the Default rotation */
/*
Default rotation

* *
  x *

 */

pub struct ClevelandZ {}

impl PieceType for ClevelandZ {
    /*
    PUBLIC
     */

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_default_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        let r = Some([*position,
            Position{column:position.column+1,..*position},
            Position{row:position.row+1,..*position},
            Position{row:position.row+1,column:position.column-1},
        ]);
        if board[position.row][position.column - 1] == TileState::Taken {
            return r;
        }
        for i in position.column..(position.column+2) {
            if board[position.row - 1][i] == TileState::Taken {
                return r;
            }
        }
        None
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    //noinspection DuplicatedCode CLion complains about one fucking duplicated line between different files
    fn check_inverted_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        self.check_default_collision(board, &Position{row:position.row-1,..*position})
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_right_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        if (board[position.row - 1][position.column + 1] == TileState::Taken) ||
            (board[position.row - 2][position.column] == TileState::Taken) {

            Some([*position,
                Position{column:position.column+1,..*position},
                Position{row:position.row-1,..*position},
                Position{row:position.row+1,column:position.column+1},
            ])
        } else {
            None
        }
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_left_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        if (board[position.row - 1][position.column - 1] == TileState::Taken) ||
            (board[position.row - 2][position.column] == TileState::Taken) {

            Some([*position,
                Position{column:position.column-1,..*position},
                Position{row:position.row+1,..*position},
                Position{row:position.row-1,..*position},
            ])
        } else {
            None
        }
    }
}
