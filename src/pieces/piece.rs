#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub trait Piece {
    fn get_position(&self) -> Position;
}
