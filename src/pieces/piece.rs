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

#[derive(Copy, Clone, PartialEq)]
pub enum Movement {
    Right,
    Left,
    Down,
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
    descent_time: f32,
    elapsed_time: f32,
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
            descent_time: (28 / 60) as f32, //It takes 28 frames to move and the game runs at 60 fps
            elapsed_time: 0 as f32,
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

    /* Changes the time it takes for the piece to descend */
    pub fn change_descent_time(&mut self, descent_time: f32) {
        self.descent_time = descent_time;
    }

    /* If the time elpased is greater than the time it takes it to descend then descend, otherwise
    just update the time elapsed */
    pub fn try_to_descend(&mut self, delta_t: f32) {
        self.elapsed_time += delta_t;
        if self.elapsed_time >= self.descent_time {
            self.elapsed_time -= self.descent_time;
            self.move_to(Movement::Down);
        }
    }

    /*Updates the piece position */
    pub fn move_to(&mut self, movement: Movement) {
        match movement {
            Movement::Right => self.position.column += 1,
            Movement::Left => self.position.column -= 1,
            Movement::Down => self.position.row -= 1,
        }
    }
}
