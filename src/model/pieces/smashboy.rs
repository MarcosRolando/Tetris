use crate::model::pieces::piece::{PieceType, Position, PieceTiles};

/* This is the square piece */
/* This piece position is fixed on the upper right corner */
/* This is the only piece that does not actually rotate */
/*
Default Orientation

* X
* *

 */

pub struct Smashboy {}

impl PieceType for Smashboy {
    fn get_default_positions(&self, position: &Position) -> PieceTiles {
        [   *position,
            Position{row:position.row-1,..*position},
            Position{column:position.column-1,..*position},
            Position{row:position.row-1,column:position.column-1},
        ]
    }

    fn get_inverted_positions(&self, position: &Position) -> PieceTiles {
        self.get_default_positions(position)
    }

    fn get_right_positions(&self, position: &Position) -> PieceTiles {
        self.get_default_positions(position)
    }

    fn get_left_positions(&self, position: &Position) -> PieceTiles {
        self.get_default_positions(position)
    }
}
