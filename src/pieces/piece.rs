use crate::game::Board;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Rotation {
    Default, /*No rotation, default piece orientation*/
    Inverted, /*180 degree rotation*/
    Right, /*90 degree rotation*/
    Left, /*-90 degree rotation*/
}

impl Rotation {
    /*
    PUBLIC
     */

    /* Given the current Rotation value and the Right or Left rotation transformation applied it
    returns the next Rotation value
     */
    pub fn change(&self, direction: Self) -> Self {
        match direction {
            Self::Right => self._change_right(),
            Self::Left => self._change_left(),
            _ => panic!("Tried to change the rotation without a Right or Left value!"), //This case should never happen
        }
    }

    /*
    PRIVATE
     */

    fn _change_right(&self) -> Self {
        match self {
            Self::Default => Self::Right,
            Self::Right => Self::Inverted,
            Self::Inverted => Self::Left,
            Self::Left => Self::Default,
        }
    }

    fn _change_left(&self) -> Self {
        match self {
            Self::Default => Self::Left,
            Self::Left => Self::Inverted,
            Self::Inverted => Self::Right,
            Self::Right => Self::Default,
        }
    }
}

pub trait PieceType {
    /* The following functions return an array of 4 elements of Positions indicating which
    board tiles have been taken if it collided, otherwise they return None */
    fn check_default_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]>;
    fn check_inverted_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]>;
    fn check_right_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]>;
    fn check_left_collision(&self, board: &Board, position: &Position) -> Option<[Position; 4]>;

    /* Changes the piece position that corresponds to the given rotation */
    fn rotate(&self, rotation: Rotation);
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

    pub fn rotate(&mut self, direction: Rotation) {
        self.rotation = self.rotation.change(direction);
        self.piece_type.rotate(self.rotation);
    }

    /* Checks if the current piece has collided with the board and if so updates the taken tiles */
    pub fn check_collision(&self, board: &Board) -> Option<[Position;4]> {
        match self.collision_checkers.get(&self.rotation) {
            Some(f) => f(&self.piece_type, board, &self.position),
            None => panic!("Tried to invoke a CollisionChecker of an unknown rotation!"), //This case should never happen
        }
    }
}
