use crate::model::pieces::piece::{PieceType, Position, PieceTiles};

/* This is the I piece */
/* This piece position is fixed on the center right (Default Orientation) */
/*
Default Orientation

* * x *

 */

pub struct Hero {}

impl PieceType for Hero {
    fn get_default_positions(&self, position: &Position) -> PieceTiles {
        [   *position,
            Position{column:position.column+1,..*position},
            Position{column:position.column-1,..*position},
            Position{column:position.column-2,..*position},
        ]
    }

    fn get_inverted_positions(&self, position: &Position) -> PieceTiles {
        self.get_default_positions(position)
    }

    fn get_right_positions(&self, position: &Position) -> PieceTiles {
        [   *position,
            Position{row:position.row-1,..*position},
            Position{row:position.row+1,..*position},
            Position{row:position.row+2,..*position},
        ]
    }

    /* Classic Tetris left rotation is the same as the right rotation */
    fn get_left_positions(&self, position: &Position) -> PieceTiles {
        self.get_right_positions(position)
    }
}
