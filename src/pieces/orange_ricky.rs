use crate::pieces::piece;
use piece::Piece;
use piece::Position;

/* This is the inversed L piece */

pub struct OrangeRicky {
    pub pos: Position,
}

impl Piece for OrangeRicky {
    fn get_position(&self) -> Position {
        self.pos
    }
}

