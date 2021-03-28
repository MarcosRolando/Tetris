use crate::pieces::piece;
use piece::Piece;
use piece::Position;

/* This is the I piece */

pub struct Hero {
    pub pos: Position,
}

impl Piece for Hero {
    fn get_position(&self) -> Position {
        self.pos
    }
}

