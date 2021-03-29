use crate::game::Board;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Orientation {
    Default, /*No Orientation, default piece orientation*/
    Inverted, /*180 degree rotation*/
    Right, /*90 degree rotation*/
    Left, /*-90 degree rotation*/
}

#[derive(Copy, Clone, PartialEq)]
pub enum Rotation {
    Right,
    Left,
}

impl Orientation {
    /*
    PUBLIC
     */

    /* Given the current Orientation value and the Right or Left Orientation transformation applied it
    returns the next Orientation value
     */
    pub fn change(&self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::Right => self._change_right(),
            Rotation::Left => self._change_left(),
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

pub type TakenTiles = [Position; 4];

pub trait PieceType {
    /* The following functions return an array of 4 elements of Positions indicating which
    board tiles have been taken if it collided, otherwise they return None */
    fn check_default_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles>;
    fn check_inverted_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles>;
    fn check_right_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles>;
    fn check_left_collision(&self, board: &Board, position: &Position) -> Option<TakenTiles>;
}

pub struct Piece<T: PieceType + ?Sized> {
    position: Position,
    piece_type: Box<T>,
    orientation: Orientation,
    collision_checkers: HashMap<Orientation, fn (&T, &Board, &Position) -> Option<TakenTiles>>,
}

impl<T: PieceType + ?Sized> Piece<T> {
    /*
    PUBLIC
     */

    pub fn new(position: Position, piece_type: Box<T>) -> Self {
        let mut piece = Piece {
            position,
            piece_type,
            orientation: Orientation::Default,
            collision_checkers: HashMap::new(),
        };
        piece.collision_checkers.insert(Orientation::Default, PieceType::check_default_collision);
        piece.collision_checkers.insert(Orientation::Inverted, PieceType::check_inverted_collision);
        piece.collision_checkers.insert(Orientation::Right, PieceType::check_right_collision);
        piece.collision_checkers.insert(Orientation::Left, PieceType::check_left_collision);
        /*Rust tiene una forma medio sidosa con collector y clone y eso de declarar de una el HashMap pero
         no funca con mi puntero a funcion porque tengo que implementarle el FromIterator y no
         se como es y paja. todo Ver como es e implementarlo!*/
        piece
    }

    /* Rotates the piece based on the Rotation given */
    pub fn rotate(&mut self, rotation: Rotation) {
        self.orientation = self.orientation.change(rotation);
    }

    /* Checks if the current piece has collided with the board and if so updates the taken tiles */
    pub fn check_collision(&self, board: &Board) -> Option<[Position;4]> {
        match self.collision_checkers.get(&self.orientation) {
            Some(f) => f(&self.piece_type, board, &self.position),
            None => panic!("Tried to invoke a CollisionChecker of an unknown Orientation!"), //This case should never happen
        }
    }
}
