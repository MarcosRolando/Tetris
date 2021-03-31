use std::collections::HashMap;
use crate::board::{BOARD_WIDTH, BOARD_BASE};

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Orientation {
    Default, /*No Orientation, default piece orientation*/
    Inverted, /*180 degree rotation*/
    Right, /*-90 degree rotation*/
    Left, /*90 degree rotation*/
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

pub type PieceTiles = [Position; 4];

/* Anyone who implements PieceType must implement Clone, this is because it is requiered to allow
the HashMap initialization in game.rs
 */
pub trait PieceType {
    /*Returns the tiles the piece occupies in the board */
    fn get_positions(&self, orientation: Orientation, position: &Position) -> PieceTiles {
        match orientation {
            Orientation::Default => self.get_default_positions(position),
            Orientation::Inverted => self.get_inverted_positions(position),
            Orientation::Right => self.get_right_positions(position),
            Orientation::Left => self.get_left_positions(position),
        }
    }

    fn get_default_positions(&self, position: &Position) -> PieceTiles;
    fn get_inverted_positions(&self, position: &Position) -> PieceTiles;
    fn get_right_positions(&self, position: &Position) -> PieceTiles;
    fn get_left_positions(&self, position: &Position) -> PieceTiles;
}

pub struct Piece<T: PieceType + ?Sized> {
    position: Position,
    piece_type: Box<T>,
    orientation: Orientation,
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
            descent_time: (28 / 60) as f32, //It takes 28 frames to move and the game runs at 60 fps
            elapsed_time: 0 as f32,
        };
        /*Rust tiene una forma medio sidosa con collector y clone y eso de declarar de una el HashMap pero
         no funca con mi puntero a funcion porque tengo que implementarle el FromIterator y no
         se como es y paja. todo Ver como es e implementarlo!*/
        piece
    }

    /* Rotates the piece based on the Rotation given */
    pub fn rotate(&mut self, rotation: Rotation) {
        self.orientation = self.orientation.change(rotation);
    }

    /* Returns the tiles in the board that the piece is ocupying */
    pub fn get_positions(&self) -> PieceTiles {
        self.piece_type.get_positions(self.orientation, &self.position)
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
    pub fn move_to(&mut self, movement: Movement) { //todo cambiar esto, en realidad depende de la pieza y de la orientacion
        let positions = self.piece_type.get_positions(self.orientation, &self.position);
        match movement {
            Movement::Right => {
                for p in &positions {
                    if p.column >= (BOARD_WIDTH - 1) {
                        return;
                    }
                }
                self.position.column += 1
            },
            Movement::Left => {
                for p in &positions {
                    if p.column <= 0 {
                        return;
                    }
                }
                self.position.column -= 1
            },
            Movement::Down => {
                for p in &positions {
                    if p.row < BOARD_BASE {
                        return;
                    }
                }
                self.position.row -= 1
            },
        }
    }
}
