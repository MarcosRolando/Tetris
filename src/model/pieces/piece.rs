
/*The only reason this is generic and not isize is because I need to cast it to and isize based
type to be able to access the board array, and casting Position to Position<usize> is
the most natural and convenient way
 */
#[derive(Copy, Clone, PartialEq)]
pub struct Position<T=isize> {
    pub row: T,
    pub column: T,
}

impl From<Position<isize>> for Position<usize> {
    fn from(position: Position) -> Self {
        Position {row:position.row as usize, column: position.column as usize}
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

/* In order to recreate the exact feeling of the classis NES Tetris I have to recreate the exact
    timings, which in the original code are given in frames assuming a framerate of 60 FPS.
    It is way easier to just cap the game at 60 fps and use everything in frames, both for coding and for
    understanding, so that is the route I took. This means that most of the source code you are reading
    here it's the exact same logic they used! Each frame is a game update.
 */
pub struct Piece<T: PieceType + ?Sized> {
    position: Position,
    piece_type: Box<T>,
    orientation: Orientation,
    autorepeat_x: i32, //a counter for dealing with the horizontal movement of the piece
    autorepeat_y: i32, //a counter for dealing with the vertical drop of the piece
    drop_speed: u32, //the amount of frames it takes to descend one tile
    fall_timer: u32, //the current amount of frames passed
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
            autorepeat_x: 0, //todo
            autorepeat_y: -96, //We will be increasing this value by one per update so that it takes 1.6 seconds to start falling after spawning (see https://tetris.wiki/Tetris_(NES,_Nintendo))
            drop_speed: 8, //The drop speed in frames (see https://tetris.wiki/Tetris_(NES,_Nintendo))
            fall_timer: 0,
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

    /* Changes the time it takes for the piece to descend */ //todo cambiar a frames en vez de segundos
    pub fn change_descent_time(&mut self, descent_time: f32) {
        //self.descent_time = descent_time;
    }

    /* If the frames elpased is greater than the frames it takes it to descend then descend, otherwise
    just update the frames elapsed */
    pub fn try_to_descend(&mut self) {
        if self.autorepeat_y < 0 {
            self.autorepeat_y += 1;
        } else {
            self.fall_timer += 1;
            if self.fall_timer >= self.drop_speed {
                self.fall_timer = 0;
                self.move_to(Movement::Down);
            }
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
