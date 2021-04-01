use crate::board::{BOARD_WIDTH, BOARD_BASE};

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub row: isize,
    pub column: isize,
}

impl From<Position> for (usize, usize) {
    fn from(position: Position) -> Self {
        (position.row as usize, position.column as usize)
    }
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

impl Rotation {
    /* Returns the opposite rotation */
    pub fn get_opposite(r: Self) -> Self {
        match r {
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Movement {
    Right,
    Left,
    Down,
    Up,
}

impl Movement {
    /* Returns the opposite movement */
    pub fn get_opposite(m: Self) -> Self {
        match m {
            Self::Right => Self::Left,
            Self::Left => Self::Right,
            Self::Down => Self::Up,
            Self::Up => Self::Down, //Though this case should never occur under normal Tetris rules!
        }
    }
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
        let piece = Piece {
            position,
            piece_type,
            orientation: Orientation::Default,
            descent_time: 28.0 / 60.0, //It takes 28 frames to move and the game runs at 60 fps
            elapsed_time: 0.0,
        };
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

    /* Moves the piece according to the given movement */
    pub fn move_to(&mut self, movement: Movement) {
        match movement {
            Movement::Right => self.position.column += 1,
            Movement::Left => self.position.column -= 1,
            Movement::Down => self.position.row -= 1,
            Movement::Up => self.position.row += 1, //Though this case should never occur under normal Tetris rules!
        }
    }
}
