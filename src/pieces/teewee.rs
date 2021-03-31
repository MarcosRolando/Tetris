use crate::pieces::piece::{PieceType, Position, PieceTiles};

/* This is the T piece */
/* This piece position is fixed on the upper center on the Default Orientation */
/*
Default Orientation

* x *
  *

 */

pub struct Teewee {}

impl PieceType for Teewee {
    fn get_default_positions(&self, position: &Position) -> PieceTiles {
        [   *position,
            Position{column:position.column-1,..*position},
            Position{column:position.column+1,..*position},
            Position{row:position.row-1,..*position},
        ]
    }

    fn get_inverted_positions(&self, position: &Position) -> PieceTiles {
        [   *position,
            Position{column:position.column-1,..*position},
            Position{column:position.column+1,..*position},
            Position{row:position.row+1,..*position},
        ]
    }

    fn get_right_positions(&self, position: &Position) -> PieceTiles {
        [   *position,
            Position{column:position.column-1,..*position},
            Position{row:position.row+1,..*position},
            Position{row:position.row-1,..*position},
        ]
    }

    fn get_left_positions(&self, position: &Position) -> PieceTiles {
        [   *position,
            Position{column:position.column+1,..*position},
            Position{row:position.row+1,..*position},
            Position{row:position.row-1,..*position},
        ]
    }
}
