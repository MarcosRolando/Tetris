use crate::pieces::piece::Piece;

/* This is the I piece */

pub struct Hero {
    pub x: i32,
}

impl Piece for Hero {
    fn print(&self) {
        println!("Hello, I am Hero!");
    }
}

