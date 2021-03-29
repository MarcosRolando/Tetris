use crate::pieces::piece::{PieceType, Position, Orientation, Rotation, TakenTiles};
use crate::game::{Board, TileState};

/* This is the I piece */
/* This piece position is fixed on the left corner on the Default Orientation */
/*
Default Orientation

x * * *

 */

pub struct Hero {}

impl Hero {
    /*
    PRIVATE
     */

    fn _rotate_right(&self, position: &Position, orientation: Orientation) -> Position {
        match orientation {
            Orientation::Default => Position {row:position.row},
        }
    }

    fn _rotate_left(&self, orientation: Orientation) {

    }
}

impl PieceType for Hero {
    /*
    PUBLIC
     */

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_default_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles> {
        for i in position.column..(position.column + 4) {
            if board[position.row - 1][i] == TileState::Taken {
                return Some([*position,
                             Position{column:position.column+1,..*position},
                             Position{column:position.column+2,..*position},
                             Position{column:position.column+3,..*position},
               ]);
            }
        }
        None
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_inverted_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles> {
        self.check_default_collision(board, &Position{column:position.column-3,..*position})
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_right_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles> {
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
    fn check_left_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles> {
        self.check_right_collision(board, &Position{row:position.row+3,..*position})
    }

    /* Changes the piece position that corresponds to the given Orientation */
    fn change_position(&self, position: &Position, orientation: Orientation, rotation: Rotation)
                        -> Position {
        match rotation {
            Rotation::Right => self._rotate_right(position, orientation),
            Rotation::Left => self._rotate_left(position, orientation),
        }
    }
}
