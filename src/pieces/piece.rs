use crate::game::Board;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Rotation {
    Default, /*No rotation, default piece orientation*/
    Inverted, /*180 degree rotation*/
    Right, /*90 degree rotation*/
    Left, /*-90 degree rotation*/
}

pub trait PieceType {
    /* The following functions return an array of 4 elements of Positions indicating which
    board tiles have been taken if it collided, otherwise they return None */
    fn check_default_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]>;
    fn check_inverted_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]>;
    fn check_right_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]>;
    fn check_left_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]>;
}

pub struct Piece<T: PieceType + ?Sized> {
    position: Position,
    piece_type: Box<T>,
    orientation: Rotation,
    collision_checkers: HashMap<Rotation, fn (&T, &Board, &Position) -> Option<[Position; 4]>>,
}

impl<T: PieceType + ?Sized> Piece<T> {
    /*
    PUBLIC
     */

    pub fn new(position: Position, piece_type: Box<T>) -> Self {
        let mut piece = Piece {
            position,
            piece_type,
            orientation: Rotation::Default,
            collision_checkers: HashMap::new(),
        };
        piece.collision_checkers.insert(Rotation::Default, PieceType::check_default_collision);
        piece.collision_checkers.insert(Rotation::Inverted, PieceType::check_inverted_collision);
        piece.collision_checkers.insert(Rotation::Right, PieceType::check_right_collision);
        piece.collision_checkers.insert(Rotation::Left, PieceType::check_left_collision);
        piece
    }

    pub fn check_collision(&self, board: &Board) {
        match self.collision_checkers.get(&self.orientation) {
            Some(f) => drop(f(&self.piece_type, board, &self.position)),
            None => (),
        }
    }
}
