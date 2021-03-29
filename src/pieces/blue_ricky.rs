use crate::pieces::piece::{PieceType, Position, TakenTiles};
use crate::game::{Board, TileState};

/* This is the inverted L piece */
/* This piece position is fixed on the upper right corner on the Default Orientation */
/*
Default Orientation

* * x
    *

 */

pub struct BlueRicky {}

impl PieceType for BlueRicky {
    /*
    PUBLIC
     */

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_default_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles> {
        let r = Some([*position,
            Position{column:position.column-1,..*position},
            Position{column:position.column-2,..*position},
            Position{row:position.row-1,..*position},
        ]);
        if board[position.row - 2][position.column] == TileState::Taken {
            return r;
        }
        for i in (position.column - 2)..position.column {
            if board[position.row - 1][i] == TileState::Taken {
                return r;
            }
        }
        None
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_inverted_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles> {
        for i in position.column..(position.column+3) {
            if board[position.row - 1][i] == TileState::Taken {
                return Some([*position,
                    Position{column:position.column+1,..*position},
                    Position{column:position.column+2,..*position},
                    Position{row:position.row+1,..*position},
                ]);
            }
        }
        None
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_right_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles> {
        for i in (position.column-1)..(position.column + 1) {
            if board[position.row - 1][i] == TileState::Taken {
                return Some([*position,
                    Position{column:position.column-1,..*position},
                    Position{row:position.row+1,..*position},
                    Position{row:position.row+2,..*position},
                ]);
            }
        }
        None
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_left_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles> {
        if (board[position.row - 1][position.column + 1] == TileState::Taken) ||
            (board[position.row - 3][position.column] == TileState::Taken) {

            Some([*position,
                Position{column:position.column+1,..*position},
                Position{row:position.row-1,..*position},
                Position{row:position.row-2,..*position},
            ])
        } else {
            None
        }
    }
}

