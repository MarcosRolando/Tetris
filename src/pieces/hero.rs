use crate::pieces::piece;
use piece::PieceType;

/* This is the I piece */

pub struct Hero {}

impl PieceType for Hero {
    fn print(&self) {
        println!("I am Hero!");
    }
}


