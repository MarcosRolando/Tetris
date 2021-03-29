use crate::pieces::piece::{PieceType, Position, TakenTiles};
use crate::game::{Board, TileState};

/* This is the inverted Z piece */
/* This piece position is fixed on the upper center corner (Default Orientation) */
/*
Default Orientation

  x *
* *

 */

pub struct RhodeIslandZ {}

impl PieceType for RhodeIslandZ {
    /*
    PUBLIC
     */

    fn new() -> Box<dyn PieceType> where Self: Sized {
        Box::new(RhodeIslandZ {})
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_default_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles> {
        let r = Some([*position,
            Position{column:position.column-1,..*position},
            Position{row:position.row+1,..*position},
            Position{row:position.row+1,column:position.column+1},
        ]);
        if board[position.row][position.column + 1] == TileState::Taken {
            return r;
        }
        for i in (position.column - 1)..(position.column + 1) {
            if board[position.row - 1][i] == TileState::Taken {
                return r;
            }
        }
        None
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    //noinspection DuplicatedCode CLion complains about one fucking duplicated line between different files
    fn check_inverted_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles> {
        self.check_default_collision(board, position)
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    /* In Classic Tetris this rotation is actually the same as the left rotation */
    fn check_right_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles> {
        self.check_left_collision(board, position)
    }

    /* Returns an array of 4 elements of Positions if it collided, otherwise returns None */
    fn check_left_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles> {
        if (board[position.row][position.column] == TileState::Taken) ||
            (board[position.row - 1][position.column + 1] == TileState::Taken) {

            Some([Position{row:position.row+1,..*position},
                Position{row:position.row+2,..*position},
                Position{column:position.column+1,..*position},
                Position{row:position.row+1,column:position.column+1},
            ])
        } else {
            None
        }
    }
}


