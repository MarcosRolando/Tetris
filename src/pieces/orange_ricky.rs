use crate::pieces::piece::Piece;

/* This is the inversed L piece */

pub struct OrangeRicky {
    pub x: i32,
}

impl Piece for OrangeRicky {
    fn print(&self) {
        println!("Hello, I am Orange Ricky!");
    }
}

