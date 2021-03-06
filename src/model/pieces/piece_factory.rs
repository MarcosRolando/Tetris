use super::{
    j_piece::JPiece,
    l_piece::LPiece,
    i_piece::IPiece,
    t_piece::TPiece,
    o_piece::OPiece,
    s_piece::SPiece,
    z_piece::ZPiece,
};
use rand::{distributions::{Distribution, Standard}, Rng, random};
use crate::model::pieces::piece::{Piece, Position};

#[derive(Clone, Copy, Hash)]
enum PieceTypeID {
    I,
    O,
    T,
    L,
    J,
    Z,
    S,
}

/* Returns a random PieceTypeID value, for selecting the next piece */
impl Distribution<PieceTypeID> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceTypeID {
        match rng.gen_range(0..=56) { //The following adhere to the Tetrominoes weights in the classic NES Tetris
                                            // See https://meatfighter.com/nintendotetrisai/?a=b#The_Mechanics_of_Nintendo_Tetris
            0..=6 => PieceTypeID::I,
            7..=14 => PieceTypeID::O,
            15..=23 => PieceTypeID::T,
            24..=30 => PieceTypeID::L,
            31..=38 => PieceTypeID::J,
            39..=46 => PieceTypeID::Z,
            _ => PieceTypeID::S,
        }
    }
}

pub struct PieceFactory {}

impl PieceFactory {
    /*
    PUBLIC
     */

    /* Returns a random new piece */
    pub fn new_piece(starting_position: Position, is_first_piece: bool) -> Piece {
        let piece_type: PieceTypeID = random();
        match piece_type {
            PieceTypeID::I => Piece::new(starting_position, Box::new(IPiece {}), is_first_piece),
            PieceTypeID::L => Piece::new(starting_position, Box::new(LPiece {}), is_first_piece),
            PieceTypeID::J => Piece::new(starting_position, Box::new(JPiece {}), is_first_piece),
            PieceTypeID::T => Piece::new(starting_position, Box::new(TPiece {}), is_first_piece),
            PieceTypeID::Z => Piece::new(starting_position, Box::new(ZPiece {}), is_first_piece),
            PieceTypeID::S => Piece::new(starting_position, Box::new(SPiece {}), is_first_piece),
            PieceTypeID::O => Piece::new(starting_position, Box::new(OPiece {}), is_first_piece),
        }
    }
}