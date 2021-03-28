use crate::game::Board;
use crate::pieces::piece::Orientation::Straight;
use std::collections::HashMap;
use crate::pieces::hero::Hero;

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Orientation {
    Straight, /*Default orientation*/
    Inverted, /*180 degree rotation*/
    Right, /*-90 degree rotation*/
    Left, /*90 degree rotation*/
}

pub trait PieceType {
    fn check_straight_collision(&self, board: &Board, position: &Position) -> [Position; 4];
    fn check_inverted_collision(&self, board: &Board, position: &Position) -> [Position; 4];
    fn check_right_collision(&self, board: &Board, position: &Position) -> [Position; 4];
    fn check_left_collision(&self, board: &Board, position: &Position) -> [Position; 4];
}

pub struct Piece<T: PieceType + ?Sized> {
    position: Position,
    piece_type: Box<T>,
    orientation: Orientation,
    collision_checkers: HashMap<Orientation, fn (&T, &Board, &Position) -> [Position; 4]>,
}

impl<T: PieceType + ?Sized> Piece<T> {
    pub fn new(position: Position, piece_type: Box<T>) -> Self {
        let mut piece = Piece {
            position,
            piece_type,
            orientation: Straight,
            collision_checkers: HashMap::new(),
        };
        piece.collision_checkers.insert(Orientation::Straight, PieceType::check_straight_collision);
        piece.collision_checkers.insert(Orientation::Inverted, PieceType::check_inverted_collision);
        piece.collision_checkers.insert(Orientation::Right, PieceType::check_right_collision);
        piece.collision_checkers.insert(Orientation::Left, PieceType::check_left_collision);
        piece
    }

    pub fn check_collision(&self, board: &Board) {
        match self.collision_checkers.get(&self.orientation) {
            Some(f) => drop(f(&self.piece_type, board, &self.position)),
            None => (),
        }
    }
}
