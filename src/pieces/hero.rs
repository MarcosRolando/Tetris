use crate::pieces::piece;
use piece::Piece;

/* This is the I piece */

pub struct Hero {
    pub x: i32,
}

impl Piece for Hero {
    fn print() {
        println!("Hello, World!");
    }
}

