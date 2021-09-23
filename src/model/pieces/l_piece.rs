use crate::model::pieces::piece::{PieceType, Position, PieceTiles};

/* This is the L piece */
/* This piece position is fixed on the upper center (Default Orientation) */
/*
Default Orientation

* x *
*

 */

pub struct LPiece {}

impl PieceType for LPiece {
    /*
    PRIVATE
    */

    fn get_default_positions(&self, position: &Position) -> PieceTiles {
        [   *position,
            Position{column:position.column+1,..*position},
            Position{column:position.column-1,..*position},
            Position{column:position.column-1, row:position.row-1},
        ]
    }

    fn get_inverted_positions(&self, position: &Position) -> PieceTiles {
        [   *position,
            Position{column:position.column-1,..*position},
            Position{column:position.column+1,..*position},
            Position{row:position.row+1,column:position.column+1},
        ]
    }

    fn get_right_positions(&self, position: &Position) -> PieceTiles {
        [   *position,
            Position{column:position.column-1,row:position.row+1},
            Position{row:position.row-1,..*position},
            Position{row:position.row+1,..*position},
        ]
    }

    fn get_left_positions(&self, position: &Position) -> PieceTiles {
        [   *position,
            Position{row:position.row+1,..*position},
            Position{row:position.row-1,..*position},
            Position{column:position.column+1,row:position.row-1},
        ]
    }
}
