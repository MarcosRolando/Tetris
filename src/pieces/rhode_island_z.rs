use crate::pieces::piece::{PieceType, Position, PieceTiles};

/* This is the inverted Z piece */
/* This piece position is fixed on the upper center corner (Default Orientation) */
/*
Default Orientation

  x *
* *

 */

pub struct RhodeIslandZ {}

impl PieceType for RhodeIslandZ {
    fn get_default_positions(&self, position: &Position) -> PieceTiles {
        [   *position,
            Position{column:position.column-1,row:position.row-1},
            Position{row:position.row-1,..*position},
            Position{column:position.column+1,..*position},
        ]
    }

    fn get_inverted_positions(&self, position: &Position) -> PieceTiles {
        self.get_default_positions(position)
    }

    fn get_right_positions(&self, position: &Position) -> PieceTiles {
        self.get_left_positions(position)
    }

    fn get_left_positions(&self, position: &Position) -> PieceTiles {
        [   *position,
            Position{row:position.row+1,..*position},
            Position{column:position.column+1,row:position.row-1},
            Position{column:position.column+1,..*position},
        ]
    }
}
