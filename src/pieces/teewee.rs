use crate::pieces::piece::{PieceType, Position};
use crate::game::{Board, TileState};

/* This is the I piece */
/* This piece position is fixed on the upper center on the Default rotation */
/*
Default rotation

* x *
  *

 */

pub struct Teewee {}

impl PieceType for Teewee {
    /*
    PUBLIC
     */

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_default_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        let r = Some([*position,
            Position{column:position.column+1,..*position},
            Position{column:position.column-1,..*position},
            Position{row:position.row-1,..*position},
        ]);
        if (board[position.row][position.column - 1] == TileState::Taken) ||
            (board[position.row][position.column + 1] == TileState::Taken) ||
            (board[position.row - 1][position.column] == TileState::Taken) {

            Some([*position,
                Position{column:position.column-1,..*position},
                Position{column:position.column+1,..*position},
                Position{row:position.row-1,..*position},
            ])
        } else {
            None
        }
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_inverted_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        for i in (position.column-1)..(position.column + 2) {
            if board[position.row - 1][i] == TileState::Taken {
                return Some([*position,
                    Position{column:position.column-1,..*position},
                    Position{column:position.column+1,..*position},
                    Position{row:position.row+1,..*position},
                ]);
            }
        }
        None
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_right_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        if (board[position.row - 1][position.column + 1] == TileState::Taken) ||
            (board[position.row - 2][position.column] == TileState::Taken) {

            Some([*position,
                Position{column:position.column+1,..*position},
                Position{row:position.row+1,..*position},
                Position{row:position.row-1,..*position},
            ])
        } else {
            None
        }
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_left_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]> {
        self.check_right_collision(board, &Position{row:position.row+3,..*position})
    }
}


