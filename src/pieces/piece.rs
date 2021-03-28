use std::borrow::Borrow;

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub trait PieceType {
    fn print(&self);
}

pub struct Piece {
    position: Position,
    piece_type: Option<Box<dyn PieceType>>,
}

impl Piece {
    pub fn new_default() -> Self {
        Piece {position: Position{x:0, y:0}, piece_type: None}
    }

    pub fn new(position: Position, piece_type: Box<dyn PieceType>) -> Self {
        Piece {position, piece_type: Some(piece_type)}
    }

    pub fn print(&self) {
        self.piece_type.borrow().as_ref().unwrap().print();
    }
}
