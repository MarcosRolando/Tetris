use crate::game::Board;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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
    rotation: Rotation,
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
            rotation: Rotation::Default,
            collision_checkers: HashMap::new(),
        };
        piece.collision_checkers.insert(Rotation::Default, PieceType::check_default_collision);
        piece.collision_checkers.insert(Rotation::Inverted, PieceType::check_inverted_collision);
        piece.collision_checkers.insert(Rotation::Right, PieceType::check_right_collision);
        piece.collision_checkers.insert(Rotation::Left, PieceType::check_left_collision);
        /*Rust tiene una forma medio sidosa con collector y clone y eso de declarar de una el HashMap pero
         no funca con mi puntero a funcion porque tengo que implementarle el FromIterator y no
         se como es y paja. todo Ver como es e implementarlo!*/
        piece
    }

    /* Checks if the current piece has collided with the board and if so updates the taken tiles */
    pub fn check_collision(&self, board: &Board) -> Option<[Position;4]> {
        match self.collision_checkers.get(&self.rotation) {
            Some(f) => f(&self.piece_type, board, &self.position),
            None => panic!("Tried to invoke a CollisionChecker of an unknown rotation!"), //This case should never happen
        }
    }
}
