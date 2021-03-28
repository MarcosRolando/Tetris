use crate::pieces::piece;
use piece::PieceType;

/* This is the inversed L piece */

pub struct OrangeRicky {}

impl PieceType for OrangeRicky {
    fn print(&self) {
        println!("I am Orange Ricky!");
    }
}

