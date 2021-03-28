use crate::game::Board;
use crate::pieces::piece::Orientation::Straight;
use std::collections::HashMap;

type CheckCollision = fn(board: &Board, position: &Position) -> [Position; 4];

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

pub struct Piece {
    position: Position,
    piece_type: Option<Box<dyn PieceType>>,
    orientation: Orientation,
    collision_checkers: HashMap<Orientation, CheckCollision>,
}

impl Piece {
    pub fn new(position: Position, piece_type: Box<dyn PieceType>) -> Self {
        let mut piece = Piece {
            position,
            piece_type: Some(piece_type),
            orientation: Straight,
            collision_checkers: HashMap::new(),
        };
        piece.collision_checkers.insert(Orientation::Straight,
                                        PieceType::check_straight_collision);
        piece
    }

    pub fn check_collision(&self, board: &Board) {
        match &self.piece_type {
            Some(piece_type) => piece_type.check_collision(board, &self.position),
            None => eprintln!("Tried to check collision with no piece!"),
        }
    }
}
