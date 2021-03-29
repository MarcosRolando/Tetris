use crate::pieces::piece::{PieceType, Position, TakenTiles};
use crate::game::{Board, TileState};

/* This is the I piece */
/* This piece position is fixed on the center right (Default Orientation) */
/*
Default Orientation

* * x *

 */

pub struct Hero {}

impl Hero {}

impl PieceType for Hero {
    /*
    PUBLIC
     */

    fn new() -> Box<dyn PieceType> {
        Box::new(Hero {})
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_default_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles> {
        for i in (position.column - 2)..(position.column + 2) {
            if board[position.row][i] == TileState::Taken {
                return Some([Position{row:position.row+1,..*position},
                             Position{column:position.column+1,row:position.row+1},
                             Position{column:position.column-1,row:position.row+1},
                             Position{column:position.column-2,row:position.row+1},
               ]);
            }
        }
        None
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_inverted_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles> {
        self.check_default_collision(board, position)
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_right_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles> {
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

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    /* In Classic Tetris this rotation is actually the same as the right rotation */
    fn check_left_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles> {
        self.check_right_collision(board, position)
    }
}
