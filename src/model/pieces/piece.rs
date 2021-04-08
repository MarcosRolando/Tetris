
/*The only reason this is generic and not isize is because I need to cast it to and isize based
type to be able to access the board array, and casting Position to Position<usize> is
the most natural and convenient way
 */
use crate::game_engine::{Input_t, INPUT_HOLD_DOWN, INPUT_HOLD_LEFT, INPUT_HOLD_RIGHT,
                         INPUT_RELEASE_DOWN, INPUT_RELEASE_RIGHT, INPUT_RELEASE_LEFT, INPUT_ROTATE_RIGHT,
                        INPUT_ROTATE_LEFT};

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

/* I originally separated rotation and cardinal movement, but in the end grouping them together
    allowed a nicer code so I went that route
 */
#[derive(Copy, Clone, PartialEq)]
pub enum Movement {
    Right,
    Left,
    Down,
    Up,
    RightRotation,
    LeftRotation,
}

impl Movement {
    /* Returns the opposite movement */
    pub fn get_opposite(m: Self) -> Self {
        match m {
            Self::Right => Self::Left,
            Self::Left => Self::Right,
            Self::Down => Self::Up,
            Self::Up => Self::Down, //Though this case should never occur under normal Tetris rules!
            Self::RightRotation => Self::LeftRotation,
            Self::LeftRotation => Self::RightRotation,
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
    pub fn change(&self, rotation: Movement) -> Self {
        match rotation {
            Movement::RightRotation => self._change_right(),
            Movement::LeftRotation => self._change_left(),
            _ => Self::Default, //The other movements are not rotations, so this case should never happen either way
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
pub struct Piece{
    position: Position,
    piece_type: Box<dyn PieceType>,
    orientation: Orientation,
    autorepeat_x: i32, //a counter for dealing with the horizontal movement of the piece
    autorepeat_y: i32, //a counter for dealing with the vertical drop of the piece
    drop_speed: u32, //the amount of frames it takes to descend one tile
    fall_timer: u32, //the current amount of frames passed
    holding: Movement, //stores the current key the player is holding down, we use the Up movement to represent no holding
}

impl Piece {
    /*
    PUBLIC
     */

    pub fn new(position: Position, piece_type: Box<dyn PieceType>, is_first_piece: bool) -> Self {
        let mut piece = Piece {
            position,
            piece_type,
            orientation: Orientation::Default,
            autorepeat_x: 0,
            autorepeat_y: 0,
            drop_speed: 48, //The drop speed in frames (see https://tetris.wiki/Tetris_(NES,_Nintendo))
            fall_timer: 0,
            holding: Movement::Up, //Up would be the no holding value since it is not a valid player movement anyway
        };
        if is_first_piece {
            piece.autorepeat_y = -96 //We will be increasing this value by one per update so that it takes 1.6 seconds to start falling after starting the game (see https://tetris.wiki/Tetris_(NES,_Nintendo))
        }
        piece
    }

    /* Returns the center position of the piece */
    pub fn get_center_position(&self) -> Position {
        self.position
    }

    /* Returns the tiles in the board that the piece is ocupying */
    pub fn get_positions(&self) -> PieceTiles {
        self.piece_type.get_positions(self.orientation, &self.position)
    }

    /* Changes the time it takes for the piece to descend */ //todo cambiar a frames en vez de segundos
    pub fn change_descent_time(&mut self, descent_time: f32) {
        //self.descent_time = descent_time;
    }

    /* Receives the player input and updates the behaviour of the piece as necessary. Returns Some
        Movement if the conditions to move are met, otherwise it returns None
     */
    pub fn process_input(&mut self, input: Input_t) -> Option<Movement> {
        match self._process_pressed_input(input) {
            Ok(result) => return result,
            _ => (),
        }
        if self._process_release_input(input) {
            return None;
        }
        return self._process_no_input(); //If we arrived here it means we received INPUT_NONE
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

    /* Moves the piece according to the given movement. While move would have been a better method
        name it is unfortunately a reserved keywoard */
    pub fn move_to(&mut self, movement: Movement) {
        match movement {
            Movement::Right => self.position.column += 1,
            Movement::Left => self.position.column -= 1,
            Movement::Down => self.position.row -= 1,
            Movement::Up => self.position.row += 1, //Though this case should never occur under normal Tetris rules, we just use it to correct an invalid one!
            Movement::RightRotation | Movement::LeftRotation => {
                self.orientation = self.orientation.change(movement);
            },
        }
    }

    /*
    PRIVATE
     */

    fn _process_no_input(&mut self) -> Option<Movement> {
        match self.holding {
            Movement::Down => {
                self.autorepeat_y += 1;
                if self.autorepeat_y == 3 { //This the same check the Classic NES Tetris uses (see https://tetris.wiki/Tetris_(NES,_Nintendo))
                    self.autorepeat_y = 1;
                    return Some(Movement::Down);
                }
            },
            Movement::Right | Movement::Left => {
                self.autorepeat_x += 1;
                if self.autorepeat_x == 16 { //16 is the number of frames Classic NES Tetris uses for initial horizontal DAS (see https://tetris.wiki/Tetris_(NES,_Nintendo))
                    self.autorepeat_x = 10;
                    return Some(self.holding);
                }
            },
            _ => (), //We don't care about the Movement::Up case because it is not valid anyway
        }
        return None;
    }

    /* Returns Ok if it handled the input, and Some movement if there is a movement to be made.
        If there is no moevement to be made it retuns None. If it didn't handle the input it returns void
     */
    fn _process_pressed_input(&mut self, input: Input_t) -> Result<Option<Movement>, ()> {
        if input == INPUT_HOLD_DOWN { //Unfortunately I can't use match here because it actually matches the data type and not the value of the data
            if self.autorepeat_y < 0 {
                self.autorepeat_y = 0;
            }
            self.holding = Movement::Down;
            Ok(None)
        } else if input == INPUT_HOLD_RIGHT {
            self.holding = Movement::Right;
            Ok(Some(Movement::Right))
        }  else if input == INPUT_HOLD_LEFT {
            self.holding = Movement::Left;
            Ok(Some(Movement::Left))
        } else if input == INPUT_ROTATE_RIGHT {
            Ok(Some(Movement::RightRotation))
        } else if input == INPUT_ROTATE_LEFT {
            Ok(Some(Movement::LeftRotation))
        } else {
            Err(())
        }
    }

    /* Returns true if it handled the input, false otherwise */
    fn _process_release_input(&mut self, input: Input_t) -> bool {
        if input == INPUT_RELEASE_DOWN { //Unfortunately I can't use match here because it actually matches the data type and not the value of the data
            self.holding = Movement::Up;
            self.autorepeat_y = 0;
            true
        } else if input == INPUT_RELEASE_RIGHT || input == INPUT_RELEASE_LEFT {
            self.holding = Movement::Up;
            self.autorepeat_x = 0;
            true
        } else {
            false
        }
    }
}
